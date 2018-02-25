use mem::Memory;
use opcode::{Opcode, OPCODES};

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

            sp: 0xfd,
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
    fn test_init() {
        let _cpu = CPU::new_nes_cpu();
        // add tests when it makes sense
    }
}
