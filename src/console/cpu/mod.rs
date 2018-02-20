mod mem;

use self::mem::Memory;

// A, X, and Y are 8-bit registers
type Register = u8;

// Status flags
type Flag = bool;

// 16-bit register
type ProgramCounter = usize;

// 8-bit register
type StackPointer = usize;

pub struct CPU {
    pub mem: Memory,

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
    pc: ProgramCounter,

    // Stack pointer
    sp: StackPointer,
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

    pub fn step(&self) {
        if let Ok(opcode) = self.mem.read(self.pc) {
            println!("{:?}", opcode);
            // get addressing mode based on opcode

            // get operands via addressing mode

            // exec instruction
        }
        else {
            panic!("out of bounds pc: {}", self.pc);
        }
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
