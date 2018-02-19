type Register = u8;
type Flag = bool;
type ProgramCounter = u8;
type StackPointer = u8;

#[allow(dead_code)]
pub struct CPU {
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
    fn new_nes_cpu() -> CPU {
        CPU {
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

            pc: 0,

            sp: 0,
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
