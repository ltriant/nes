use cpu::CPU;

#[derive(Debug)]
pub enum Opcode {
    Jump
}

impl Opcode {
    pub fn execute(&self, cpu: &CPU) {
        match *self {
            Opcode::Jump => {
                let lo = cpu.mem.read(cpu.pc+1)
                    .expect("low byte") as u16;
                let hi = cpu.mem.read(cpu.pc+2)
                    .expect("high byte") as u16;
                let addr = (hi << 8) | lo;
                //cpu.pc = addr;
            },
        }
    }

    pub fn debug_data(&self) -> (u8, &str, usize, usize) {
        match *self {
            Opcode::Jump => (0x4c, "JMP", 3, 3),
        }
    }
}
