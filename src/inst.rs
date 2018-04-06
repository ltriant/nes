use mem::Memory;
use cpu::CPU;
use addr::AddressingMode;

#[derive(Debug)]
pub enum Instruction {
    None,
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

impl Instruction {
    pub fn run(&self, cpu: &mut CPU, addr: u16, val: u8, addr_mode: &AddressingMode) {
        match *self {
            Instruction::ADC => adc(cpu, addr, val),
            Instruction::AND => and(cpu, addr, val),
            Instruction::ASL => asl(cpu, addr, val, addr_mode),
            Instruction::BCC => bcc(cpu, addr, val),
            Instruction::BCS => bcs(cpu, addr, val),
            Instruction::BEQ => beq(cpu, addr, val),
            Instruction::BIT => bit(cpu, addr, val),
            Instruction::BMI => bmi(cpu, addr, val),
            Instruction::BNE => bne(cpu, addr, val),
            Instruction::BPL => bpl(cpu, addr, val),
            Instruction::BVC => bvc(cpu, addr, val),
            Instruction::BVS => bvs(cpu, addr, val),
            Instruction::CLC => clc(cpu, addr, val),
            Instruction::CLD => cld(cpu, addr, val),
            Instruction::CLV => clv(cpu, addr, val),
            Instruction::CMP => cmp(cpu, addr, val),
            Instruction::CPX => cpx(cpu, addr, val),
            Instruction::CPY => cpy(cpu, addr, val),
            Instruction::DEC => dec(cpu, addr, val),
            Instruction::DEX => dex(cpu, addr, val),
            Instruction::DEY => dey(cpu, addr, val),
            Instruction::EOR => eor(cpu, addr, val),
            Instruction::INC => inc(cpu, addr, val),
            Instruction::INX => inx(cpu, addr, val),
            Instruction::INY => iny(cpu, addr, val),
            Instruction::JMP => jmp(cpu, addr, val),
            Instruction::JSR => jsr(cpu, addr, val),
            Instruction::LDA => lda(cpu, addr, val),
            Instruction::LDX => ldx(cpu, addr, val),
            Instruction::LDY => ldy(cpu, addr, val),
            Instruction::LSR => lsr(cpu, addr, val, addr_mode),
            Instruction::NOP => nop(cpu, addr, val),
            Instruction::ORA => ora(cpu, addr, val),
            Instruction::PHA => pha(cpu, addr, val),
            Instruction::PHP => php(cpu, addr, val),
            Instruction::PLA => pla(cpu, addr, val),
            Instruction::PLP => plp(cpu, addr, val),
            Instruction::ROL => rol(cpu, addr, val, addr_mode),
            Instruction::ROR => ror(cpu, addr, val, addr_mode),
            Instruction::RTI => rti(cpu, addr, val),
            Instruction::RTS => rts(cpu, addr, val),
            Instruction::SBC => sbc(cpu, addr, val),
            Instruction::SEC => sec(cpu, addr, val),
            Instruction::SED => sed(cpu, addr, val),
            Instruction::SEI => sei(cpu, addr, val),
            Instruction::STA => sta(cpu, addr, val),
            Instruction::STX => stx(cpu, addr, val),
            Instruction::STY => sty(cpu, addr, val),
            Instruction::TAX => tax(cpu, addr, val),
            Instruction::TAY => tay(cpu, addr, val),
            Instruction::TSX => tsx(cpu, addr, val),
            Instruction::TXA => txa(cpu, addr, val),
            Instruction::TXS => txs(cpu, addr, val),
            Instruction::TYA => tya(cpu, addr, val),
            _ => panic!("unsupported instruction {:?}", *self),
        }
    }
}

fn adc(cpu: &mut CPU, _: u16, val: u8) {
    let n = (val as u16) + (cpu.a as u16) + (cpu.c as u16);

    let a = (n & 0xff) as u8;
    cpu.update_sz(a);

    cpu.c = n > 0xff;

    // I took this from the NesDev forums.
    // It's only concerned with the 8th bit, which indicates the sign of each
    // value. The overflow bit is set if adding two positive numbers results
    // in a negative, or if adding two negative numbers results in a positive.
    cpu.v = ((cpu.a ^ val) & 0x80 == 0) && ((cpu.a ^ n as u8) & 0x80 > 0);

    cpu.a = a;
}

fn and(cpu: &mut CPU, _: u16, val: u8) {
    cpu.a &= val;
    let a = cpu.a;
    cpu.update_sz(a);
}

fn asl(cpu: &mut CPU, addr: u16, val: u8, addr_mode: &AddressingMode) {
    cpu.c = val & 0x80 != 0;
    let n = (val << 1) & 0xff;

    match *addr_mode {
        AddressingMode::Accumulator => { cpu.a = n; },
        _ => { cpu.mem.write(addr, n).expect("ASL failed"); }
    };

    cpu.update_sz(n);
}

fn bcc(cpu: &mut CPU, addr: u16, _: u8) {
    if !cpu.c {
        let pc = cpu.pc;
        cpu.add_branch_cycles(pc, addr);
        cpu.pc = addr;
    }
}

fn bcs(cpu: &mut CPU, addr: u16, _: u8) {
    if cpu.c {
        let pc = cpu.pc;
        cpu.add_branch_cycles(pc, addr);
        cpu.pc = addr;
    }
}

fn beq(cpu: &mut CPU, addr: u16, _: u8) {
    if cpu.z {
        let pc = cpu.pc;
        cpu.add_branch_cycles(pc, addr);
        cpu.pc = addr;
    }
}

fn bit(cpu: &mut CPU, _: u16, val: u8) {
    cpu.s = val & 0x80 != 0;
    cpu.v = (val >> 0x06 & 0x01) == 1;
    let f = cpu.a & val;
    cpu.z = f == 0;
}

fn bmi(cpu: &mut CPU, addr: u16, _: u8) {
    if cpu.s {
        let pc = cpu.pc;
        cpu.add_branch_cycles(pc, addr);
        cpu.pc = addr;
    }
}

fn bne(cpu: &mut CPU, addr: u16, _: u8) {
    if !cpu.z {
        let pc = cpu.pc;
        cpu.add_branch_cycles(pc, addr);
        cpu.pc = addr;
    }
}

fn bpl(cpu: &mut CPU, addr: u16, _: u8) {
    if !cpu.s {
        let pc = cpu.pc;
        cpu.add_branch_cycles(pc, addr);
        cpu.pc = addr;
    }
}

fn bvc(cpu: &mut CPU, addr: u16, _: u8) {
    if !cpu.v {
        let pc = cpu.pc;
        cpu.add_branch_cycles(pc, addr);
        cpu.pc = addr;
    }
}

fn bvs(cpu: &mut CPU, addr: u16, _: u8) {
    if cpu.v {
        let pc = cpu.pc;
        cpu.add_branch_cycles(pc, addr);
        cpu.pc = addr;
    }
}

fn clc(cpu: &mut CPU, _: u16, _: u8) {
    cpu.c = false;
}

fn cld(cpu: &mut CPU, _: u16, _: u8) {
    cpu.d = false;
}

fn clv(cpu: &mut CPU, _: u16, _: u8) {
    cpu.v = false;
}

fn cmp(cpu: &mut CPU, _: u16, val: u8) {
    let n = cpu.a.wrapping_sub(val);
    cpu.c = cpu.a >= val;
    cpu.update_sz(n);
}

fn cpx(cpu: &mut CPU, _: u16, val: u8) {
    let n = cpu.x.wrapping_sub(val);
    cpu.update_sz(n);
    cpu.c = cpu.x >= val;
}

fn cpy(cpu: &mut CPU, _: u16, val: u8) {
    let n = cpu.y.wrapping_sub(val);
    cpu.update_sz(n);
    cpu.c = cpu.y >= val;
}

fn dec(cpu: &mut CPU, addr: u16, val: u8) {
    let n = val.wrapping_sub(1);
    cpu.update_sz(n);
    cpu.mem.write(addr, n)
        .expect("DEC failed");
}

fn dex(cpu: &mut CPU, _: u16, _: u8) {
    let n = cpu.x.wrapping_sub(1);
    cpu.x = n;
    cpu.update_sz(n);
}

fn dey(cpu: &mut CPU, _: u16, _: u8) {
    let n = cpu.y.wrapping_sub(1);
    cpu.y = n;
    cpu.update_sz(n);
}

fn eor(cpu: &mut CPU, _: u16, val: u8) {
    let val = val ^ cpu.a;
    cpu.a = val;
    cpu.update_sz(val);
}

fn inc(cpu: &mut CPU, addr: u16, val: u8) {
    let n = val.wrapping_add(1);
    cpu.mem.write(addr, n)
        .expect("INC failed");
    cpu.update_sz(n);
}

fn inx(cpu: &mut CPU, _: u16, _: u8) {
    let n = cpu.x.wrapping_add(1);
    cpu.x = n;
    cpu.update_sz(n);
}

fn iny(cpu: &mut CPU, _: u16, _: u8) {
    let n = cpu.y.wrapping_add(1);
    cpu.y = n;
    cpu.update_sz(n);
}

fn jmp(cpu: &mut CPU, addr: u16, _: u8) {
    cpu.pc = addr;
}

fn jsr(cpu: &mut CPU, addr: u16, _: u8) {
    let retaddr = cpu.pc - 1;
    cpu.stack_push16(retaddr);
    cpu.pc = addr;
}

fn lda(cpu: &mut CPU, _: u16, val: u8) {
    cpu.a = val;
    cpu.update_sz(val);
}

fn ldx(cpu: &mut CPU, _: u16, val: u8) {
    cpu.x = val;
    cpu.update_sz(val);
}

fn ldy(cpu: &mut CPU, _: u16, val: u8) {
    cpu.y = val;
    cpu.update_sz(val);
}

fn lsr(cpu: &mut CPU, addr: u16, val: u8, addr_mode: &AddressingMode) {
    cpu.c = val & 0x01 == 1;
    let n = val >> 1;
    cpu.update_sz(n);

    match *addr_mode {
        AddressingMode::Accumulator => { cpu.a = n; },
        _ => { cpu.mem.write(addr, n).expect("LSR failed"); }
    };
}

fn nop(_: &mut CPU, _: u16, _: u8) { }

fn ora(cpu: &mut CPU, _: u16, val: u8) {
    let na = cpu.a | val;
    cpu.a = na;
    cpu.update_sz(na);
}

fn pha(cpu: &mut CPU, _: u16, _: u8) {
    let a = cpu.a;
    cpu.stack_push8(a);
}

fn php(cpu: &mut CPU, _: u16, _: u8) {
    // https://wiki.nesdev.com/w/index.php/CPU_status_flag_behavior
    // According to the above link, the PHP instruction sets bits 4 and 5 on
    // the value it pushes onto the stack.
    // The PLP call later will ignore these bits.
    let flags = cpu.flags() | 0x10;
    cpu.stack_push8(flags);
}

fn pla(cpu: &mut CPU, _: u16, _: u8) {
    let rv = cpu.stack_pop8();
    cpu.a = rv;
    cpu.update_sz(rv);
}

fn plp(cpu: &mut CPU, _: u16, _: u8) {
    let p = cpu.stack_pop8() & 0xef | 0x20;
    cpu.set_flags(p);
}

fn rol(cpu: &mut CPU, addr: u16, val: u8, addr_mode: &AddressingMode) {
    let c = cpu.c;
    cpu.c = val & 0x80 != 0;
    let n = (val << 1) | (c as u8);
    cpu.update_sz(n);

    match *addr_mode {
        AddressingMode::Accumulator => { cpu.a = n; },
        _ => { cpu.mem.write(addr, n).expect("ROR failed"); }
    };
}

fn ror(cpu: &mut CPU, addr: u16, val: u8, addr_mode: &AddressingMode) {
    let c = cpu.c;
    cpu.c = val & 0x01 == 1;
    let n = (val >> 1) | ((c as u8) << 7);
    cpu.update_sz(n);

    match *addr_mode {
        AddressingMode::Accumulator => { cpu.a = n; },
        _ => { cpu.mem.write(addr, n).expect("ROR failed"); }
    };
}

fn rti(cpu: &mut CPU, _: u16, _: u8) {
    let flags = cpu.stack_pop8() & 0xef | 0x20;
    cpu.set_flags(flags);

    let retaddr = cpu.stack_pop16();
    cpu.pc = retaddr;
}

fn rts(cpu: &mut CPU, _: u16, _: u8) {
    let retaddr = cpu.stack_pop16();
    cpu.pc = retaddr + 1;
}

fn sbc(cpu: &mut CPU, _: u16, val: u8) {
    let n: i8 = (cpu.a as i8)
        .wrapping_sub(val as i8)
        .wrapping_sub(1 - cpu.c as i8) ;

    let a = n as u8;
    cpu.update_sz(a);
    cpu.c = n >= 0;
    cpu.v = ((cpu.a ^ val) & 0x80 > 0) && ((cpu.a ^ n as u8) & 0x80 > 0);
    cpu.a = a;
}

fn sec(cpu: &mut CPU, _: u16, _: u8) {
    cpu.c = true;
}

fn sed(cpu: &mut CPU, _: u16, _: u8) {
    cpu.d = true;
}

fn sei(cpu: &mut CPU, _: u16, _: u8) {
    cpu.i = true;
}

fn sta(cpu: &mut CPU, addr: u16, _: u8) {
    cpu.mem.write(addr, cpu.a)
        .expect("STA failed");
}

fn stx(cpu: &mut CPU, addr: u16, _: u8) {
    cpu.mem.write(addr, cpu.x)
        .expect("STX failed");
}

fn sty(cpu: &mut CPU, addr: u16, _: u8) {
    cpu.mem.write(addr, cpu.y)
        .expect("STY failed");
}

fn tax(cpu: &mut CPU, _: u16, _: u8) {
    let n = cpu.a;
    cpu.x = n;
    cpu.update_sz(n);
}

fn tay(cpu: &mut CPU, _: u16, _: u8) {
    let n = cpu.a;
    cpu.y = n;
    cpu.update_sz(n);
}

fn tsx(cpu: &mut CPU, _: u16, _: u8) {
    let s = cpu.sp;
    cpu.update_sz(s);
    cpu.x = s;
}

fn txa(cpu: &mut CPU, _: u16, _: u8) {
    let n = cpu.x;
    cpu.a = n;
    cpu.update_sz(n);
}

fn txs(cpu: &mut CPU, _: u16, _: u8) {
    cpu.sp = cpu.x;
}

fn tya(cpu: &mut CPU, _: u16, _: u8) {
    let n = cpu.y;
    cpu.a = n;
    cpu.update_sz(n);
}
