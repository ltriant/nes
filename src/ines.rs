use mem::Memory;

use std::fs::File;
use std::io::Read;
use std::io;

#[derive(Debug)]
pub enum CartridgeError {
    IO(io::Error),
    InvalidMagic,
    InvalidZeroes,
}

pub fn load_file_into_memory(fh: &mut File, mem: &mut Memory) -> Result<(), CartridgeError> {
    let mut header = [0; 16];
    let bytes = fh.read(&mut header)
        .map_err(CartridgeError::IO)?;

    // NES^Z
    let magic = &header[0 .. 4];
    if magic != [0x4e, 0x45, 0x53, 0x1a] {
        return Err(CartridgeError::InvalidMagic);
    }

    // Get the number of 16KB ROM banks
    let n_rom_banks = header[4] as usize;
    println!("16KB ROM banks: {}", n_rom_banks);

    // Get the number of 8KB VROM banks
    let n_vrom_banks = header[5] as usize;
    println!("8KB VROM banks: {}", n_vrom_banks);

    // Get the mapper
    // TODO the high 4 bits are for things?
    let mapper_low = header[6] & 0x0f;

    // TODO the high 4 bits are for things?
    let mapper_high = header[7] & 0x0f;

    let mapper = (mapper_high << 4) & mapper_low;
    // only support mapper 0 for now
    println!("mapper: {}", mapper);

    // Get the number of 8KB RAM banks
    let n_ram_banks = header[8];
    println!("8KB RAM banks: {}", n_ram_banks);

    // Get the cartridge type, 1 for PAL, anything else means NTSC
    let cartridge_type = header[9] >> 7;
    println!("cartridge type: {}", cartridge_type);

    // Reserved bytes, must all be zeroes
    let zeroes = &header[10 .. 16];
    if zeroes != [0, 0, 0, 0, 0, 0] {
        return Err(CartridgeError::InvalidZeroes);
    }

    if n_rom_banks > 0 {
        // Read the banks of ROM data
        let mut rom = vec![0; n_rom_banks * 16 * 1024];
        let bytes = fh.read(&mut rom)
            .map_err(CartridgeError::IO)?;
        println!("read {} banks ({} bytes) of 16KB ROM data", n_rom_banks, bytes);

        mem.load_rom(&rom);
    }

    if n_vrom_banks > 0 {
        // Read the banks of VROM data
        let mut vrom = vec![0; n_vrom_banks * 8 * 1024];
        let bytes = fh.read(&mut vrom)
            .map_err(CartridgeError::IO)?;
        println!("read {} banks ({} bytes) of 8KB VROM data", n_vrom_banks, bytes);
    }

    Ok(())
}
