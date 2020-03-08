use std::io::{Read, Write};
use std::io;
use std::fs::File;

use crate::mapper::Mapper;
use crate::mapper::MirrorMode;
use crate::serde;

const PRG_BANK_SIZE: usize = 32768;

//
// AxROM (mapper 7)
//
pub struct Mapper7 {
    chr_rom: Vec<u8>,
    prg_rom: Vec<u8>,
    sram: [u8; 0x2000],
    mirror_mode: MirrorMode,

    prg_bank: u8,
}

impl Mapper7 {
    pub fn new_mapper(rom: Vec<u8>,
                      vrom: Vec<u8>,
                      mirror_mode: u8)
        -> Self
    {
        Self {
            chr_rom: vrom,
            prg_rom: rom,
            sram: [0; 0x2000],
            mirror_mode: MirrorMode::from_hv01(mirror_mode),

            prg_bank: 0,
        }
    }
}

impl Mapper for Mapper7 {
    fn mirror_mode(&self) -> &MirrorMode {
        &self.mirror_mode
    }

    fn read(&mut self, address: u16) -> u8 {
        match address {
            // CHR-ROM
            0x0000 ..= 0x1fff => self.chr_rom[address as usize & 0x1fff],

            // SRAM
            0x6000 ..= 0x7fff => self.sram[address as usize - 0x6000],

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
            // CHR-ROM
            0x0000 ..= 0x1fff => { self.chr_rom[address as usize & 0x1fff] = val },

            // SRAM
            0x6000 ..= 0x7fff => { self.sram[address as usize - 0x6000] = val },

            // PRG-ROM
            0x8000 ..= 0xffff => {
                // 7  bit  0
                // ---- ----
                // xxxM xPPP
                //    |  |||
                //    |  +++- Select 32 KB PRG ROM bank for CPU $8000-$FFFF
                //    +------ Select 1 KB VRAM page for all 4 nametables
                self.prg_bank  =  val & 0b0000_0111;
                let chr_mirror = (val & 0b0001_0000) != 0;

                self.mirror_mode = if chr_mirror {
                    MirrorMode::Single0
                } else {
                    MirrorMode::Single1
                }
            }
            _ => { },
        }
    }

    fn save(&self, output: &mut File) -> io::Result<()> {
        serde::encode_vec(output, &self.chr_rom)?;
        serde::encode_vec(output, &self.prg_rom)?;
        output.write(&self.sram)?;
        serde::encode_u8(output, self.prg_bank)?;
        serde::encode_u8(output, self.mirror_mode as u8)?;
        Ok(())
    }

    fn load(&mut self, input: &mut File) -> io::Result<()> {
        self.chr_rom = serde::decode_vec(input)?;
        self.prg_rom = serde::decode_vec(input)?;
        input.read(&mut self.sram)?;
        self.prg_bank = serde::decode_u8(input)?;
        self.mirror_mode = MirrorMode::from_hv01(serde::decode_u8(input)?);
        Ok(())
    }
}

