use std::io::{Read, Write};
use std::io;
use std::fs::File;

use crate::mapper::Mapper;
use crate::mapper::MirrorMode;
use crate::serde;

const PRG_BANK_SIZE: usize = 32768;
const CHR_BANK_SIZE: usize = 8192;

//
// GxROM (mapper 66)
//
pub struct Mapper66 {
    chr_rom: Vec<u8>,
    prg_rom: Vec<u8>,
    sram: [u8; 0x2000],

    // Registers
    chr_bank: u8,
    prg_bank: u8,

    // The number of PRG-ROM banks in this cartridge
    n_banks: usize,

    mirror_mode: MirrorMode,
}

impl Mapper66 {
    pub fn new_mapper(rom: Vec<u8>,
                      vrom: Vec<u8>,
                      mirror_mode: u8)
        -> Self
    {
        let n_banks = rom.len() / PRG_BANK_SIZE;

        Self {
            chr_rom: vrom,
            prg_rom: rom,
            sram: [0; 0x2000],

            chr_bank: 0,
            prg_bank: 0,

            n_banks: n_banks,

            mirror_mode: MirrorMode::from(mirror_mode),
        }
    }
}

impl Mapper for Mapper66 {
    fn mirror_mode(&self) -> &MirrorMode {
        &self.mirror_mode
    }

    fn read(&mut self, address: u16) -> u8 {
        match address {
            // CHR-ROM
            0x0000 ..= 0x1fff => {
                let index = (CHR_BANK_SIZE * self.chr_bank as usize) | address as usize;
                self.chr_rom[index]
            },

            // SRAM
            0x6000 ..= 0x7fff => {
                self.sram[address as usize - 0x6000]
            },

            // PRG-ROM
            0x8000 ..= 0xffff => {
                let bank = self.prg_bank as usize;
                let index = ((PRG_BANK_SIZE * bank) | address as usize & 0x7fff) % self.prg_rom.len();
                self.prg_rom[index]
            },

            _ => 0,
        }
    }

    fn write(&mut self, address: u16, val: u8) {
        match address {
            // SRAM
            0x6000 ..= 0x7fff => { self.sram[address as usize - 0x6000] = val },

            // PRG-ROM
            0x8000 ..= 0xffff => {
                // 7  bit  0
                // ---- ----
                // xxPP xxCC
                //   ||   ||
                //   ||   ++- Select 8 KB CHR ROM bank for PPU $0000-$1FFF
                //   ++------ Select 32 KB PRG ROM bank for CPU $8000-$FFFF
                self.chr_bank =  val & 0b0000_0011;
                self.prg_bank = (val & 0b0011_0000) >> 4;
            }
            _ => { },
        }
    }

    fn save(&self, output: &mut File) -> io::Result<()> {
        serde::encode_vec(output, &self.chr_rom)?;
        serde::encode_vec(output, &self.prg_rom)?;
        output.write(&self.sram)?;
        serde::encode_u8(output, self.chr_bank)?;
        serde::encode_u8(output, self.prg_bank)?;
        serde::encode_usize(output, self.n_banks)?;
        serde::encode_u8(output, self.mirror_mode as u8)?;
        Ok(())
    }

    fn load(&mut self, input: &mut File) -> io::Result<()> {
        self.chr_rom = serde::decode_vec(input)?;
        self.prg_rom = serde::decode_vec(input)?;
        input.read(&mut self.sram)?;
        self.chr_bank = serde::decode_u8(input)?;
        self.prg_bank = serde::decode_u8(input)?;
        self.n_banks = serde::decode_usize(input)?;
        self.mirror_mode = MirrorMode::from(serde::decode_u8(input)?);
        Ok(())
    }
}
