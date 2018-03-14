use cpu::CPU;
use ines;

use std::fs::File;
use std::io;

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

    pub fn power_up(&mut self) {
        self.cpu.init();
        loop {
            self.cpu.step();
        }
    }

    pub fn insert_cartridge(&mut self, filename: &str) -> Result<(), ines::CartridgeError> {
        println!("loading {}", filename);
        let mut fh = File::open(filename)
            .map_err(ines::CartridgeError::IO)?;
        ines::load_file_into_memory(&mut fh, &mut self.cpu.mem)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_cartridge() {
        // TODO change all the asserts to returning errors, and start testing
    }
}
