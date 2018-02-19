mod cpu;
mod mem;

use self::cpu::CPU;
use self::mem::Memory;

pub struct Console {
    cpu: CPU,
    mem: Memory,

    // TODO PPU, APU, controllers, etc
}

impl Console {
    pub fn new_nes_console() -> Console {
        Console {
            cpu: CPU::new_nes_cpu(),
            mem: Memory::new_nes_mem(),
        }
    }
}
