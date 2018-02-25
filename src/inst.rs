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
    BCS
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
            _ => panic!("unsupported instruction"),
        }
    }
}

fn jmp(cpu: &mut CPU, (addr, _): (u16, u8)) {
    cpu.pc = addr;
}

fn ldx(cpu: &mut CPU, (_, val): (u16, u8)) {
    cpu.x = val;
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

fn bcs(cpu: &mut CPU, (_, _): (u16, u8)) {
    // TODO
    if cpu.c {
    }
    panic!("BCS");
}
