use std::collections::HashSet;
use std::io::{Read, Write};
use std::io;
use std::fs::File;

use crate::mapper::Mapper;
use crate::mapper::MirrorMode;
use crate::serde;

//
// NROM (mapper 0)
//
pub struct Mapper0 {
    chr_rom: Vec<u8>,
    prg_rom: Vec<u8>,
    prg_ram: [u8; 0x2000],

    mirror_mode: MirrorMode,
    address_maps: HashSet<std::ops::RangeInclusive<u16>>,
}

impl Mapper for Mapper0 {
    fn mirror_mode(&self) -> &MirrorMode {
        &self.mirror_mode
    }

    fn address_maps(&self) -> &HashSet<std::ops::RangeInclusive<u16>> {
        &self.address_maps
    }

    fn read(&mut self, address: u16) -> u8 {
        match address {
            0x0000 ..= 0x1fff => {
                let len = self.chr_rom.len();
                self.chr_rom[address as usize % len]
            },
            0x6000 ..= 0x7fff => self.prg_ram[address as usize - 0x6000],
            0x8000 ..= 0xffff => self.prg_rom[address as usize % self.prg_rom.len()],
            _ => 0,
        }
    }

    fn write(&mut self, address: u16, val: u8) {
        match address {
            0x0000 ..= 0x1fff => {
                let len = self.chr_rom.len();
                self.chr_rom[address as usize % len] = val;
            },
            0x6000 ..= 0x7fff => {
                self.prg_ram[address as usize - 0x6000] = val;
            },
            _ => { },
        }
    }

    fn save(&self, output: &mut File) -> io::Result<()> {
        serde::encode_vec(output, &self.chr_rom)?;
        serde::encode_vec(output, &self.prg_rom)?;
        output.write(&self.prg_ram)?;
        Ok(())
    }

    fn load(&mut self, input: &mut File) -> io::Result<()> {
        self.chr_rom = serde::decode_vec(input)?;
        self.prg_rom = serde::decode_vec(input)?;
        input.read(&mut self.prg_ram)?;
        Ok(())
    }
}

impl Mapper0 {
    pub fn new_mapper(rom: Vec<u8>, vrom: Vec<u8>, mirror_mode: u8) -> Self {
        Self {
            chr_rom: vrom,
            prg_rom: rom,
            prg_ram: [0; 0x2000],
            mirror_mode: MirrorMode::from_hv01(mirror_mode),
            address_maps: vec![
                (0x0000 ..= 0x1fff), // CHR-ROM
                (0x6000 ..= 0x7fff), // PRG-RAM
                (0x8000 ..= 0xffff), // PRG-ROM
            ].into_iter().collect()
        }
    }
}
