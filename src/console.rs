use cpu::CPU;
use ines::{Cartridge, CartridgeError};

use std::fs::File;

pub struct Console {
    cpu: CPU,

    // TODO PPU, APU, controllers, etc
}

impl Console {
    pub fn new_nes_console() -> Console {
        Console {
            cpu: CPU::new_nes_cpu(),
        }
    }

    pub fn insert_cartridge(&mut self, filename: &str) -> Result<(), CartridgeError> {
        println!("loading {}", filename);
        let mut fh = File::open(filename)
            .map_err(CartridgeError::IO)?;
        let _ = Cartridge::load_file_into_memory(&mut fh, &mut self.cpu.mem)?;
        Ok(())
    }

    pub fn power_up(&mut self) {
        self.cpu.init();
        loop {
            self.cpu.step();
        }
    }
}
