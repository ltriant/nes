use mem::Memory;
use opcode::Opcode;

use std::collections::HashMap;

const INST_JMP: u8 = 0x4c;

// A, X, and Y are 8-bit registers
type Register = u8;

// Status flags
type Flag = bool;

// 16-bit register
type ProgramCounter = u16;

// 8-bit register
type StackPointer = u16;

// The available CPU addressing modes
enum AddressingMode {
    ZeroPageIndexed,
    AbsoluteIndexed,
    IndirectIndexed,
    IndexedIndirect,
    Implied,
}

pub struct CPU {
    pub mem: Memory,

    call_table: HashMap<u8, Opcode>,

    // Main registers
    a: Register,  // Accumulator
    x: Register,  // X Index
    y: Register,  // Y Index

    // Status register flags
    c: Flag,  // Carry
    z: Flag,  // Zero
    i: Flag,  // Interrupt
    d: Flag,  // Decimal mode
    b: Flag,  // Software interrupt (BRK)
    v: Flag,  // Overflow
    s: Flag,  // Sign

    // Program counter
    pub pc: ProgramCounter,

    // Stack pointer
    sp: StackPointer,
}

impl CPU {
    pub fn new_nes_cpu() -> CPU {
        let mut cpu = CPU {
            mem: Memory::new_nes_mem(),

            call_table: HashMap::new(),

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
        };

        cpu.call_table.insert(0x4c, Opcode::Jump);
        cpu
    }

    fn debug(&self, o: &Opcode) {
        let (code, name, _bytes, _cycles) = o.debug_data();
        println!("{:4X}  {:02X}  {:32} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
                 self.pc,
                 code,
                 name,
                 self.a,
                 self.x,
                 self.y,
                 0, // TODO status flags or'd together
                 self.sp);
    }

    pub fn step(&mut self) {
        let opcode = self.mem.read(self.pc)
            .expect("unable to read next opcode");

        let op = self.call_table.get(&opcode)
            .expect("unsupported opcode");

        self.debug(&op);
        op.execute(&self);
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
