use cpu::CPU;

#[derive(Debug)]
pub enum AddressingMode {
    None,
    Immediate,
    Absolute,
    Implied,
    Accumulator,
    AbsoluteX,
    AbsoluteY,
    ZeroPageIndexed,
    ZeroPageAbsoluteX,
    ZeroPageAbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
    Relative,
}

fn pages_differ(addr_a: u16, addr_b: u16) -> bool {
    (addr_a & 0xff00) != (addr_b & 0xff00)
}

impl AddressingMode {
    pub fn n_bytes(&self) -> Result<usize, ()> {
        match *self {
              AddressingMode::Implied
            | AddressingMode::Accumulator => Ok(1),

              AddressingMode::Immediate
            | AddressingMode::ZeroPageIndexed
            | AddressingMode::Relative
            | AddressingMode::ZeroPageAbsoluteX
            | AddressingMode::ZeroPageAbsoluteY
            | AddressingMode::IndexedIndirect
            | AddressingMode::IndirectIndexed => Ok(2),

              AddressingMode::Absolute
            | AddressingMode::AbsoluteX
            | AddressingMode::AbsoluteY
            | AddressingMode::Indirect => Ok(3),

            _ => Err(()),
        }
    }

    pub fn get_bytes(&self, cpu: &CPU) -> Vec<u8> {
        let n_bytes = self.n_bytes().unwrap() as u16;
        (0 .. n_bytes).map(|n| cpu.mem.read(cpu.pc + n).unwrap())
            .collect::<Vec<_>>()
    }

    pub fn get_data(&self, cpu: &CPU) -> Result<(u16, u8, bool), ()> {
        // At this point, cpu.pc points to the next instruction.
        let pc = cpu.pc - self.n_bytes().unwrap() as u16;

        match *self {
            AddressingMode::Immediate => {
                let addr = pc + 1;
                let val = cpu.mem.read(addr)
                    .expect("Immediate val");
                Ok((addr, val, false))
            },
            AddressingMode::Absolute => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("Absolute arg 1") as u16;
                let hi = cpu.mem.read(pc + 2)
                    .expect("Absolute arg 2") as u16;
                let addr = (hi << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("Absolute addr");
                Ok((addr, val, false))
            },
            AddressingMode::Implied => Ok((0, 0, false)),
            AddressingMode::Accumulator => Ok((0, cpu.a, false)),
            AddressingMode::ZeroPageIndexed => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("ZeroPageIndexed arg") as u16;
                let addr = (0x00 << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("ZeroPageIndexed addr");
                Ok((addr, val, false))
            },
            AddressingMode::Relative => {
                let offset = cpu.mem.read(pc + 1)
                    .expect("Relative arg") as u16;

                let is_neg = (offset as i16) < 0;
                if is_neg {
                    panic!("negatory");
                }

                // TODO negative offset?

                // NOTE This has to be based off the current program counter,
                // _after_ it has been advanced, but before the instruction is
                // being executed. I don't know why though?
                Ok((cpu.pc + offset, 0, false))
            },
            AddressingMode::AbsoluteX => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("AbsoluteX arg 1") as u16;
                let hi = cpu.mem.read(pc + 2)
                    .expect("AbsoluteX arg 2") as u16;
                let addr = (hi << 8) | lo;
                let n_addr = addr.wrapping_add(cpu.x as u16);
                let val = cpu.mem.read(n_addr)
                    .expect("AbsoluteX addr");
                Ok((n_addr, val, pages_differ(addr, n_addr)))
            },
            AddressingMode::AbsoluteY => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("AbsoluteY arg 1") as u16;
                let hi = cpu.mem.read(pc + 2)
                    .expect("AbsoluteY arg 2") as u16;
                let addr = (hi << 8) | lo;
                let n_addr = addr.wrapping_add(cpu.y as u16);
                let val = cpu.mem.read(n_addr)
                    .expect("AbsoluteY addr");
                Ok((n_addr, val, pages_differ(addr, n_addr)))
            },
            AddressingMode::Indirect => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("Indirect arg 1") as u16;
                let hi = cpu.mem.read(pc + 2)
                    .expect("Indirect arg 2") as u16;
                let addr = (hi << 8) | lo;

                let lo = cpu.mem.read(addr)
                    .expect("Indirect addr 1") as u16;

                let hi =
                    if addr & 0xff == 0xff {
                        cpu.mem.read(addr & 0xff00)
                            .expect("Indirect addr 2 bug") as u16
                    }
                    else {
                        cpu.mem.read(addr + 1)
                            .expect("Indirect addr 2") as u16
                    };

                let addr = (hi << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("Indirect addr val");

                Ok((addr, val, false))
            }
            AddressingMode::ZeroPageAbsoluteX => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("ZeroPageAbsoluteX arg 1") as u16;
                let addr = (0 << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("ZeroPageAbsoluteX addr");
                Ok((0, val.wrapping_add(cpu.x), false))
            },
            AddressingMode::ZeroPageAbsoluteY => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("ZeroPageAbsoluteY arg 1") as u16;
                let addr = (0 << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("ZeroPageAbsoluteY addr");
                Ok((0, val.wrapping_add(cpu.y), false))
            },
            AddressingMode::IndexedIndirect => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("IndexedIndirect addr");
                let addr = lo.wrapping_add(cpu.x) as u16;

                let lo = cpu.mem.read(addr)
                    .expect("IndexedIndirect lo val") as u16;

                let hi =
                    if addr & 0xff == 0xff {
                        cpu.mem.read(addr & 0xff00)
                            .expect("IndexedIndirect hi val bug") as u16
                    }
                    else {
                        cpu.mem.read(addr + 1)
                            .expect("IndexedIndirect hi val") as u16
                    };

                let addr = (hi << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("IndexedIndirect val");
                Ok((addr, val, false))
            },
            AddressingMode::IndirectIndexed => {
                let addr = cpu.mem.read(pc + 1)
                    .expect("IndirectIndexed addr") as u16;

                let lo = cpu.mem.read(addr)
                    .expect("IndirectIndexed lo val") as u16;

                let hi =
                    if addr & 0xff == 0xff {
                        cpu.mem.read(addr & 0xff00)
                            .expect("IndexedIndirect hi val bug") as u16
                    }
                    else {
                        cpu.mem.read(addr + 1)
                            .expect("IndexedIndirect hi val") as u16
                    };

                let addr = (hi << 8) | lo;
                let n_addr = addr.wrapping_add(cpu.y as u16);

                let val = cpu.mem.read(n_addr)
                    .expect("IndexedIndirect val");

                Ok((n_addr, val, pages_differ(addr, n_addr)))
            },
            _ => Err(())
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
