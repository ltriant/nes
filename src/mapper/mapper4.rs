use std::io;
use std::io::{Read, Write};
use std::fs::File;

use crate::mapper::Mapper;
use crate::mapper::MirrorMode;
use crate::serde;

const PRG_BANK_SIZE: usize = 8192;
const CHR_BANK_SIZE: usize = 1024;

//
// MMC3/TxROM (mapper 4)
//
pub struct Mapper4 {
    chr_rom: Vec<u8>,
    prg_rom: Vec<u8>,
    sram: [u8; 0x2000],

    mirror_mode: MirrorMode,

    n_prg_banks: usize,

    regs: [usize; 8],
    index: usize,
    chr_mode: bool,
    prg_mode: bool,

    irq_counter: u8,
    irq_period: u8,
    irq_enabled: bool,
    irq_flag: bool,
}

impl Mapper4 {
    pub fn new_mapper(rom: Vec<u8>, vrom: Vec<u8>, mirror_mode: u8) -> Self {
        let n_banks = rom.len() / PRG_BANK_SIZE;

        Self {
            chr_rom: vrom,
            prg_rom: rom,
            sram: [0; 0x2000],

            mirror_mode: MirrorMode::from(mirror_mode),
            
            n_prg_banks: n_banks,

            regs: [0; 8],
            index: 0,
            chr_mode: false,
            prg_mode: false,

            irq_counter: 0,
            irq_period: 0,
            irq_enabled: false,
            irq_flag: false,
        }
    }
}

impl Mapper for Mapper4 {
    fn mirror_mode(&self) -> &MirrorMode {
        &self.mirror_mode
    }

    fn read(&mut self, address: u16) -> u8 {
        match address {
            // CHR-ROM
            0x0000 ..= 0x1fff => {
                let bank = match (address, self.chr_mode) {
                    (0x0000 ..= 0x03ff, false) => self.regs[0] & 0xfe,
                    (0x0000 ..= 0x03ff, true)  => self.regs[2],
                    (0x0400 ..= 0x07ff, false) => self.regs[0] | 0x01,
                    (0x0400 ..= 0x07ff, true)  => self.regs[3],
                    (0x0800 ..= 0x0bff, false) => self.regs[1] & 0xfe,
                    (0x0800 ..= 0x0bff, true)  => self.regs[4],
                    (0x0c00 ..= 0x0fff, false) => self.regs[1] | 0x01,
                    (0x0c00 ..= 0x0fff, true)  => self.regs[5],
                    (0x1000 ..= 0x13ff, false) => self.regs[2],
                    (0x1000 ..= 0x13ff, true)  => self.regs[0] & 0xfe,
                    (0x1400 ..= 0x17ff, false) => self.regs[3],
                    (0x1400 ..= 0x17ff, true)  => self.regs[0] | 0x01,
                    (0x1800 ..= 0x1bff, false) => self.regs[4],
                    (0x1800 ..= 0x1bff, true)  => self.regs[1] & 0xfe,
                    (0x1c00 ..= 0x1fff, false) => self.regs[5],
                    (0x1c00 ..= 0x1fff, true)  => self.regs[1] | 0x01,
                    _ => panic!("should not happen ever"),
                };

                let offset = address as usize % 0x0400;
                let index = ((CHR_BANK_SIZE * bank) | offset) % self.chr_rom.len();
                self.chr_rom[index]
            },

            // SRAM
            0x6000 ..= 0x7fff => self.sram[address as usize - 0x6000],

            // PRG-ROM
            0x8000 ..= 0x9fff => {
                let bank = if self.prg_mode {
                    self.n_prg_banks - 2
                }
                else {
                    self.regs[6]
                };

                let offset = address as usize & 0x1fff;
                let index = ((PRG_BANK_SIZE * bank) | offset) % self.prg_rom.len();
                self.prg_rom[index]
            },
            0xa000 ..= 0xbfff => {
                let bank = self.regs[7];
                let offset = address as usize & 0x1fff;
                let index = ((PRG_BANK_SIZE * bank) | offset) % self.prg_rom.len();
                self.prg_rom[index]
            },
            0xc000 ..= 0xdfff => {
                let bank = if self.prg_mode {
                    self.regs[6]
                }
                else {
                    self.n_prg_banks - 2
                };

                let offset = address as usize & 0x1fff;
                let index = ((PRG_BANK_SIZE * bank) | offset) % self.prg_rom.len();
                self.prg_rom[index]
            },
            0xe000 ..= 0xffff => {
                let bank = self.n_prg_banks - 1;
                let offset = address as usize & 0x1fff;
                let index = ((PRG_BANK_SIZE * bank) | offset) % self.prg_rom.len();
                self.prg_rom[index]
            },

            _ => 0,
        }
    }

    fn write(&mut self, address: u16, val: u8) {
        let even = address & 1 == 0;

        match address {
            // CHR-ROM
            0x0000 ..= 0x1fff => { self.chr_rom[address as usize] = val },

            // SRAM
            0x6000 ..= 0x7fff => { self.sram[address as usize - 0x6000] = val },

            // PRG-ROM
            0x8000 ..= 0x9fff => {
                if even {
                    // Bank select
                    self.index = val as usize & 0x07;
                    self.prg_mode = val & 0x40 != 0;
                    self.chr_mode = val & 0x80 != 0;
                }
                else {
                    // Bank data
                    self.regs[self.index] = val as usize;
                }
            },
            0xa000 ..= 0xbfff => {
                if even {
                    self.mirror_mode = if val & 1 == 0 {
                        MirrorMode::Vertical
                    }
                    else {
                        MirrorMode::Horizontal
                    };
                }
                else {
                    // PRG-RAM protect
                    //
                    // Though these bits are functional on the MMC3, their main
                    // purpose is to write-protect save RAM during power-off.
                    // Many emulators choose not to implement them as part of
                    // iNES Mapper 4 to avoid an incompatibility with the MMC6.
                }
            },

            0xc000 ..= 0xdfff => {
                if even {
                    // IRQ latch
                    self.irq_period = val;
                }
                else {
                    // IRQ reload
                    self.irq_counter = 0;
                }
            },

            0xe000 ..= 0xffff => {
                if even {
                    // IRQ disable
                    self.irq_enabled = false;
                    self.irq_flag = false;
                }
                else {
                    // IRQ enable
                    self.irq_enabled = true;
                }
            },

            _ => { },
        }
    }

    fn irq_flag(&self) -> bool {
        self.irq_flag
    }

    fn signal_scanline(&mut self) {
        if self.irq_counter == 0 {
            self.irq_counter = self.irq_period;
        }
        else {
            self.irq_counter -= 1;

            if self.irq_counter == 0 && self.irq_enabled {
                self.irq_flag = true;
            }
        }
    }

    fn save(&self, output: &mut File) -> io::Result<()> {
        serde::encode_vec(output, &self.chr_rom)?;
        serde::encode_vec(output, &self.prg_rom)?;
        output.write(&self.sram)?;
        serde::encode_u8(output, self.mirror_mode as u8)?;
        serde::encode_usize(output, self.n_prg_banks)?;

        for i in 0 .. 8 {
            serde::encode_usize(output, self.regs[i])?;
        }
        serde::encode_usize(output, self.index)?;
        serde::encode_u8(output, self.chr_mode as u8)?;
        serde::encode_u8(output, self.prg_mode as u8)?;

        serde::encode_u8(output, self.irq_counter)?;
        serde::encode_u8(output, self.irq_period)?;
        serde::encode_u8(output, self.irq_enabled as u8)?;
        serde::encode_u8(output, self.irq_flag as u8)?;

        Ok(())
    }

    fn load(&mut self, input: &mut File) -> io::Result<()> {
        self.chr_rom = serde::decode_vec(input)?;
        self.prg_rom = serde::decode_vec(input)?;
        input.read(&mut self.sram)?;
        self.mirror_mode = MirrorMode::from(serde::decode_u8(input)?);
        self.n_prg_banks = serde::decode_usize(input)?;

        for i in 0 .. 8 {
            self.regs[i] = serde::decode_usize(input)?;
        }
        self.index = serde::decode_usize(input)?;
        self.chr_mode = serde::decode_u8(input)? != 0;
        self.prg_mode = serde::decode_u8(input)? != 0;

        self.irq_counter = serde::decode_u8(input)?;
        self.irq_period = serde::decode_u8(input)?;
        self.irq_enabled = serde::decode_u8(input)? != 0;
        self.irq_flag = serde::decode_u8(input)? != 0;

        Ok(())
    }
}
