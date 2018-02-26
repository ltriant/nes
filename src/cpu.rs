use mem::Memory;
use opcode::{Opcode, OPCODES};

const STACK_INIT: u8 = 0xfd;

// A, X, and Y are 8-bit registers
type Register = u8;

// Status flags
type Flag = bool;

// 16-bit register
type ProgramCounter = u16;

// 8-bit register
type StackPointer = u8;

pub struct CPU {
    pub mem: Memory,

    // Main registers
    pub a: Register,  // Accumulator
    pub x: Register,  // X Index
    pub y: Register,  // Y Index

    // Status register flags
    pub c: Flag,  // Carry
    z: Flag,  // Zero
    i: Flag,  // Interrupt
    d: Flag,  // Decimal mode
    b: Flag,  // Software interrupt (BRK)
    v: Flag,  // Overflow
    s: Flag,  // Sign

    // Program counter
    pub pc: ProgramCounter,

    // Stack pointer
    pub sp: StackPointer,
}

impl CPU {
    pub fn new_nes_cpu() -> CPU {
        CPU {
            mem: Memory::new_nes_mem(),

            a: 0,
            x: 0,
            y: 0,

            c: false,
            z: false,
            i: false,
            d: false,
            b: false,
            v: false,
            s: false,

            pc: 0xc000,

            sp: STACK_INIT,
        }
    }

    fn flags(&self) -> u8 {
        self.c as u8
            | ((self.z as u8) << 1)
            | ((self.i as u8) << 2)
            | ((self.d as u8) << 3)
            | ((self.b as u8) << 4)
            | (0 << 5)
            | ((self.v as u8) << 6)
            | ((self.s as u8) << 7)
    }

    fn debug(&self, op: &Opcode) {
        let Opcode(ref inst, ref addr_mode, _, _) = *op;
        let bytes = addr_mode.get_bytes(self)
            .iter()
            .map(|arg| String::from(format!("{:02X}", arg)))
            .collect::<Vec<_>>()
            .join(" ");

        println!("{:4X}  {:8}  {:02?} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
                 self.pc,
                 bytes,
                 inst,
                 self.a,
                 self.x,
                 self.y,
                 self.flags(),
                 self.sp);
    }

    pub fn stack_push8(&mut self, val: u8) {
        if self.sp == 0 {
            panic!("cannot push onto a full stack");
        }

        self.mem.write(self.sp as u16, val)
            .expect("unable to write to stack");
        self.sp -= 1;
    }

    pub fn stack_pop8(&mut self) -> u8 {
        if self.sp == STACK_INIT {
            panic!("cannot pop from an empty stack");
        }

        self.sp += 1;
        let val = self.mem.read(self.sp as u16)
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

    pub fn step(&mut self) {
        let opcode = self.mem.read(self.pc)
            .expect("unable to read next opcode");

        let op = &OPCODES[opcode as usize];
        self.debug(&op);

        let &Opcode(ref inst, ref addr_mode, ref bytes, ref _cycles) = op;
        let operand = addr_mode.get_data(self);
        self.pc += *bytes as u16;
        inst.run(self, operand);
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
        assert_eq!(cpu.mem.ram[(cpu.sp as usize) + 1], 0xff);

        cpu.stack_push16(0xdead);
        assert_eq!(cpu.sp, 0xfa);
        assert_eq!(cpu.mem.ram[(cpu.sp as usize) + 1], 0xad);
        assert_eq!(cpu.mem.ram[(cpu.sp as usize) + 2], 0xde);

        let rv = cpu.stack_pop16();
        assert_eq!(cpu.sp, 0xfc);
        assert_eq!(rv, 0xdead);

        let rv = cpu.stack_pop8();
        assert_eq!(cpu.sp, 0xfd);
        assert_eq!(rv, 0xff);
    }
}
