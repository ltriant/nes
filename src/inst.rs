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
    BNE,
    STA,
    BIT,
    BVS,
    BVC,
    BPL,
    RTS,
    SEI,
    SED,
    PHP,
    PLA,
    AND,
    CMP,
    CLD,
    PHA,
    PLP,
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
            Instruction::STA => sta(cpu, param),
            Instruction::BIT => bit(cpu, param),
            Instruction::BVS => bvs(cpu, param),
            Instruction::BVC => bvc(cpu, param),
            Instruction::BPL => bpl(cpu, param),
            Instruction::RTS => rts(cpu, param),
            Instruction::SEI => sei(cpu, param),
            Instruction::SED => sed(cpu, param),
            Instruction::PHP => php(cpu, param),
            Instruction::PLA => pla(cpu, param),
            Instruction::AND => and(cpu, param),
            Instruction::CMP => cmp(cpu, param),
            Instruction::CLD => cld(cpu, param),
            Instruction::PHA => pha(cpu, param),
            Instruction::PLP => plp(cpu, param),
            _ => panic!("unsupported instruction {:?}", *self),
        }
    }
}

fn update_sz(cpu: &mut CPU, val: u8) {
    cpu.s = val & 0x80 != 0;
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

fn sta(cpu: &mut CPU, (addr, _): (u16, u8)) {
    cpu.mem.write(addr, cpu.a)
        .expect("STA failed");
}

fn bit(cpu: &mut CPU, (_, val): (u16, u8)) {
    let f = cpu.a & val;
    cpu.v = val & 0x40 != 0;
    update_sz(cpu, f);
}

fn bvs(cpu: &mut CPU, (addr, _): (u16, u8)) {
    if cpu.v {
        cpu.pc = addr;
    }
}

fn bvc(cpu: &mut CPU, (addr, _): (u16, u8)) {
    if !cpu.v {
        cpu.pc = addr;
    }
}

fn bpl(cpu: &mut CPU, (addr, _): (u16, u8)) {
    if !cpu.s {
        cpu.pc = addr;
    }
}

fn rts(cpu: &mut CPU, (_, _): (u16, u8)) {
    let retaddr = cpu.stack_pop16();
    cpu.pc = retaddr;
}

fn sei(cpu: &mut CPU, (_, _): (u16, u8)) {
    cpu.i = true;
}

fn sed(cpu: &mut CPU, (_, _): (u16, u8)) {
    cpu.d = true;
}

fn php(cpu: &mut CPU, (_, _): (u16, u8)) {
    let flags = cpu.flags();
    cpu.stack_push8(flags);
}

fn pla(cpu: &mut CPU, (_, _): (u16, u8)) {
    let rv = cpu.stack_pop8();
    cpu.a = rv;
    update_sz(cpu, rv);
}

fn and(cpu: &mut CPU, (_, val): (u16, u8)) {
    cpu.a &= val;
    let a = cpu.a;
    update_sz(cpu, a);
}

fn cmp(cpu: &mut CPU, (_, val): (u16, u8)) {
    let n = cpu.a.wrapping_sub(val);
    cpu.s = n & 0x80 != 0;
    cpu.c = n > val;
    cpu.z = n == 0;
}

fn cld(cpu: &mut CPU, (_, _): (u16, u8)) {
    cpu.d = false;
}

fn pha(cpu: &mut CPU, (_, _): (u16, u8)) {
    let a = cpu.a;
    cpu.stack_push8(a);
}

fn plp(cpu: &mut CPU, (_, _): (u16, u8)) {
    let p = cpu.stack_pop8();
    cpu.set_flags(p);
}
