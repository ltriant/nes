use std::collections::HashSet;
use std::io;
use std::io::{Read, Write};
use std::fs::File;

use crate::mapper::Mapper;
use crate::mapper::MirrorMode;
use crate::serde;

const PRG_BANK_SIZE: usize = 16384;

//
// UxROM (mapper 2)
//
pub struct Mapper2 {
    chr_rom: Vec<u8>,
    prg_rom: Vec<u8>,
    prg_ram: [u8; 0x2000],

    n_banks: usize,
    prg_bank1: u8,
    prg_bank2: u8,

    mirror_mode: MirrorMode,
    address_maps: HashSet<std::ops::RangeInclusive<u16>>,
}

impl Mapper2 {
    pub fn new_mapper(rom: Vec<u8>, vrom: Vec<u8>, mirror_mode: u8) -> Self {
        let n_banks = rom.len() / PRG_BANK_SIZE;

        Self {
            chr_rom: vrom,
            prg_rom: rom,
            prg_ram: [0; 0x2000],

            n_banks: n_banks,
            prg_bank1: 1,
            prg_bank2: n_banks as u8 - 1,

            mirror_mode: MirrorMode::from_hv01(mirror_mode),
            address_maps: vec![
                (0x0000 ..= 0x1fff), // CHR-ROM
                (0x8000 ..= 0xffff), // PRG-ROM
            ].into_iter().collect(),
        }
    }
}

impl Mapper for Mapper2 {
    fn mirror_mode(&self) -> &MirrorMode {
        &self.mirror_mode
    }

    fn address_maps(&self) -> &HashSet<std::ops::RangeInclusive<u16>> {
        &self.address_maps
    }

    fn read(&mut self, address: u16) -> u8 {
        match address {
            // CHR-ROM
            0x0000 ..= 0x1fff => self.chr_rom[address as usize],

            // PRG-RAM
            0x6000 ..= 0x7fff => self.prg_ram[address as usize & 0x1fff],

            // PRG-ROM
            0x8000 ..= 0xbfff => {
                let bank = (self.prg_bank1 as usize) * PRG_BANK_SIZE;
                let index = bank | (address as usize & 0x3fff);
                self.prg_rom[index]
            },
            0xc000 ..= 0xffff => {
                let bank = (self.prg_bank2 as usize) * PRG_BANK_SIZE;
                let index = bank | (address as usize & 0x3fff);
                self.prg_rom[index]
            },

            _ => 0,
        }
    }

    fn write(&mut self, address: u16, val: u8) {
        match address {
            // CHR-ROM
            0x0000 ..= 0x1fff => { self.chr_rom[address as usize] = val },

            // PRG-RAM
            0x6000 ..=  0x7fff => { self.prg_ram[address as usize & 0x1fff] = val },

            // PRG-ROM
            0x8000 ..= 0xffff => { self.prg_bank1 = val & (self.n_banks as u8 - 1) },

            _ => { },
        }
    }

    fn save(&self, output: &mut File) -> io::Result<()> {
        serde::encode_vec(output, &self.chr_rom)?;
        serde::encode_vec(output, &self.prg_rom)?;
        output.write(&self.prg_ram)?;
        serde::encode_u8(output, self.prg_bank1)?;
        serde::encode_u8(output, self.prg_bank2)?;
        serde::encode_usize(output, self.n_banks)?;
        Ok(())
    }

    fn load(&mut self, input: &mut File) -> io::Result<()> {
        self.chr_rom = serde::decode_vec(input)?;
        self.prg_rom = serde::decode_vec(input)?;
        input.read(&mut self.prg_ram)?;
        self.prg_bank1 = serde::decode_u8(input)?;
        self.prg_bank2 = serde::decode_u8(input)?;
        self.n_banks   = serde::decode_usize(input)?;
        Ok(())
    }
}
