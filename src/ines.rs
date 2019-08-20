use crate::mem::NESMemory;

use std::fs::File;
use std::io::Read;
use std::io;

const INES_MAGIC: [u8; 4] = [0x4e, 0x45, 0x53, 0x1a];

pub struct Cartridge;

#[derive(Debug)]
pub enum CartridgeError {
    IO(io::Error),
    InvalidMagic,
    // InvalidZeroes,
    UnsupportedCartridge,
    UnsupportedMapper(u8),
}

impl Cartridge {
    pub fn load_file_into_memory(fh: &mut File, mem: &mut NESMemory) -> Result<(), CartridgeError> {
        let mut header = [0; 16];
        let _ = fh.read(&mut header)
            .map_err(CartridgeError::IO)?;

        // NES^Z
        let magic = &header[0 .. 4];
        if magic != INES_MAGIC {
            return Err(CartridgeError::InvalidMagic);
        }

        // Get the number of 16KB ROM banks
        let n_rom_banks = header[4] as usize;
        debug!("16KB ROM banks: {}", n_rom_banks);

        // Get the number of 8KB VROM banks
        let n_vrom_banks = header[5] as usize;
        debug!("8KB VROM banks: {}", n_vrom_banks);

        // Get the mapper
        // TODO the low 4 bits are for things?
        let mapper_low = (header[6] & 0xf0) >> 4;

        // TODO the low 4 bits are for things?
        let mapper_high = header[7] & 0xf0 >> 4;

        let mapper = (mapper_high << 4) | mapper_low;
        debug!("mapper: {}", mapper);

        // only support mapper 0
        if mapper != 0 {
            return Err(CartridgeError::UnsupportedMapper(mapper));
        }

        // Get the number of 8KB RAM banks
        let n_ram_banks = header[8];
        debug!("8KB RAM banks: {}", n_ram_banks);

        // Get the cartridge type, 1 for PAL, anything else means NTSC
        let cartridge_type = header[9] >> 7;
        debug!("cartridge type: {}", cartridge_type);
        if cartridge_type == 1 {
            return Err(CartridgeError::UnsupportedCartridge);
        }

        // Reserved bytes, must all be zeroes
        let zeroes = &header[10 .. 16];
        if zeroes != [0, 0, 0, 0, 0, 0] {
            warn!("Header section should be full of zeroes, but contains {:?}",
                  zeroes);

            // Don't throw an error, because it doesn't seem to cause any issues
            // that actually matter. Balloon Fight won't start-up because of
            // this.
            //return Err(CartridgeError::InvalidZeroes);
        }

        if n_rom_banks > 0 {
            // Read the banks of ROM data
            let mut rom = vec![0; n_rom_banks * 16 * 1024];
            let bytes = fh.read(&mut rom)
                .map_err(CartridgeError::IO)?;
            debug!("read {} banks ({} bytes) of 16KB ROM data", n_rom_banks, bytes);

            mem.load_rom(&rom);
        }

        if n_vrom_banks > 0 {
            // Read the banks of VROM data
            let mut vrom = vec![0; n_vrom_banks * 8 * 1024];
            let bytes = fh.read(&mut vrom)
                .map_err(CartridgeError::IO)?;
            debug!("read {} banks ({} bytes) of 8KB VROM data", n_vrom_banks, bytes);

            mem.ppu.load_vrom(&vrom);
        }

        Ok(())
    }
}
