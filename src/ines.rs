use crate::mapper::Mapper;
use crate::mapper::Mapper0;
use crate::mapper::Mapper1;
use crate::mapper::Mapper2;
use crate::mapper::Mapper3;
use crate::mapper::Mapper4;
use crate::mapper::Mapper7;
use crate::mapper::Mapper34;
use crate::mapper::Mapper66;
use crate::mapper::Mapper68;
use crate::mapper::Mapper69;

use crate::mapper::MirrorMode;

use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::io;
use std::rc::Rc;

const INES_MAGIC: [u8; 4] = [0x4e, 0x45, 0x53, 0x1a];

#[derive(Debug)]
pub enum CartridgeError {
    IO(io::Error),
    InvalidMagic,
    // InvalidZeroes,
    UnsupportedCartridge,
    UnsupportedMapper(u8),
}

pub fn load_file_into_memory(fh: &mut File)
    -> Result<Rc<RefCell<Box<dyn Mapper>>>, CartridgeError>
{
    let mut header = [0; 16];
    let _ = fh.read(&mut header).map_err(CartridgeError::IO)?;

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

    let mut mirror_mode = header[6] & 0x01;
    debug!("mirroring: {}", if mirror_mode == 0 { "horizontal" } else { "vertical" });

    if header[6] & 0x08 != 0 {
        debug!("ignoring mirroring, using four-screen instead");
        mirror_mode = MirrorMode::Four as u8;
    }

    let battery_backed = (header[7] & 0x02) != 0;
    debug!("battery backed PRG-RAM: {}", if battery_backed { "yes" } else { "no" });

    // Get the mapper
    let mapper_low =  (header[6] & 0xf0) >> 4;
    let mapper_high = (header[7] & 0xf0) >> 4;
    let mapper = (mapper_high << 4) | mapper_low;
    debug!("mapper: {}", mapper);

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

    let mut rom = vec![];
    let mut vrom = vec![];

    if n_rom_banks > 0 {
        // Read the banks of ROM data
        rom = vec![0; n_rom_banks * 16 * 1024];
        let bytes = fh.read(&mut rom).map_err(CartridgeError::IO)?;
        debug!("read {} banks ({} bytes) of 16KB PRG-ROM data", n_rom_banks, bytes);
    }

    if n_vrom_banks > 0 {
        // Read the banks of VROM data
        vrom = vec![0; n_vrom_banks * 8 * 1024];
        let bytes = fh.read(&mut vrom).map_err(CartridgeError::IO)?;
        debug!("read {} banks ({} bytes) of 8KB CHR-ROM data", n_vrom_banks, bytes);
    }

    if n_vrom_banks == 0 {
        vrom = vec![0; 8192];
        debug!("making 8KB of CHR-RAM");
    }

    match mapper {
        0 => Ok(Rc::new(RefCell::new(Box::new(Mapper0::new_mapper(rom, vrom, mirror_mode))))),
        1 => Ok(Rc::new(RefCell::new(Box::new(Mapper1::new_mapper(rom, vrom, mirror_mode))))),
        2 => Ok(Rc::new(RefCell::new(Box::new(Mapper2::new_mapper(rom, vrom, mirror_mode))))),
        3 => Ok(Rc::new(RefCell::new(Box::new(Mapper3::new_mapper(rom, vrom, mirror_mode))))),
        4 => Ok(Rc::new(RefCell::new(Box::new(Mapper4::new_mapper(rom, vrom, mirror_mode))))),
        7 => Ok(Rc::new(RefCell::new(Box::new(Mapper7::new_mapper(rom, vrom, mirror_mode))))),
        34 => Ok(Rc::new(RefCell::new(Box::new(Mapper34::new_mapper(rom, vrom, mirror_mode))))),
        66 => Ok(Rc::new(RefCell::new(Box::new(Mapper66::new_mapper(rom, vrom, mirror_mode))))),
        68 => Ok(Rc::new(RefCell::new(Box::new(Mapper68::new_mapper(rom, vrom, mirror_mode))))),
        69 => Ok(Rc::new(RefCell::new(Box::new(Mapper69::new_mapper(rom, vrom, mirror_mode))))),
        _ => Err(CartridgeError::UnsupportedMapper(mapper)),
    }
}
