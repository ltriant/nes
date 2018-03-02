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
    Opcode(Instruction::PHP, AddressingMode::Implied, 1, 3),
    Opcode(Instruction::ORA, AddressingMode::Immediate, 2, 2),
    Opcode(Instruction::ASL, AddressingMode::Accumulator, 1, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::BPL, AddressingMode::Relative, 2, 2),
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
    Opcode(Instruction::BIT, AddressingMode::ZeroPageIndexed, 2, 3),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::PLP, AddressingMode::Implied, 1, 4),
    Opcode(Instruction::AND, AddressingMode::Immediate, 2, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::BMI, AddressingMode::Relative, 2, 2),
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
    Opcode(Instruction::PHA, AddressingMode::Implied, 1, 3),
    Opcode(Instruction::EOR, AddressingMode::Immediate, 2, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::JMP, AddressingMode::Absolute, 3, 3),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::BVC, AddressingMode::Relative, 2, 2),
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
    Opcode(Instruction::RTS, AddressingMode::Implied, 1, 6),
    Opcode(Instruction::ADC, AddressingMode::PreIndexedIndirectX, 2, 6),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::PLA, AddressingMode::Implied, 1, 4),
    Opcode(Instruction::ADC, AddressingMode::Immediate, 2, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::JMP, AddressingMode::Indirect, 3, 5),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::BVS, AddressingMode::Relative, 2, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::ADC, AddressingMode::ZeroPageAbsoluteX, 2, 4),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::SEI, AddressingMode::Implied, 1, 2),
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
    Opcode(Instruction::STY, AddressingMode::ZeroPageIndexed, 2, 3),
    Opcode(Instruction::STA, AddressingMode::ZeroPageIndexed, 2, 3),
    Opcode(Instruction::STX, AddressingMode::ZeroPageIndexed, 2, 3),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::DEY, AddressingMode::Implied, 1, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::TXA, AddressingMode::Implied, 1, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::STA, AddressingMode::Absolute, 3, 4),
    Opcode(Instruction::STX, AddressingMode::Absolute, 3, 4),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::BCC, AddressingMode::Relative, 2, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::TYA, AddressingMode::Implied, 1, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::TXS, AddressingMode::Implied, 1, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::LDY, AddressingMode::Immediate, 2, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::LDX, AddressingMode::Immediate, 2, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::LDA, AddressingMode::ZeroPageIndexed, 2, 3),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::TAY, AddressingMode::Implied, 1, 2),
    Opcode(Instruction::LDA, AddressingMode::Immediate, 2, 2),
    Opcode(Instruction::TAX, AddressingMode::Implied, 1, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::LDA, AddressingMode::Absolute, 3, 4),
    Opcode(Instruction::LDX, AddressingMode::Absolute, 3, 4),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::BCS, AddressingMode::Relative, 2, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::CLV, AddressingMode::Implied, 1, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::TSX, AddressingMode::Implied, 1, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::LDA, AddressingMode::AbsoluteX, 3, 4),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::CPY, AddressingMode::Immediate, 2, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::CMP, AddressingMode::ZeroPageIndexed, 2, 3),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::INY, AddressingMode::Implied, 1, 2),
    Opcode(Instruction::CMP, AddressingMode::Immediate, 2, 2),
    Opcode(Instruction::DEX, AddressingMode::Implied, 1, 2),
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
    Opcode(Instruction::CLD, AddressingMode::Implied, 1, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::CPX, AddressingMode::Immediate, 2, 2),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::INC, AddressingMode::ZeroPageIndexed, 2, 5),
    Opcode(Instruction::None, AddressingMode::None, 0, 0),
    Opcode(Instruction::INX, AddressingMode::Implied, 1, 2),
    Opcode(Instruction::SBC, AddressingMode::Immediate, 2, 2),
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
    Opcode(Instruction::SED, AddressingMode::Implied, 1, 2),
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
    AbsoluteX,
    IndexedY,
    ZeroPageIndexed,
    ZeroPageAbsoluteX,
    ZeroPageAbsoluteY,
    Indirect,
    PreIndexedIndirectX,
    PostIndexedIndirectY,
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
            AddressingMode::AbsoluteX => 3,
            AddressingMode::Indirect => 3,
            AddressingMode::ZeroPageAbsoluteX => 3,
            AddressingMode::PreIndexedIndirectX => 2,
            AddressingMode::PostIndexedIndirectY => 2,
            _ => {
                let opcode = cpu.mem.read(cpu.pc).unwrap();
                panic!("unsupported addressing mode {:?} at PC {:04X}, opcode {:02X}",
                       self,
                       cpu.pc,
                       opcode);
            }
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
                    .expect("Absolute arg 2") as u16;
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
                let offset = cpu.mem.read(pc + 1)
                    .expect("Relative arg") as u16;

                // TODO negative offset?

                // NOTE This has to be based off the current program counter,
                // _after_ it has been advanced, but before the instruction is
                // being executed. I don't know why though?
                (cpu.pc + offset, 0)
            },
            AddressingMode::AbsoluteX => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("AbsoluteX arg 1") as u16;
                let hi = cpu.mem.read(pc + 2)
                    .expect("AbsoluteX arg 2") as u16;
                let addr = (hi << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("AbsoluteX addr");
                (0, val + cpu.x)
            },
            AddressingMode::Indirect => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("Indirect arg 1") as u16;
                let hi = cpu.mem.read(pc + 2)
                    .expect("Indirect arg 2") as u16;
                let addr = (hi << 8) | lo;

                let lo = cpu.mem.read(addr)
                    .expect("Indirect addr 1") as u16;
                let hi = cpu.mem.read(addr + 1)
                    .expect("Indirect addr 2") as u16;
                let addr = (hi << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("Indirect addr val");

                (addr, val)
            }
            AddressingMode::ZeroPageAbsoluteX => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("ZeroPageAbsoluteX arg 1") as u16;
                let addr = (0 << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("ZeroPageAbsoluteX addr");
                (0, val + cpu.x)
            },
            AddressingMode::ZeroPageAbsoluteY => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("ZeroPageAbsoluteY arg 1") as u16;
                let addr = (0 << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("ZeroPageAbsoluteY addr");
                (0, val + cpu.y)
            },
            AddressingMode::PreIndexedIndirectX => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("PreIndexedIndirectX arg 1");
                let x = cpu.x;
                let addr = lo.wrapping_add(x) as u16;
                let val = cpu.mem.read(addr)
                    .expect("PreIndexedIndirectX val");
                (addr, val)
            },
            AddressingMode::PostIndexedIndirectY => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("PostIndexedIndirectY arg 1") as u16;
                let val = cpu.mem.read(lo)
                    .expect("PostIndexedIndirectY val");
                (lo, val + cpu.y)
            },
            _ => {
                let opcode = cpu.mem.read(pc).unwrap();
                panic!("unsupported addressing mode {:?} at PC {:04X}, opcode {:02X}",
                       self,
                       pc,
                       opcode);
            }
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
