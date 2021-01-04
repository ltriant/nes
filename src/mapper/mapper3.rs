use std::io::{Read, Write};
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
    sram: [u8; 0x2000],

    chr_bank: u8,

    mirror_mode: MirrorMode,
}

impl Mapper3 {
    pub fn new_mapper(rom: Vec<u8>, vrom: Vec<u8>, mirror_mode: u8) -> Self {
        Self {
            chr_rom: vrom,
            prg_rom: rom,
            sram: [0; 0x2000],

            chr_bank: 0,

            mirror_mode: MirrorMode::from_hv01(mirror_mode),
        }
    }
}

impl Mapper for Mapper3 {
    fn mirror_mode(&self) -> &MirrorMode {
        &self.mirror_mode
    }

    fn read(&mut self, address: u16) -> u8 {
        match address {
            // CHR-ROM
            0x0000 ..= 0x1fff => {
                let bank = self.chr_bank as usize;
                let index = (CHR_BANK_SIZE * bank) | address as usize & 0x1fff;
                self.chr_rom[index]
            },

            // SRAM
            0x6000 ..= 0x7fff => self.sram[address as usize - 0x6000],

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

            // SRAM
            0x6000 ..= 0x7fff => { self.sram[address as usize - 0x6000] = val },

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
        output.write(&self.sram)?;
        serde::encode_u8(output, self.chr_bank)?;
        Ok(())
    }

    fn load(&mut self, input: &mut File) -> io::Result<()> {
        self.chr_rom = serde::decode_vec(input)?;
        self.prg_rom = serde::decode_vec(input)?;
        input.read(&mut self.sram)?;
        self.chr_bank = serde::decode_u8(input)?;
        Ok(())
    }
}
