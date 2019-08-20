use std::process;

use crate::console::NES_CPU_DEBUG;
use crate::addr::AddressingMode;
use crate::mem::{Memory, NESMemory};
use crate::opcode::{Opcode, OPCODES};

const STACK_INIT: u8 = 0xfd;
const PPU_DOTS_PER_SCANLINE: usize = 341;

pub struct CPU {
    pub mem: NESMemory,

    // Main registers
    pub a: u8,  // Accumulator
    pub x: u8,  // X Index
    pub y: u8,  // Y Index

    // Status register flags
    pub c: bool,  // Carry
    pub z: bool,  // Zero
    pub i: bool,  // Interrupt
    pub d: bool,  // Decimal mode
    pub b: bool,  // Software interrupt (BRK)
    pub u: bool,  // Unused flag
    pub v: bool,  // Overflow
    pub s: bool,  // Sign

    // Program counter
    pub pc: u16,

    // Stack pointer
    pub sp: u8,

    cycles: usize,
}

impl CPU {
    pub fn new_nes_cpu(mem: NESMemory) -> Self {
        Self {
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
        self.pc = if *NES_CPU_DEBUG { 0xc000 } else { addr };
        debug!("PC: 0x{:04X}", self.pc);

        self.set_flags(0x24);
        debug!("flags: 0x{:02X}", self.flags());
    }

    fn flags(&self) -> u8 {
           (self.c as u8)
        | ((self.z as u8) << 1)
        | ((self.i as u8) << 2)
        | ((self.d as u8) << 3)
        | ((self.b as u8) << 4)
        | ((self.u as u8) << 5)
        | ((self.v as u8) << 6)
        | ((self.s as u8) << 7)
    }

    fn set_flags(&mut self, val: u8) {
        self.c = val & 0x01 == 1;
        self.z = (val >> 1 & 0x01) == 1;
        self.i = (val >> 2 & 0x01) == 1;
        self.d = (val >> 3 & 0x01) == 1;
        self.b = (val >> 4 & 0x01) == 1;
        self.u = (val >> 5 & 0x01) == 1;
        self.v = (val >> 6 & 0x01) == 1;
        self.s = (val >> 7 & 0x01) == 1;
    }

    fn debug(&mut self, op: &Opcode) {
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

    pub fn nmi(&mut self) {
        let pc = self.pc;
        self.stack_push16(pc);
        self.php();

        let lo = self.mem.read(0xFFFA).expect("low NMI byte") as u16;
        let hi = self.mem.read(0xFFFB).expect("high NMI byte") as u16;
        let addr = (hi << 8) | lo;
        self.cycles += 7;
        self.cycles %= PPU_DOTS_PER_SCANLINE;

        debug!("NMI: {:04X}", addr);
        self.pc = addr;
    }

    fn stack_push8(&mut self, val: u8) {
        // The stack page exists from 0x0100 to 0x01FF
        let addr = (0x01 << 8) | self.sp as u16;
        self.mem.write(addr, val)
            .expect("unable to write to stack");

        let n = self.sp.wrapping_sub(1);
        self.sp = n;
    }

    fn stack_pop8(&mut self) -> u8 {
        let n = self.sp.wrapping_add(1);
        self.sp = n;

        // The stack page exists from 0x0100 to 0x01FF
        let addr = (0x01 << 8) | self.sp as u16;
        let val = self.mem.read(addr)
            .expect("unable to read from stack");

        val
    }

    fn stack_push16(&mut self, val: u16) {
        let hi = (val >> 8) as u8;
        self.stack_push8(hi);

        let lo = (val & 0x00ff) as u8;
        self.stack_push8(lo);
    }

    fn stack_pop16(&mut self) -> u16 {
        let lo = self.stack_pop8() as u16;
        let hi = self.stack_pop8() as u16;
        (hi << 8) | lo
    }

    fn update_sz(&mut self, val: u8) {
        self.s = val & 0x80 != 0;
        self.z = val == 0;
    }

    fn add_branch_cycles(&mut self, pc: u16, addr: u16) {
        self.cycles += 1;

        if (pc & 0xff00) != (addr & 0xff00) {
            self.cycles += 1;
        }
    }

    pub fn step(&mut self) -> usize {
        let opcode = self.mem.read(self.pc)
            .expect("unable to read next opcode");

        let op = &OPCODES[opcode as usize];

        if *NES_CPU_DEBUG {
            self.debug(&op);
        }

        let &Opcode(ref inst, ref addr_mode, ref cycles, ref extra_cycles) = op;

        let mut n_cycles = 0;

        if let Ok(bytes) = addr_mode.n_bytes() {
            self.pc += bytes as u16;
            self.cycles = (self.cycles + cycles) % PPU_DOTS_PER_SCANLINE;
            n_cycles += cycles;

            if let Ok((addr, val, page_crossed)) = addr_mode.get_data(self) {
                inst.run(self, addr, val, addr_mode);

                if page_crossed {
                    self.cycles += extra_cycles;
                    n_cycles += extra_cycles;
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

        n_cycles
    }

    //
    // Legal instructions
    //

    pub fn adc(&mut self, val: u8) {
        let n = (val as u16) + (self.a as u16) + (self.c as u16);

        let a = (n & 0xff) as u8;
        self.update_sz(a);

        self.c = n > 0xff;

        // I took this from the NesDev forums.
        // It's only concerned with the 8th bit, which indicates the sign of each
        // value. The overflow bit is set if adding two positive numbers results
        // in a negative, or if adding two negative numbers results in a positive.
        self.v = ((self.a ^ val) & 0x80 == 0) && ((self.a ^ n as u8) & 0x80 > 0);

        self.a = a;
    }

    pub fn and(&mut self, val: u8) {
        self.a &= val;
        let a = self.a;
        self.update_sz(a);
    }

    pub fn asl(&mut self, addr: u16, val: u8, addr_mode: &AddressingMode) {
        self.c = val & 0x80 != 0;
        let n = (val << 1) & 0xff;

        match *addr_mode {
            AddressingMode::Accumulator => { self.a = n; },
            _ => { self.mem.write(addr, n).expect("ASL failed"); }
        };

        self.update_sz(n);
    }

    pub fn bcc(&mut self, addr: u16) {
        if !self.c {
            let pc = self.pc;
            self.add_branch_cycles(pc, addr);
            self.pc = addr;
        }
    }

    pub fn bcs(&mut self, addr: u16) {
        if self.c {
            let pc = self.pc;
            self.add_branch_cycles(pc, addr);
            self.pc = addr;
        }
    }

    pub fn beq(&mut self, addr: u16) {
        if self.z {
            let pc = self.pc;
            self.add_branch_cycles(pc, addr);
            self.pc = addr;
        }
    }

    pub fn bit(&mut self, val: u8) {
        self.s = val & 0x80 != 0;
        self.v = (val >> 0x06 & 0x01) == 1;
        let f = self.a & val;
        self.z = f == 0;
    }

    pub fn bmi(&mut self, addr: u16) {
        if self.s {
            let pc = self.pc;
            self.add_branch_cycles(pc, addr);
            self.pc = addr;
        }
    }

    pub fn bne(&mut self, addr: u16) {
        if !self.z {
            let pc = self.pc;
            self.add_branch_cycles(pc, addr);
            self.pc = addr;
        }
    }

    pub fn bpl(&mut self, addr: u16) {
        if !self.s {
            let pc = self.pc;
            self.add_branch_cycles(pc, addr);
            self.pc = addr;
        }
    }

    pub fn brk(&mut self) {
        let pc = self.pc + 1;
        self.stack_push16(pc);

        self.b = true;

        let flags = self.flags() | 0x10;
        self.stack_push8(flags);

        self.i = true;

        let lo = self.mem.read(0xFFFE).expect("BRK read low byte") as u16;
        let hi = self.mem.read(0xFFFF).expect("BRK read high byte") as u16;
        let pc = (hi << 8) | lo;
        self.pc = pc;
    }

    pub fn bvc(&mut self, addr: u16) {
        if !self.v {
            let pc = self.pc;
            self.add_branch_cycles(pc, addr);
            self.pc = addr;
        }
    }

    pub fn bvs(&mut self, addr: u16) {
        if self.v {
            let pc = self.pc;
            self.add_branch_cycles(pc, addr);
            self.pc = addr;
        }
    }

    pub fn clc(&mut self) {
        self.c = false;
    }

    pub fn cld(&mut self) {
        self.d = false;
    }

    pub fn clv(&mut self) {
        self.v = false;
    }

    pub fn cmp(&mut self, val: u8) {
        let n = self.a.wrapping_sub(val);
        self.c = self.a >= val;
        self.update_sz(n);
    }

    pub fn cpx(&mut self, val: u8) {
        let n = self.x.wrapping_sub(val);
        self.update_sz(n);
        self.c = self.x >= val;
    }

    pub fn cpy(&mut self, val: u8) {
        let n = self.y.wrapping_sub(val);
        self.update_sz(n);
        self.c = self.y >= val;
    }

    pub fn dec(&mut self, addr: u16, val: u8) {
        let n = val.wrapping_sub(1);
        self.update_sz(n);
        self.mem.write(addr, n)
            .expect("DEC failed");
    }

    pub fn dex(&mut self) {
        let n = self.x.wrapping_sub(1);
        self.x = n;
        self.update_sz(n);
    }

    pub fn dey(&mut self) {
        let n = self.y.wrapping_sub(1);
        self.y = n;
        self.update_sz(n);
    }

    pub fn eor(&mut self, val: u8) {
        let val = val ^ self.a;
        self.a = val;
        self.update_sz(val);
    }

    pub fn inc(&mut self, addr: u16, val: u8) {
        let n = val.wrapping_add(1);
        self.mem.write(addr, n)
            .expect("INC failed");
        self.update_sz(n);
    }

    pub fn inx(&mut self) {
        let n = self.x.wrapping_add(1);
        self.x = n;
        self.update_sz(n);
    }

    pub fn iny(&mut self) {
        let n = self.y.wrapping_add(1);
        self.y = n;
        self.update_sz(n);
    }

    pub fn jmp(&mut self, addr: u16) {
        self.pc = addr;
    }

    pub fn jsr(&mut self, addr: u16) {
        let retaddr = self.pc - 1;
        self.stack_push16(retaddr);
        self.pc = addr;
    }

    pub fn lda(&mut self, val: u8) {
        self.a = val;
        self.update_sz(val);
    }

    pub fn ldx(&mut self, val: u8) {
        self.x = val;
        self.update_sz(val);
    }

    pub fn ldy(&mut self, val: u8) {
        self.y = val;
        self.update_sz(val);
    }

    pub fn lsr(&mut self, addr: u16, val: u8, addr_mode: &AddressingMode) {
        self.c = val & 0x01 == 1;
        let n = val >> 1;
        self.update_sz(n);

        match *addr_mode {
            AddressingMode::Accumulator => { self.a = n; },
            _ => { self.mem.write(addr, n).expect("LSR failed"); }
        };
    }

    pub fn nop(&self) { }

    pub fn ora(&mut self, val: u8) {
        let na = self.a | val;
        self.a = na;
        self.update_sz(na);
    }

    pub fn pha(&mut self) {
        let a = self.a;
        self.stack_push8(a);
    }

    pub fn php(&mut self) {
        // https://wiki.nesdev.com/w/index.php/CPU_status_flag_behavior
        // According to the above link, the PHP instruction sets bits 4 and 5 on
        // the value it pushes onto the stack.
        // The PLP call later will ignore these bits.
        let flags = self.flags() | 0x10;
        self.stack_push8(flags);
    }

    pub fn pla(&mut self) {
        let rv = self.stack_pop8();
        self.a = rv;
        self.update_sz(rv);
    }

    pub fn plp(&mut self) {
        let p = self.stack_pop8() & 0xef | 0x20;
        self.set_flags(p);
    }

    pub fn rol(&mut self, addr: u16, val: u8, addr_mode: &AddressingMode) {
        let c = self.c;
        self.c = val & 0x80 != 0;
        let n = (val << 1) | (c as u8);
        self.update_sz(n);

        match *addr_mode {
            AddressingMode::Accumulator => { self.a = n; },
            _ => { self.mem.write(addr, n).expect("ROR failed"); }
        };
    }

    pub fn ror(&mut self, addr: u16, val: u8, addr_mode: &AddressingMode) {
        let c = self.c;
        self.c = val & 0x01 == 1;
        let n = (val >> 1) | ((c as u8) << 7);
        self.update_sz(n);

        match *addr_mode {
            AddressingMode::Accumulator => { self.a = n; },
            _ => { self.mem.write(addr, n).expect("ROR failed"); }
        };
    }

    pub fn rti(&mut self) {
        let flags = self.stack_pop8() & 0xef | 0x20;
        self.set_flags(flags);

        let retaddr = self.stack_pop16();
        self.pc = retaddr;
    }

    pub fn rts(&mut self) {
        let retaddr = self.stack_pop16();
        self.pc = retaddr + 1;
    }

    pub fn sbc(&mut self, val: u8) {
        let n: i8 = (self.a as i8)
            .wrapping_sub(val as i8)
            .wrapping_sub(1 - self.c as i8) ;

        let a = n as u8;
        self.update_sz(a);
        self.c = n >= 0;
        self.v = ((self.a ^ val) & 0x80 > 0) && ((self.a ^ n as u8) & 0x80 > 0);
        self.a = a;
    }

    pub fn sec(&mut self) {
        self.c = true;
    }

    pub fn sed(&mut self) {
        self.d = true;
    }

    pub fn sei(&mut self) {
        self.i = true;
    }

    pub fn sta(&mut self, addr: u16) {
        self.mem.write(addr, self.a)
            .expect("STA failed");
    }

    pub fn stx(&mut self, addr: u16) {
        self.mem.write(addr, self.x)
            .expect("STX failed");
    }

    pub fn sty(&mut self, addr: u16) {
        self.mem.write(addr, self.y)
            .expect("STY failed");
    }

    pub fn tax(&mut self) {
        let n = self.a;
        self.x = n;
        self.update_sz(n);
    }

    pub fn tay(&mut self) {
        let n = self.a;
        self.y = n;
        self.update_sz(n);
    }

    pub fn tsx(&mut self) {
        let s = self.sp;
        self.update_sz(s);
        self.x = s;
    }

    pub fn txa(&mut self) {
        let n = self.x;
        self.a = n;
        self.update_sz(n);
    }

    pub fn txs(&mut self) {
        self.sp = self.x;
    }

    pub fn tya(&mut self) {
        let n = self.y;
        self.a = n;
        self.update_sz(n);
    }

    //
    // Illegal instructions
    //

    pub fn anc(&mut self) { }

    pub fn lax(&mut self, val: u8) {
        self.a = val;
        self.x = val;
        self.update_sz(val);
    }

    pub fn sax(&mut self, addr: u16) {
        let val = self.x & self.a;
        self.mem.write(addr, val)
            .expect("SAX failed");
    }

    pub fn dcp(&mut self, addr: u16, val: u8) {
        // dec value
        let n = val.wrapping_sub(1);
        self.update_sz(n);
        self.mem.write(addr, n)
            .expect("DEC failed");

        // cmp with A register
        let n = self.a.wrapping_sub(n);
        self.c = self.a >= n;
        self.update_sz(n);
    }

    pub fn isb(&mut self, addr: u16, val: u8) {
        self.inc(addr, val);
        let n = self.mem.read(addr)
            .expect("ISB failed");
        self.sbc(n);
        self.c = (n as i8) >= 0;
    }

    pub fn slo(&mut self, addr: u16, val: u8, addr_mode: &AddressingMode) {
        self.asl(addr, val, addr_mode);
        let val = self.mem.read(addr)
            .expect("SLO failed");
        self.ora(val);
    }

    pub fn rla(&mut self, addr: u16, val: u8, addr_mode: &AddressingMode) {
        self.rol(addr, val, addr_mode);
        let val = self.mem.read(addr)
            .expect("RLA failed");
        self.and(val);
    }

    pub fn sre(&mut self, addr: u16, val: u8, addr_mode: &AddressingMode) {
        self.lsr(addr, val, addr_mode);
        let val = self.mem.read(addr)
            .expect("SRE failed");
        self.eor(val);
    }

    pub fn rra(&mut self, addr: u16, val: u8, addr_mode: &AddressingMode) {
        self.ror(addr, val, addr_mode);
        let val = self.mem.read(addr)
            .expect("RRA failed");
        self.adc(val);
    }

    pub fn jam(&mut self) {
        process::exit(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ppu::PPU;
    use crate::controller::Controller;

    #[test]
    fn test_stack_pop_empty() {
        let ppu = PPU::new_nes_ppu();
        let ctrl = Controller::new_controller();
        let mem = NESMemory::new_nes_mem(ppu, ctrl);
        let mut cpu = CPU::new_nes_cpu(mem);
        let _ = cpu.stack_pop8();
        assert_eq!(cpu.sp, STACK_INIT + 1);

        let _ = cpu.stack_pop8();
        assert_eq!(cpu.sp, STACK_INIT + 2);

        // The stack pointer should wrap around from 0xff to 0x00
        // TODO verify this behaviour
        let _ = cpu.stack_pop8();
        assert_eq!(cpu.sp, 0x00);
    }

    #[test]
    fn test_stack_push_full() {
        let ppu = PPU::new_nes_ppu();
        let ctrl = Controller::new_controller();
        let mem = NESMemory::new_nes_mem(ppu, ctrl);
        let mut cpu = CPU::new_nes_cpu(mem);

        for _ in 0 .. STACK_INIT {
            cpu.stack_push8(0xff);
        }

        assert_eq!(cpu.sp, 0x00);

        // The stack pointer should wrap around from 0x00 to 0xff
        // TODO verify this behaviour
        cpu.stack_push8(0xee);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn test_stack() {
        let ppu = PPU::new_nes_ppu();
        let ctrl = Controller::new_controller();
        let mem = NESMemory::new_nes_mem(ppu, ctrl);
        let mut cpu = CPU::new_nes_cpu(mem);

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
        let ppu = PPU::new_nes_ppu();
        let ctrl = Controller::new_controller();
        let mem = NESMemory::new_nes_mem(ppu, ctrl);
        let mut cpu = CPU::new_nes_cpu(mem);

        assert_eq!(cpu.flags(), 0x00);

        cpu.set_flags(0x24);
        assert_eq!(cpu.flags(), 0x24);

        cpu.set_flags(0x00);
        assert_eq!(cpu.flags(), 0x00);

        cpu.c = true;
        assert_eq!(cpu.flags(), 0x01);
    }

    #[test]
    fn test_nmi() {
        let ppu = PPU::new_nes_ppu();
        let ctrl = Controller::new_controller();
        let mem = NESMemory::new_nes_mem(ppu, ctrl);
        let mut cpu = CPU::new_nes_cpu(mem);

        let mut rom = vec![0; 0xffff];
        rom[0xfffa] = 0xad;
        rom[0xfffb] = 0xde;
        cpu.mem.load_rom(&rom);
        cpu.nmi();
        assert_eq!(cpu.pc, 0xdead);
    }
}
