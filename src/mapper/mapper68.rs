use std::collections::HashSet;
use std::io;
use std::io::{Read, Write};
use std::fs::File;

use crate::mapper::Mapper;
use crate::mapper::MirrorMode;
use crate::serde;

const PRG_BANK_SIZE: usize = 16384;
const CHR_BANK_SIZE: usize = 2048;
const CHR_NT_BANK_SIZE: usize = 1024;

//
// Sunsoft-4 (mapper 68)
//
pub struct Mapper68 {
    chr_rom: Vec<u8>,
    prg_rom: Vec<u8>,
    prg_ram: [u8; 0x2000],

    // Registers
    prg_bank0: u8,
    prg_bank1: u8,

    ram_enabled: bool,

    chr_bank0: u8,
    chr_bank1: u8,
    chr_bank2: u8,
    chr_bank3: u8,

    chr_nt_enabled: bool,
    chr_nt_bank0: u8,
    chr_nt_bank1: u8,

    address_maps: HashSet<std::ops::RangeInclusive<u16>>,

    mirror_mode: MirrorMode,
}

impl Mapper68 {
    pub fn new_mapper(rom: Vec<u8>,
                      vrom: Vec<u8>,
                      mirror_mode: u8)
        -> Self
    {
        let n_banks = rom.len() / PRG_BANK_SIZE;

        Self {
            chr_rom: vrom,
            prg_rom: rom,
            prg_ram: [0; 0x2000],

            prg_bank0: 0,
            prg_bank1: n_banks as u8 - 1,

            ram_enabled: false,

            chr_bank0: 0,
            chr_bank1: 0,
            chr_bank2: 0,
            chr_bank3: 0,

            chr_nt_enabled: false,
            chr_nt_bank0: 0,
            chr_nt_bank1: 0,

            address_maps: HashSet::new(),

            mirror_mode: MirrorMode::from_vh01(mirror_mode),
        }
    }

    fn nametable_mirror_address(&self, address: u16) -> usize {
        let banks = match self.mirror_mode() {
            MirrorMode::Vertical => [
                self.chr_nt_bank0,
                self.chr_nt_bank1,
                self.chr_nt_bank0,
                self.chr_nt_bank1
            ],
            MirrorMode::Horizontal => [
                self.chr_nt_bank0,
                self.chr_nt_bank0,
                self.chr_nt_bank1,
                self.chr_nt_bank1
            ],
            MirrorMode::Single0 => [
                self.chr_nt_bank0,
                self.chr_nt_bank0,
                self.chr_nt_bank0,
                self.chr_nt_bank0
            ],
            MirrorMode::Single1 => [
                self.chr_nt_bank1,
                self.chr_nt_bank1,
                self.chr_nt_bank1,
                self.chr_nt_bank1
            ],

            // Only 2 bits to select the mirroring means this shouldn't be possible
            MirrorMode::Four => unreachable!(),
        };

        let address = (address - 0x2000) % 0x1000;
        let table = address / 0x400;
        let bank = banks[table as usize] as usize;
        let offset = address as usize % 0x400;

        bank * CHR_NT_BANK_SIZE + offset
    }
}

impl Mapper for Mapper68 {
    fn mirror_mode(&self) -> &MirrorMode {
        &self.mirror_mode
    }

    fn address_maps(&self) -> &HashSet<std::ops::RangeInclusive<u16>> {
        &self.address_maps
    }

    fn read(&mut self, address: u16) -> u8 {
        match address {
            // CHR-ROM
            0x0000 ..= 0x07ff => {
                let base = self.chr_bank0 as usize * CHR_BANK_SIZE;
                let index = base | (address as usize & 0x7ff);
                self.chr_rom[index]
            },
            0x0800 ..= 0x0fff => {
                let base = self.chr_bank1 as usize * CHR_BANK_SIZE;
                let index = base | (address as usize & 0x7ff);
                self.chr_rom[index]
            },
            0x1000 ..= 0x17ff => {
                let base = self.chr_bank2 as usize * CHR_BANK_SIZE;
                let index = base | (address as usize & 0x7ff);
                self.chr_rom[index]
            },
            0x1800 ..= 0x1fff => {
                let base = self.chr_bank3 as usize * CHR_BANK_SIZE;
                let index = base | (address as usize & 0x7ff);
                self.chr_rom[index]
            },

            // Nametables
            0x2000 ..= 0x3eff => {
                if self.chr_nt_enabled {
                    let index = self.nametable_mirror_address(address);
                    self.chr_rom[index]
                } else {
                    0
                }
            },

            // PRG-RAM
            0x6000 ..= 0x7fff => {
                if self.ram_enabled {
                    self.prg_ram[address as usize & 0x1fff]
                } else {
                    0
                }
            },

            // PRG-ROM
            0x8000 ..= 0xbfff => {
                let base = self.prg_bank0 as usize * PRG_BANK_SIZE;
                let index = base | (address as usize & 0x3fff);
                self.prg_rom[index]
            },
            0xc000 ..= 0xffff => {
                let base = self.prg_bank1 as usize * PRG_BANK_SIZE;
                let index = base | (address as usize & 0x3fff);
                self.prg_rom[index]
            },

            _ => 0,
        }
    }

    fn write(&mut self, address: u16, val: u8) {
        match address {
            // PRG-RAM
            0x6000 ..= 0x7fff => {
                if self.ram_enabled {
                    self.prg_ram[address as usize & 0x1fff] = val;
                }

                // Note: The game "Nantettatte!! Baseball" uses writes to this register in a
                // different way.
                //
                // When RAM is disabled, this will actually write to a licensing circuit as a
                // form of DRM. This is currently not supported.
                //
                // See this wiki page:
                //   https://wiki.nesdev.com/w/index.php/INES_Mapper_068
                //
                // For all other games, if RAM isn't enabled, it's open bus behaviour.
            },

            0x8000 ..= 0x8fff => { self.chr_bank0 = val },
            0x9000 ..= 0x9fff => { self.chr_bank1 = val },
            0xa000 ..= 0xafff => { self.chr_bank2 = val },
            0xb000 ..= 0xbfff => { self.chr_bank3 = val },

            // Nametable registers
            // Only D6-D0 are used; D7 is ignored and treated as 1, so nametables must be in the
            // last 128 KiB of CHR ROM.
            0xc000 ..= 0xcfff => { self.chr_nt_bank0 = val | 0b1000_0000 },
            0xd000 ..= 0xdfff => { self.chr_nt_bank1 = val | 0b1000_0000 },

            0xe000 ..= 0xefff => {
                // 7654 3210
                //    |   ||
                //    |   ++- Mirroring
                //    |       0: vertical (0101); 1: horizontal (0011);
                //    |       2: 1-screen (0000); 3: 1-screen (1111)
                //    +------ Chip select for PPU $2000-$2FFF (nametables):
                //            0 for CIRAM or 1 for CHR ROM

                let nt_mirror = val & 0b0000_0011;
                self.mirror_mode = MirrorMode::from_vh01(nt_mirror);
                self.chr_nt_enabled = (val & 0b0001_0000) != 0;

                if self.chr_nt_enabled {
                    self.address_maps.insert(0x2000 ..= 0x3eff);
                }
            },

            0xf000 ..= 0xffff => {
                // 7  bit  0
                // ---- ----
                // ...E BBBB
                //    | ||||
                //    | ++++- Select 16 KiB PRG banked into $8000-$BFFF
                //    +------ 1:Enable PRG RAM = WRAM +CS2
                self.prg_bank0   =  val & 0b0000_1111;
                self.ram_enabled = (val & 0b0001_0000) != 0;

                if self.ram_enabled {
                    self.address_maps.insert(0x6000 ..= 0x7fff);
                }

                // Note: The game "Nantettatte!! Baseball" repurposes the bits of this register,
                // and as such, is not supported.
            } ,

            _ => { },
        }
    }

    fn save(&self, output: &mut File) -> io::Result<()> {
        serde::encode_vec(output, &self.chr_rom)?;
        serde::encode_vec(output, &self.prg_rom)?;
        output.write(&self.prg_ram)?;

        serde::encode_u8(output, self.prg_bank0)?;
        serde::encode_u8(output, self.prg_bank1)?;
        serde::encode_u8(output, self.ram_enabled as u8)?;

        serde::encode_u8(output, self.chr_bank0)?;
        serde::encode_u8(output, self.chr_bank1)?;
        serde::encode_u8(output, self.chr_bank2)?;
        serde::encode_u8(output, self.chr_bank3)?;

        serde::encode_u8(output, self.chr_nt_enabled as u8)?;
        serde::encode_u8(output, self.chr_nt_bank0)?;
        serde::encode_u8(output, self.chr_nt_bank1)?;

        serde::encode_u8(output, self.mirror_mode as u8)?;

        Ok(())
    }

    fn load(&mut self, input: &mut File) -> io::Result<()> {
        self.chr_rom = serde::decode_vec(input)?;
        self.prg_rom = serde::decode_vec(input)?;
        input.read(&mut self.prg_ram)?;

        self.prg_bank0 = serde::decode_u8(input)?;
        self.prg_bank1 = serde::decode_u8(input)?;
        self.ram_enabled = serde::decode_u8(input)? != 0;

        self.chr_bank0 = serde::decode_u8(input)?;
        self.chr_bank1 = serde::decode_u8(input)?;
        self.chr_bank2 = serde::decode_u8(input)?;
        self.chr_bank3 = serde::decode_u8(input)?;

        self.chr_nt_enabled = serde::decode_u8(input)? != 0;
        self.chr_nt_bank0 = serde::decode_u8(input)?;
        self.chr_nt_bank1 = serde::decode_u8(input)?;

        self.mirror_mode = MirrorMode::from_vh01(serde::decode_u8(input)?);

        Ok(())
    }
}


