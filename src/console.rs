use cpu::CPU;

use std::fs::File;
use std::io::Read;

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

    // TODO move iNES parsing into a separate module?
    pub fn insert_cartridge(&mut self, filename: &str) -> Result<(), &str> {
        println!("loading {}", filename);

        let mut fh = File::open(filename)
            .expect("cannot open file");

        let mut header = [0; 16];
        let bytes = fh.read(&mut header)
            .expect("cannot read catridge header");

        assert_eq!(bytes, 16);

        let magic = &header[0 .. 4];
        assert_eq!(magic, [0x4e, 0x45, 0x53, 0x1a]); // NES^Z

        // Get the number of 16KB ROM banks
        let n_rom_banks = header[4];
        println!("16KB ROM banks: {}", n_rom_banks);

        // Get the number of 8KB VROM banks
        let n_vrom_banks = header[5];
        println!("8KB VROM banks: {}", n_vrom_banks);

        // Get the mapper
        // TODO the high 4 bits are for things?
        let mapper_low = header[6] & 0x0f;

        // TODO the high 4 bits are for things?
        let mapper_high = header[7] & 0x0f;

        let mapper = (mapper_high << 4) & mapper_low;
        // only support mapper 0 for now
        assert_eq!(mapper, 0);
        println!("mapper: {}", mapper);

        // Get the number of 8KB RAM banks
        let n_ram_banks = header[8];
        println!("8KB RAM banks: {}", n_ram_banks);

        // Get the cartridge type, 1 for PAL, anything else means NTSC
        let cartridge_type = header[9] >> 7;
        println!("cartridge type: {}", cartridge_type);

        // Reserved bytes, must all be zeroes
        let zeroes = &header[10 .. 16];
        assert_eq!(zeroes, [0, 0, 0, 0, 0, 0]);

        if n_rom_banks > 0 {
            // Read the banks of ROM data
            let mut rom = vec![0; n_rom_banks as usize * 16 * 1024];
            let bytes = fh.read(&mut rom)
                .expect("cannot read ROM banks");
            println!("read {} banks ({} bytes) of 16KB ROM data", n_rom_banks, bytes);

            self.cpu.mem.load_rom(&rom);
        }

        if n_vrom_banks > 0 {
            // Read the banks of VROM data
            let mut vrom = vec![0; n_vrom_banks as usize * 8 * 1024];
            let bytes = fh.read(&mut vrom)
                .expect("cannot read VROM banks");
            println!("read {} banks ({} bytes) of 8KB VROM data", n_vrom_banks, bytes);
        }

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
