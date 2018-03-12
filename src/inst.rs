use cpu::CPU;
use opcode::AddressingMode;

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
    BMI,
    ORA,
    CLV,
    EOR,
    ADC,
    LDY,
    CPY,
    CPX,
    SBC,
    STY,
    INY,
    INX,
    DEY,
    DEX,
    TAY,
    TAX,
    TYA,
    TXA,
    TSX,
    TXS,
    INC,
    ASL,
    RRA,
    RTI,
    ROR,
}

impl Instruction {
    pub fn run(&self, cpu: &mut CPU, (addr, val): (u16, u8), addr_mode: &AddressingMode) {
        match *self {
            Instruction::JMP => jmp(cpu, addr, val),
            Instruction::LDX => ldx(cpu, addr, val),
            Instruction::STX => stx(cpu, addr, val),
            Instruction::JSR => jsr(cpu, addr, val),
            Instruction::NOP => nop(cpu, addr, val),
            Instruction::SEC => sec(cpu, addr, val),
            Instruction::BCS => bcs(cpu, addr, val),
            Instruction::CLC => clc(cpu, addr, val),
            Instruction::BCC => bcc(cpu, addr, val),
            Instruction::LDA => lda(cpu, addr, val),
            Instruction::BEQ => beq(cpu, addr, val),
            Instruction::BNE => bne(cpu, addr, val),
            Instruction::STA => sta(cpu, addr, val),
            Instruction::BIT => bit(cpu, addr, val),
            Instruction::BVS => bvs(cpu, addr, val),
            Instruction::BVC => bvc(cpu, addr, val),
            Instruction::BPL => bpl(cpu, addr, val),
            Instruction::RTS => rts(cpu, addr, val),
            Instruction::SEI => sei(cpu, addr, val),
            Instruction::SED => sed(cpu, addr, val),
            Instruction::PHP => php(cpu, addr, val),
            Instruction::PLA => pla(cpu, addr, val),
            Instruction::AND => and(cpu, addr, val),
            Instruction::CMP => cmp(cpu, addr, val),
            Instruction::CLD => cld(cpu, addr, val),
            Instruction::PHA => pha(cpu, addr, val),
            Instruction::PLP => plp(cpu, addr, val),
            Instruction::BMI => bmi(cpu, addr, val),
            Instruction::ORA => ora(cpu, addr, val),
            Instruction::CLV => clv(cpu, addr, val),
            Instruction::EOR => eor(cpu, addr, val),
            Instruction::ADC => adc(cpu, addr, val),
            Instruction::LDY => ldy(cpu, addr, val),
            Instruction::CPY => cpy(cpu, addr, val),
            Instruction::CPX => cpx(cpu, addr, val),
            Instruction::SBC => sbc(cpu, addr, val),
            Instruction::STY => sty(cpu, addr, val),
            Instruction::INY => iny(cpu, addr, val),
            Instruction::INX => inx(cpu, addr, val),
            Instruction::DEY => dey(cpu, addr, val),
            Instruction::DEX => dex(cpu, addr, val),
            Instruction::TAY => tay(cpu, addr, val),
            Instruction::TAX => tax(cpu, addr, val),
            Instruction::TYA => tya(cpu, addr, val),
            Instruction::TXA => txa(cpu, addr, val),
            Instruction::TXS => txs(cpu, addr, val),
            Instruction::TSX => tsx(cpu, addr, val),
            Instruction::INC => inc(cpu, addr, val),
            Instruction::ASL => asl(cpu, addr, val, addr_mode),
            Instruction::RRA => rra(cpu, addr, val),
            Instruction::RTI => rti(cpu, addr, val),
            Instruction::ROR => ror(cpu, addr, val, addr_mode),
            _ => panic!("unsupported instruction {:?}", *self),
        }
    }
}

fn update_sz(cpu: &mut CPU, val: u8) {
    cpu.s = val & 0x80 != 0;
    cpu.z = val == 0;
}

fn jmp(cpu: &mut CPU, addr: u16, _: u8) {
    cpu.pc = addr;
}

fn ldx(cpu: &mut CPU, _: u16, val: u8) {
    cpu.x = val;
    update_sz(cpu, val);
}

fn stx(cpu: &mut CPU, addr: u16, _: u8) {
    cpu.mem.write(addr, cpu.x)
        .expect("STX failed");
}

fn jsr(cpu: &mut CPU, addr: u16, _: u8) {
    let retaddr = cpu.pc - 1;
    cpu.stack_push16(retaddr);
    cpu.pc = addr;
}

fn nop(_: &mut CPU, _: u16, _: u8) { }

fn sec(cpu: &mut CPU, _: u16, _: u8) {
    cpu.c = true;
}

fn bcs(cpu: &mut CPU, addr: u16, _: u8) {
    if cpu.c {
        cpu.pc = addr;
    }
}

fn clc(cpu: &mut CPU, _: u16, _: u8) {
    cpu.c = false;
}

fn bcc(cpu: &mut CPU, addr: u16, _: u8) {
    if !cpu.c {
        cpu.pc = addr;
    }
}

fn lda(cpu: &mut CPU, _: u16, val: u8) {
    cpu.a = val;
    update_sz(cpu, val);
}

fn beq(cpu: &mut CPU, addr: u16, _: u8) {
    if cpu.z {
        cpu.pc = addr;
    }
}

fn bne(cpu: &mut CPU, addr: u16, _: u8) {
    if !cpu.z {
        cpu.pc = addr;
    }
}

fn sta(cpu: &mut CPU, addr: u16, _: u8) {
    cpu.mem.write(addr, cpu.a)
        .expect("STA failed");
}

fn bit(cpu: &mut CPU, _: u16, val: u8) {
    let f = cpu.a & val;
    cpu.v = val & 0x40 != 0;
    update_sz(cpu, f);
}

fn bvs(cpu: &mut CPU, addr: u16, _: u8) {
    if cpu.v {
        cpu.pc = addr;
    }
}

fn bvc(cpu: &mut CPU, addr: u16, _: u8) {
    if !cpu.v {
        cpu.pc = addr;
    }
}

fn bpl(cpu: &mut CPU, addr: u16, _: u8) {
    if !cpu.s {
        cpu.pc = addr;
    }
}

fn rts(cpu: &mut CPU, _: u16, _: u8) {
    let retaddr = cpu.stack_pop16();
    cpu.pc = retaddr + 1;
}

fn sei(cpu: &mut CPU, _: u16, _: u8) {
    cpu.i = true;
}

fn sed(cpu: &mut CPU, _: u16, _: u8) {
    cpu.d = true;
}

fn php(cpu: &mut CPU, _: u16, _: u8) {
    // https://wiki.nesdev.com/w/index.php/CPU_status_flag_behavior
    // According to the above link, the PHP instruction sets bits 4 and 5 on
    // the value it pushes onto the stack.
    // The PLP call later will ignore these bits.
    let flags = cpu.flags() | 0x10;
    cpu.stack_push8(flags);
}

fn plp(cpu: &mut CPU, _: u16, _: u8) {
    let p = cpu.stack_pop8() & 0xef | 0x20;
    cpu.set_flags(p);
}

fn pla(cpu: &mut CPU, _: u16, _: u8) {
    let rv = cpu.stack_pop8();
    cpu.a = rv;
    update_sz(cpu, rv);
}

fn and(cpu: &mut CPU, _: u16, val: u8) {
    cpu.a &= val;
    let a = cpu.a;
    update_sz(cpu, a);
}

fn cmp(cpu: &mut CPU, _: u16, val: u8) {
    let n = cpu.a.wrapping_sub(val);
    cpu.c = cpu.a >= val;
    update_sz(cpu, n);
}

fn cld(cpu: &mut CPU, _: u16, _: u8) {
    cpu.d = false;
}

fn pha(cpu: &mut CPU, _: u16, _: u8) {
    let a = cpu.a;
    cpu.stack_push8(a);
}

fn bmi(cpu: &mut CPU, addr: u16, _: u8) {
    if cpu.s {
        cpu.pc = addr;
    }
}

fn ora(cpu: &mut CPU, _: u16, val: u8) {
    let na = cpu.a | val;
    cpu.a = val;
    update_sz(cpu, na);
}

fn clv(cpu: &mut CPU, _: u16, _: u8) {
    cpu.v = false;
}

fn eor(cpu: &mut CPU, _: u16, val: u8) {
    let val = val ^ cpu.a;
    cpu.a = val;
    update_sz(cpu, val);
}

fn adc(cpu: &mut CPU, _: u16, val: u8) {
    let n = (val as u16) + (cpu.a as u16) + (cpu.c as u16);
    cpu.v = cpu.a as u16 & 0x80 == 0 && n & 0x80 != 0;
    cpu.s = n & 0x80 != 0;
    cpu.z = n == 0;
    cpu.c = n > 0xff;
    cpu.a = (n & 0xff) as u8;
}

fn ldy(cpu: &mut CPU, _: u16, val: u8) {
    cpu.y = val;
    update_sz(cpu, val);
}

fn cpy(cpu: &mut CPU, _: u16, val: u8) {
    let n = cpu.y.wrapping_sub(val);
    cpu.s = n & 0x80 != 0;
    cpu.c = n > val;
    cpu.z = n == 0;
}

fn cpx(cpu: &mut CPU, _: u16, val: u8) {
    let n = cpu.x.wrapping_sub(val);
    cpu.s = n & 0x80 != 0;
    cpu.c = n > val;
    cpu.z = n == 0;
}

fn sbc(cpu: &mut CPU, _: u16, val: u8) {
    let n = cpu.a.wrapping_sub(val)
        .wrapping_sub(cpu.c as u8);
    cpu.c = n >= 0;
    cpu.a = n & 0xff;
    update_sz(cpu, n);
}

fn sty(cpu: &mut CPU, addr: u16, _: u8) {
    cpu.mem.write(addr, cpu.y)
        .expect("STY failed");
}

fn iny(cpu: &mut CPU, _: u16, _: u8) {
    let n = cpu.y.wrapping_add(1);
    cpu.y = n;
    update_sz(cpu, n);
}

fn dey(cpu: &mut CPU, _: u16, _: u8) {
    let n = cpu.y.wrapping_sub(1);
    cpu.y = n;
    update_sz(cpu, n);
}

fn inx(cpu: &mut CPU, _: u16, _: u8) {
    let n = cpu.x.wrapping_add(1);
    cpu.x = n;
    update_sz(cpu, n);
}

fn dex(cpu: &mut CPU, _: u16, _: u8) {
    let n = cpu.x.wrapping_sub(1);
    cpu.x = n;
    update_sz(cpu, n);
}

fn tay(cpu: &mut CPU, _: u16, _: u8) {
    let n = cpu.a;
    cpu.y = n;
    update_sz(cpu, n);
}

fn tax(cpu: &mut CPU, _: u16, _: u8) {
    let n = cpu.a;
    cpu.x = n;
    update_sz(cpu, n);
}

fn tya(cpu: &mut CPU, _: u16, _: u8) {
    let n = cpu.y;
    cpu.a = n;
    update_sz(cpu, n);
}

fn txa(cpu: &mut CPU, _: u16, _: u8) {
    let n = cpu.x;
    cpu.a = n;
    update_sz(cpu, n);
}

fn txs(cpu: &mut CPU, _: u16, _: u8) {
    cpu.sp = cpu.x;
}

fn tsx(cpu: &mut CPU, _: u16, _: u8) {
    let x = cpu.stack_pop8();
    cpu.x = x;
}

fn inc(cpu: &mut CPU, addr: u16, val: u8) {
    let n = (val + 1) & 0xff;
    cpu.mem.write(addr, n)
        .expect("INC failed");
    update_sz(cpu, n);
}

fn asl(cpu: &mut CPU, addr: u16, val: u8, addr_mode: &AddressingMode) {
    cpu.c = val & 0x80 == 1;
    let n = (val << 1) & 0xff;

    // TODO When the addressing mode is accumulator, the addr variable will be 0... ?
    match *addr_mode {
        AddressingMode::Accumulator => { cpu.a = n; },
        _ => { cpu.mem.write(addr, n).expect("ASL failed"); }
    };

    update_sz(cpu, n);
}

fn rti(cpu: &mut CPU, _: u16, _: u8) {
    let flags = cpu.stack_pop8() & 0xef;
    update_sz(cpu, flags);
    cpu.set_flags(flags);

    let retaddr = cpu.stack_pop16();
    cpu.pc = retaddr;
}

fn ror(cpu: &mut CPU, addr: u16, val: u8, addr_mode: &AddressingMode) {
    panic!("ROR");
}

// Illegal opcodes
fn rra(_: &mut CPU, _: u16, _: u8) { }
