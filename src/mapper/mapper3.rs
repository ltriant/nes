use std::collections::HashSet;
use std::io;
use std::fs::File;

use crate::mapper::Mapper;
use crate::mapper::MirrorMode;
use crate::serde;

const CHR_BANK_SIZE: usize = 8192;

//
// CNROM (mapper 3)
//
pub struct Mapper3 {
    chr_rom: Vec<u8>,
    prg_rom: Vec<u8>,

    chr_bank: u8,

    mirror_mode: MirrorMode,
    address_maps: HashSet<std::ops::RangeInclusive<u16>>,
}

impl Mapper3 {
    pub fn new_mapper(rom: Vec<u8>, vrom: Vec<u8>, mirror_mode: u8) -> Self {
        Self {
            chr_rom: vrom,
            prg_rom: rom,

            chr_bank: 0,

            mirror_mode: MirrorMode::from_hv01(mirror_mode),
            address_maps: vec![
                (0x0000 ..= 0x1fff), // CHR
                (0x8000 ..= 0xffff), // PRG
            ].into_iter().collect(),
        }
    }
}

impl Mapper for Mapper3 {
    fn mirror_mode(&self) -> &MirrorMode {
        &self.mirror_mode
    }

    fn address_maps(&self) -> &HashSet<std::ops::RangeInclusive<u16>> {
        &self.address_maps
    }

    fn read(&mut self, address: u16) -> u8 {
        match address {
            // CHR-ROM
            0x0000 ..= 0x1fff => {
                let bank = self.chr_bank as usize;
                let index = (CHR_BANK_SIZE * bank) | address as usize & 0x1fff;
                self.chr_rom[index]
            },

            // PRG-ROM
            0x8000 ..= 0xffff => self.prg_rom[address as usize - 0x8000],

            _ => 0,
        }
    }

    fn write(&mut self, address: u16, val: u8) {
        match address {
            // CHR-ROM
            0x0000 ..= 0x1fff => {
                let bank = self.chr_bank as usize;
                let index = (CHR_BANK_SIZE * bank) | address as usize & 0x1fff;
                self.chr_rom[index] = val;
            },

            // PRG-ROM
            0x8000 ..= 0xffff => {
                // CNROM only uses the first 2 bits, but other boards may use
                // the rest, apparently.
                self.chr_bank = val & 0b0000_0011;
            },
            _ =>  { },
        }
    }

    fn save(&self, output: &mut File) -> io::Result<()> {
        serde::encode_vec(output, &self.chr_rom)?;
        serde::encode_vec(output, &self.prg_rom)?;
        serde::encode_u8(output, self.chr_bank)?;
        Ok(())
    }

    fn load(&mut self, input: &mut File) -> io::Result<()> {
        self.chr_rom = serde::decode_vec(input)?;
        self.prg_rom = serde::decode_vec(input)?;
        self.chr_bank = serde::decode_u8(input)?;
        Ok(())
    }
}
