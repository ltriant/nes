use std::io::{Read, Write};
use std::io;
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
    sram: [u8; 0x2000],

    prg_bank1: u8,
    prg_bank2: u8,

    mirror_mode: MirrorMode,
}

impl Mapper2 {
    pub fn new_mapper(rom: Vec<u8>, vrom: Vec<u8>, mirror_mode: u8) -> Self {
        let n_banks = rom.len() / PRG_BANK_SIZE;

        Self {
            chr_rom: vrom,
            prg_rom: rom,
            sram: [0; 0x2000],

            prg_bank1: 1,
            prg_bank2: n_banks as u8 - 1,

            mirror_mode: MirrorMode::from(mirror_mode),
        }
    }
}

impl Mapper for Mapper2 {
    fn mirror_mode(&self) -> &MirrorMode {
        &self.mirror_mode
    }

    fn read(&mut self, address: u16) -> u8 {
        match address {
            // CHR-ROM
            0x0000 ..= 0x1fff => self.chr_rom[address as usize],

            // SRAM
            0x6000 ..= 0x7fff => self.sram[address as usize - 0x6000],

            // PRG-ROM
            0x8000 ..= 0xbfff => {
                let index = (self.prg_bank1 as usize * PRG_BANK_SIZE)
                          + (address as usize - 0x8000);
                self.prg_rom[index]
            },
            0xc000 ..= 0xffff => {
                let index = (self.prg_bank2 as usize * PRG_BANK_SIZE)
                          + (address as usize - 0xc000);
                self.prg_rom[index]
            },

            _ => 0,
        }
    }

    fn write(&mut self, address: u16, val: u8) {
        match address {
            // CHR-ROM
            0x0000 ..= 0x1fff => { self.chr_rom[address as usize] = val },

            // SRAM
            0x6000 ..= 0x7fff => { self.sram[address as usize - 0x6000] = val },

            // PRG-ROM
            0x8000 ..= 0xffff => { self.prg_bank1 = val & 0x0f },

            _ => { },
        }
    }

    fn save(&self, output: &mut File) -> io::Result<()> {
        serde::encode_vec(output, &self.chr_rom)?;
        serde::encode_vec(output, &self.prg_rom)?;
        output.write(&self.sram)?;
        serde::encode_u8(output, self.prg_bank1)?;
        serde::encode_u8(output, self.prg_bank2)?;
        Ok(())
    }

    fn load(&mut self, input: &mut File) -> io::Result<()> {
        self.chr_rom = serde::decode_vec(input)?;
        self.prg_rom = serde::decode_vec(input)?;
        input.read(&mut self.sram)?;
        self.prg_bank1 = serde::decode_u8(input)?;
        self.prg_bank2 = serde::decode_u8(input)?;
        Ok(())
    }
}
