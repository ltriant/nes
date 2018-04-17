use cpu::CPU;
use mem::NESMemory;
use ppu::PPU;
use ines::{Cartridge, CartridgeError};

use std::fs::File;

pub struct Console {
    cpu: CPU,
}

impl Console {
    pub fn new_nes_console() -> Console {
        let ppu = PPU::new_nes_ppu();
        let mem = NESMemory::new_nes_mem(ppu);
        Console {
            cpu: CPU::new_nes_cpu(mem),
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
