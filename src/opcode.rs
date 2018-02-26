use cpu::CPU;
use inst::Instruction;

pub const OPCODES: [Opcode; 256] = [
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::CLC, AddressingMode::Implied, 1, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::JSR, AddressingMode::Absolute, 3, 6),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::SEC, AddressingMode::Implied, 1, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::JMP, AddressingMode::Absolute, 3, 3),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::STX, AddressingMode::ZeroPageIndexed, 2, 3),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::BCC, AddressingMode::Relative, 2, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::LDX, AddressingMode::Immediate, 2, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::LDA, AddressingMode::Immediate, 2, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::BCS, AddressingMode::Relative, 2, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::BNE, AddressingMode::Relative, 2, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::NOP, AddressingMode::Implied, 1, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::BEQ, AddressingMode::Relative, 2, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
];

#[derive(Debug)]
pub struct Opcode(pub Instruction, pub AddressingMode, pub u8, pub u8);

#[derive(Debug)]
pub enum AddressingMode {
    None,
    Immediate,
    ZeroPageAbsolute,
    Absolute,
    Implied,
    Accumulator,
    Indexed,
    ZeroPageIndexed,
    ZeroPageIndexedX,
    ZeroPageIndexedY,
    Indirect,
    PreIndexedIndirect,
    PostIndexedIndirect,
    Relative,
}

impl AddressingMode {
    pub fn get_bytes(&self, cpu: &CPU) -> Vec<u8> {
        let n_bytes = match *self {
            AddressingMode::Immediate => 2,
            AddressingMode::ZeroPageAbsolute => 2,
            AddressingMode::Absolute => 3,
            AddressingMode::Implied => 1,
            AddressingMode::Accumulator => 1,
            AddressingMode::ZeroPageIndexed => 2,
            AddressingMode::Relative => 2,
            _ => panic!("unsupported addressing mode {:?} at PC {:04X}", self, cpu.pc)
        };

        (0 .. n_bytes).map(|n| cpu.mem.read(cpu.pc + n).unwrap())
            .collect::<Vec<_>>()
    }

    pub fn get_data(&self, cpu: &CPU, pc: u16) -> (u16, u8) {
        match *self {
            AddressingMode::Immediate => {
                let addr = pc + 1;
                let val = cpu.mem.read(addr)
                    .expect("Immediate val");
                (addr, val)
            },
            AddressingMode::ZeroPageAbsolute => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("ZeroPageAbsolute arg") as u16;
                let addr = (0x00 << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("Absolute addr");
                (addr, val)
            },
            AddressingMode::Absolute => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("Absolute arg 1") as u16;
                let hi = cpu.mem.read(pc + 2)
                    .expect("Absolute arg 1") as u16;
                let addr = (hi << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("Absolute addr");
                (addr, val)
            },
            AddressingMode::Implied => (0, 0),
            AddressingMode::Accumulator => (0, cpu.a),
            AddressingMode::ZeroPageIndexed => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("ZeroPageIndexed arg") as u16;
                let addr = (0x00 << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("ZeroPageIndexed addr");
                (addr, val)
            },
            AddressingMode::Relative => {
                let offset = (cpu.mem.read(pc + 1)
                    .expect("Relative arg") as i8) as u16;

                // NOTE This has to be based off the current program counter,
                // _after_ it has been advanced, but before the instruction is
                // being executed. I don't know why though?
                (cpu.pc + offset, 0)
            },
            _ => panic!("unsupported addressing mode {:?} at PC {:04X}", self, pc)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addr_mode_immediate() {
        let _cpu = CPU::new_nes_cpu();
        // write ROM data
    }
}
