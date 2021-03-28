use std::collections::HashSet;
use std::io;
use std::fs::File;

use crate::mapper::Mapper;
use crate::mapper::MirrorMode;
use crate::serde;

const PRG_BANK_SIZE: usize = 32768;
const CHR_BANK_SIZE: usize = 4096;

#[derive(Clone, Copy, Debug)]
enum Board {
    BxROM,
    NINA001,
}

//
// BxROM/NINA-001 (mapper 34)
//
pub struct Mapper34 {
    chr_rom: Vec<u8>,
    prg_rom: Vec<u8>,
    prg_ram: [u8; 0x2000],

    board: Board,

    // Registers
    prg_bank: u8,
    chr_bank0: u8,
    chr_bank1: u8,

    mirror_mode: MirrorMode,
    address_maps: HashSet<std::ops::RangeInclusive<u16>>,
}

impl Mapper34 {
    pub fn new_mapper(rom: Vec<u8>,
                      vrom: Vec<u8>,
                      mirror_mode: u8)
        -> Self
    {
        let board = if vrom.len() <= 8192 {
            Board::BxROM
        } else {
            Board::NINA001
        };

        let mirror_mode = match board {
            Board::NINA001 => MirrorMode::Horizontal,  // hardwired
            Board::BxROM   => MirrorMode::from_hv01(mirror_mode),
        };

        debug!("board: {:?}, mirror_mode: {:?}", board, mirror_mode);

        Self {
            chr_rom: vrom,
            prg_rom: rom,
            prg_ram: [0; 0x2000],

            board: board,

            prg_bank: 0,
            chr_bank0: 0,
            chr_bank1: 0,

            mirror_mode: mirror_mode,
            address_maps: vec![
                (0x0000 ..= 0x1fff), // CHR-ROM
                (0x6000 ..= 0x7fff), // PRG-RAM
                (0x8000 ..= 0xffff), // PRG-ROM
            ].into_iter().collect(),
        }
    }
}

impl Mapper for Mapper34 {
    fn mirror_mode(&self) -> &MirrorMode {
        &self.mirror_mode
    }

    fn address_maps(&self) -> &HashSet<std::ops::RangeInclusive<u16>> {
        &self.address_maps
    }

    fn read(&mut self, address: u16) -> u8 {
        match self.board {
            Board::NINA001 => {
                match address {
                    // CHR-ROM
                    0x0000 ..= 0x0fff => {
                        let base = (self.chr_bank0 as usize) * CHR_BANK_SIZE;
                        let index = base | address as usize;
                        self.chr_rom[index]
                    },
                    0x1000 ..= 0x1fff => {
                        let base = (self.chr_bank1 as usize) * CHR_BANK_SIZE;
                        let index = base | (address as usize & 0x0fff);
                        self.chr_rom[index]
                    },

                    // PRG-RAM
                    0x6000 ..= 0x7fff => self.prg_ram[address as usize & 0x1fff],

                    // PRG-ROM
                    0x8000 ..= 0xffff => {
                        let base = (self.prg_bank as usize) * PRG_BANK_SIZE;
                        let index = base | (address as usize & 0x7fff);
                        self.prg_rom[index]
                    },

                    _ => 0,
                }
            },

            Board::BxROM => {
                match address {
                    // CHR-RAM
                    0x0000 ..= 0x1fff => self.chr_rom[address as usize],

                    // PRG-ROM
                    0x8000 ..= 0xffff => {
                        let index = (self.prg_bank as usize * PRG_BANK_SIZE)
                            | (address as usize & 0x7fff);

                        self.prg_rom[index]
                    },

                    _ => 0,
                }
            }
        }
    }

    fn write(&mut self, address: u16, val: u8) {
        match self.board {
            Board::NINA001 => {
                match address {
                    // PRG-RAM
                    0x6000 ..= 0x7ffc => { self.prg_ram[address as usize - 0x6000] = val },

                    // These registers reside "on top" of PRG RAM: each write to the register goes
                    // both to the register and to the RAM location at the same address. Thus,
                    // reading the register's address returns the last value written to the RAM,
                    // which is also the last value written to the register.

                    // $7FFD: .... ...P - Select 32 KB PRG ROM bank
                    0x7ffd => {
                        debug!("select PRG bank: {}", val);
                        self.prg_bank = val & 0b0000_0001;
                        self.prg_ram[0x1ffd] = val;
                    },

                    // $7FFE: .... CCCC - Select 4 KB CHR bank at $0000
                    0x7ffe => {
                        self.chr_bank0 = val & 0b0000_1111;
                        self.prg_ram[0x1ffe] = val;
                    },

                    // $7FFF: .... CCCC - Select 4 KB CHR bank at $1000
                    0x7fff => {
                        self.chr_bank1 = val & 0b0000_1111;
                        self.prg_ram[0x1fff] = val;
                    },

                    _ => { },
                }
            },

            Board::BxROM => {
                match address {
                    // CHR-RAM
                    0x0000 ..= 0x1fff => { self.chr_rom[address as usize] = val; },

                    // PRG-ROM
                    0x8000 ..= 0xffff => {
                        //              7  bit  0
                        //             ---------
                        // $8000-FFFF: .... ..PP - Select 32 KB PRG ROM bank
                        //
                        // Emulators can support the entire 8 bits, instead of just 2.
                        self.prg_bank = val & 0b1111_1111;
                    },

                    _ => { },
                }
            }
        }
    }

    fn save(&self, output: &mut File) -> io::Result<()> {
        serde::encode_vec(output, &self.chr_rom)?;
        serde::encode_vec(output, &self.prg_rom)?;
        serde::encode_u8(output, self.prg_bank)?;
        serde::encode_u8(output, self.chr_bank0)?;
        serde::encode_u8(output, self.chr_bank1)?;
        serde::encode_u8(output, self.mirror_mode as u8)?;
        Ok(())
    }

    fn load(&mut self, input: &mut File) -> io::Result<()> {
        self.chr_rom = serde::decode_vec(input)?;
        self.prg_rom = serde::decode_vec(input)?;
        self.prg_bank = serde::decode_u8(input)?;
        self.chr_bank0 = serde::decode_u8(input)?;
        self.chr_bank1 = serde::decode_u8(input)?;
        self.mirror_mode = MirrorMode::from_hv01(serde::decode_u8(input)?);
        Ok(())
    }
}

