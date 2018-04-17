use mem::{Memory, NESMemory};
use opcode::{Opcode, OPCODES};

const STACK_INIT: u8 = 0xfd;
const PPU_DOTS_PER_SCANLINE: usize = 341;

// A, X, and Y are 8-bit registers
type Register = u8;

// Status flags
type Flag = bool;

// 16-bit register
type ProgramCounter = u16;

// 8-bit register
type StackPointer = u8;

pub struct CPU {
    pub mem: NESMemory,

    // Main registers
    pub a: Register,  // Accumulator
    pub x: Register,  // X Index
    pub y: Register,  // Y Index

    // Status register flags
    pub c: Flag,  // Carry
    pub z: Flag,  // Zero
    pub i: Flag,  // Interrupt
    pub d: Flag,  // Decimal mode
    pub b: Flag,  // Software interrupt (BRK)
    pub u: Flag,  // Unused flag
    pub v: Flag,  // Overflow
    pub s: Flag,  // Sign

    // Program counter
    pub pc: ProgramCounter,

    // Stack pointer
    pub sp: StackPointer,

    cycles: usize,
}

impl CPU {
    pub fn new_nes_cpu(mem: NESMemory) -> CPU {
        CPU {
            mem: mem,

            a: 0,
            x: 0,
            y: 0,

            c: false,
            z: false,
            i: false,
            d: false,
            b: false,
            u: false,
            v: false,
            s: false,

            pc: 0x0000,

            sp: STACK_INIT,

            cycles: 0,
        }
    }

    pub fn init(&mut self) {
        let lo = self.mem.read(0xFFFC).expect("low PC byte") as u16;
        let hi = self.mem.read(0xFFFD).expect("high PC byte") as u16;
        let addr = (hi << 8) | lo;
        self.pc = addr;
        self.pc = 0xc000;
        println!("starting program counter: 0x{:04X}", self.pc);

        self.set_flags(0x24);
        println!("initial flags: 0x{:02X}", self.flags());
    }

    pub fn flags(&self) -> u8 {
           (self.c as u8)
        | ((self.z as u8) << 1)
        | ((self.i as u8) << 2)
        | ((self.d as u8) << 3)
        | ((self.b as u8) << 4)
        | ((self.u as u8) << 5)
        | ((self.v as u8) << 6)
        | ((self.s as u8) << 7)
    }

    pub fn set_flags(&mut self, val: u8) {
        self.c = val & 0x01 == 1;
        self.z = (val >> 1 & 0x01) == 1;
        self.i = (val >> 2 & 0x01) == 1;
        self.d = (val >> 3 & 0x01) == 1;
        self.b = (val >> 4 & 0x01) == 1;
        self.u = (val >> 5 & 0x01) == 1;
        self.v = (val >> 6 & 0x01) == 1;
        self.s = (val >> 7 & 0x01) == 1;
    }

    fn debug(&self, op: &Opcode) {
        let Opcode(ref inst, ref addr_mode, _, _) = *op;

        if let Err(_) = addr_mode.n_bytes() {
            let opcode = self.mem.read(self.pc).unwrap();
            panic!("unsupported addressing mode {:?} at PC {:04X}, opcode {:02X}",
                   addr_mode,
                   self.pc,
                   opcode);
        }

        let bytes = addr_mode.get_bytes(self)
            .iter()
            .map(|arg| String::from(format!("{:02X}", arg)))
            .collect::<Vec<_>>()
            .join(" ");

        let ppu_dots = self.cycles * 3 % PPU_DOTS_PER_SCANLINE;

        println!("{:04X}  {:8}  {:32?} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{:-3}",
                 self.pc,
                 bytes,
                 inst,
                 self.a,
                 self.x,
                 self.y,
                 self.flags(),
                 self.sp,
                 ppu_dots);
    }

    pub fn stack_push8(&mut self, val: u8) {
        if self.sp == 0 {
            panic!("cannot push onto a full stack");
        }

        // The stack page exists from 0x0100 to 0x01FF
        let addr = (0x01 << 8) | self.sp as u16;
        self.mem.write(addr, val)
            .expect("unable to write to stack");
        self.sp -= 1;
    }

    pub fn stack_pop8(&mut self) -> u8 {
        if self.sp == STACK_INIT {
            panic!("cannot pop from an empty stack");
        }

        self.sp += 1;

        // The stack page exists from 0x0100 to 0x01FF
        let addr = (0x01 << 8) | self.sp as u16;
        let val = self.mem.read(addr)
            .expect("unable to read from stack");

        val
    }

    pub fn stack_push16(&mut self, val: u16) {
        let hi = (val >> 8) as u8;
        self.stack_push8(hi);

        let lo = (val & 0x00ff) as u8;
        self.stack_push8(lo);
    }

    pub fn stack_pop16(&mut self) -> u16 {
        let lo = self.stack_pop8() as u16;
        let hi = self.stack_pop8() as u16;
        (hi << 8) | lo
    }

    pub fn update_sz(&mut self, val: u8) {
        self.s = val & 0x80 != 0;
        self.z = val == 0;
    }

    pub fn add_branch_cycles(&mut self, pc: ProgramCounter, addr: u16) {
        self.cycles += 1;

        if (pc & 0xff00) != (addr & 0xff00) {
            self.cycles += 1;
        }
    }

    pub fn step(&mut self) {
        let opcode = self.mem.read(self.pc)
            .expect("unable to read next opcode");

        let op = &OPCODES[opcode as usize];
        self.debug(&op);

        let &Opcode(ref inst, ref addr_mode, ref cycles, ref extra_cycles) = op;

        if let Ok(bytes) = addr_mode.n_bytes() {
            self.pc += bytes as u16;
            self.cycles = (self.cycles + cycles) % PPU_DOTS_PER_SCANLINE;

            if let Ok((addr, val, page_crossed)) = addr_mode.get_data(self) {
                inst.run(self, addr, val, addr_mode);

                if page_crossed {
                    self.cycles += extra_cycles;
                }
            }
            else {
                panic!("unable to get data");
            }
        }
        else {
            let opcode = self.mem.read(self.pc).unwrap();
            panic!("unsupported addressing mode {:?} at PC {:04X}, opcode {:02X}",
                   addr_mode,
                   self.pc,
                   opcode);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_stack_pop_empty() {
        let mut cpu = CPU::new_nes_cpu();
        let _ = cpu.stack_pop8();
        assert!(false);
    }

    #[test]
    #[should_panic]
    fn test_stack_push_full() {
        let mut cpu = CPU::new_nes_cpu();
        for _ in 0 .. 255 {
            cpu.stack_push8(0xff);
        }
        assert!(false);
    }

    #[test]
    fn test_stack() {
        let mut cpu = CPU::new_nes_cpu();

        cpu.stack_push8(0xff);
        assert_eq!(cpu.sp, 0xfc);
        assert_eq!(cpu.mem.ram[0x0100 + (cpu.sp as usize) + 1], 0xff);

        cpu.stack_push16(0xdead);
        assert_eq!(cpu.sp, 0xfa);
        assert_eq!(cpu.mem.ram[0x100 + (cpu.sp as usize) + 1], 0xad);
        assert_eq!(cpu.mem.ram[0x100 + (cpu.sp as usize) + 2], 0xde);

        let rv = cpu.stack_pop16();
        assert_eq!(cpu.sp, 0xfc);
        assert_eq!(rv, 0xdead);

        let rv = cpu.stack_pop8();
        assert_eq!(cpu.sp, 0xfd);
        assert_eq!(rv, 0xff);
    }

    #[test]
    fn test_flags() {
        let mut cpu = CPU::new_nes_cpu();

        assert_eq!(cpu.flags(), 0x00);

        cpu.set_flags(0x24);
        assert_eq!(cpu.flags(), 0x24);

        cpu.set_flags(0x00);
        assert_eq!(cpu.flags(), 0x00);

        cpu.c = true;
        assert_eq!(cpu.flags(), 0x01);
    }
}
