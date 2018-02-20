mod cpu;

use self::cpu::CPU;

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

    // TODO move iNES parsing into a separate module?
    pub fn insert_cartridge(&mut self, filename: &str) -> Result<(), &str> {
        println!("loading {}", filename);

        let mut fh = File::open(filename)
            .expect("cannot open file");

        let mut header = [0; 4];
        let bytes = fh.read(&mut header)
            .expect("cannot read header");

        // Require an iNES format file
        assert_eq!(bytes, 4);
        assert_eq!(header, [0x4e, 0x45, 0x53, 0x1a]); // NES^Z

        // Get the number of 16KB ROM banks
        let mut n_rom_banks = [0; 1];
        let bytes = fh.read(&mut n_rom_banks)
            .expect("cannot read ROM banks");
        assert_eq!(bytes, 1);
        let n_rom_banks = n_rom_banks[0];
        println!("16KB ROM banks: {}", n_rom_banks);

        // Get the number of 8KB VROM banks
        let mut n_vrom_banks = [0; 1];
        let bytes = fh.read(&mut n_vrom_banks)
            .expect("cannot read VROM banks");
        assert_eq!(bytes, 1);
        let n_vrom_banks = n_vrom_banks[0];
        println!("8KB VROM banks: {}", n_vrom_banks);

        // Get the mapper
        let mut mapper_low = [0; 1];
        let bytes = fh.read(&mut mapper_low)
            .expect("cannot read low mapper byte");
        assert_eq!(bytes, 1);
        // TODO the high 4 bits are for things?
        let mapper_low = mapper_low[0] & 0x0f;

        let mut mapper_high = [0; 1];
        let bytes = fh.read(&mut mapper_high)
            .expect("cannot read high mapper byte");
        assert_eq!(bytes, 1);
        // TODO the high 4 bits are for things?
        let mapper_high = mapper_high[0] & 0x0f;

        let mapper = (mapper_high << 4) & mapper_low;
        // only support mapper 0 for now
        assert_eq!(mapper, 0);
        println!("mapper: {}", mapper);

        // Get the number of 8KB RAM banks
        let mut n_ram_banks = [0; 1];
        let bytes = fh.read(&mut n_ram_banks)
            .expect("cannot read RAM banks");
        assert_eq!(bytes, 1);
        let n_ram_banks = n_ram_banks[0];
        println!("8KB RAM banks: {}", n_ram_banks);

        // Get the cartridge type, 1 for PAL, anything else means NTSC
        let mut cartridge_type = [0; 1];
        let bytes = fh.read(&mut cartridge_type)
            .expect("cannot read cartridge type");
        assert_eq!(bytes, 1);
        let cartridge_type = cartridge_type[0] >> 7;
        println!("cartridge type: {}", cartridge_type);

        // Reserved bytes, must all be zeroes
        let mut zeroes = [1; 6];
        let bytes = fh.read(&mut zeroes)
            .expect("cannot read bytes 11-16");
        assert_eq!(bytes, 6);
        assert_eq!(zeroes, [0, 0, 0, 0, 0, 0]);

        // Read the banks of ROM data
        let mut rom = vec![0; n_rom_banks as usize * 16 * 1024];
        let bytes = fh.read(&mut rom)
            .expect("cannot read ROM data");
        println!("read {} banks ({} bytes) of 16KB ROM data", n_rom_banks, bytes);

        // Read the banks of VROM data
        let mut vrom = vec![0; n_vrom_banks as usize * 8 * 1024];
        let bytes = fh.read(&mut vrom)
            .expect("cannot read VROM data");
        println!("read {} banks ({} bytes) of 8KB VROM data", n_vrom_banks, bytes);

        self.cpu.mem.load_rom(&rom);

        Ok(())
    }
}
