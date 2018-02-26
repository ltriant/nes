use cpu::CPU;

#[derive(Debug)]
pub enum Instruction {
    None,
    JMP,
    LDX,
    STX,
    JSR,
    NOP,
    SEC,
    BCS,
    CLC,
    BCC,
    LDA,
    BEQ,
    BNE
}

impl Instruction {
    pub fn run(&self, cpu: &mut CPU, param: (u16, u8)) {
        match *self {
            Instruction::JMP => jmp(cpu, param),
            Instruction::LDX => ldx(cpu, param),
            Instruction::STX => stx(cpu, param),
            Instruction::JSR => jsr(cpu, param),
            Instruction::NOP => nop(cpu, param),
            Instruction::SEC => sec(cpu, param),
            Instruction::BCS => bcs(cpu, param),
            Instruction::CLC => clc(cpu, param),
            Instruction::BCC => bcc(cpu, param),
            Instruction::LDA => lda(cpu, param),
            Instruction::BEQ => beq(cpu, param),
            Instruction::BNE => bne(cpu, param),
            _ => panic!("unsupported instruction {:?}", *self),
        }
    }
}

fn update_sz(cpu: &mut CPU, val: u8) {
    cpu.s = val & 0x80 == 1;
    cpu.z = val == 0;
}

fn jmp(cpu: &mut CPU, (addr, _): (u16, u8)) {
    cpu.pc = addr;
}

fn ldx(cpu: &mut CPU, (_, val): (u16, u8)) {
    cpu.x = val;
    update_sz(cpu, val);
}

fn stx(cpu: &mut CPU, (addr, _): (u16, u8)) {
    cpu.mem.write(addr, cpu.x)
        .expect("STX failed");
}

fn jsr(cpu: &mut CPU, (addr, _): (u16, u8)) {
    let retaddr = cpu.pc;
    cpu.pc = addr;
    cpu.stack_push16(retaddr);
}

fn nop(_: &mut CPU, (_, _): (u16, u8)) { }

fn sec(cpu: &mut CPU, (_, _): (u16, u8)) {
    cpu.c = true;
}

fn bcs(cpu: &mut CPU, (addr, _): (u16, u8)) {
    if cpu.c {
        cpu.pc = addr;
    }
}

fn clc(cpu: &mut CPU, (_, _): (u16, u8)) {
    cpu.c = false;
}

fn bcc(cpu: &mut CPU, (addr, _): (u16, u8)) {
    if !cpu.c {
        cpu.pc = addr;
    }
}

fn lda(cpu: &mut CPU, (_, val): (u16, u8)) {
    cpu.a = val;
    update_sz(cpu, val);
}

fn beq(cpu: &mut CPU, (addr, _): (u16, u8)) {
    if cpu.z {
        cpu.pc = addr;
    }
}

fn bne(cpu: &mut CPU, (addr, _): (u16, u8)) {
    if !cpu.z {
        cpu.pc = addr;
    }
}
