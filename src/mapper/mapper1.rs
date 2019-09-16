use crate::mapper::Mapper;
use crate::mapper::MirrorMode;

//
// SxROM (mapper 1)
//
pub struct Mapper1 {
    chr_rom: Vec<u8>,
    prg_rom: Vec<u8>,
    sram: [u8; 0x2000],

    // Registers
    control: u8,
    chr_bank0: u8,
    chr_bank1: u8,
    prg_bank: u8,

    shift_register: u8,
    write_count: u8,

    // The number of PRG-ROM banks in this cartridge
    n_banks: usize,

    pub mirror_mode: MirrorMode,
}

impl Mapper1 {
    pub fn new_mapper(rom: Vec<u8>, vrom: Vec<u8>, mirror_mode: u8, n_prg_banks: usize) -> Self {
        Self {
            chr_rom: vrom,
            prg_rom: rom,
            sram: [0; 0x2000],

            control: (3 << 2),
            chr_bank0: 0,
            chr_bank1: 0,
            prg_bank: 0,

            shift_register: 0,
            write_count: 0,
            n_banks: n_prg_banks,

            mirror_mode: MirrorMode::from(mirror_mode),
        }
    }

    fn load_register(&mut self, address: u16, val: u8) -> Result<u8, String> {
        if val & 0x80 == 0x80 {
            self.shift_register = 0;
            self.control = 3 << 2;
            self.write_count = 0;
        }
        else {
            self.shift_register |= (val & 1) << (self.write_count as usize);
            self.write_count += 1;

            if self.write_count == 5 {
                self.write_count = 0;
                self.write_register(address, self.shift_register);
                self.shift_register = 0;
            }
        }

        Ok(0)
    }

    fn prg_mode(&self) -> u8 {
        (self.control >> 2) & 3
    }

    fn chr_mode(&self) -> u8 {
        (self.control >> 4) & 1
    }

    fn write_register(&mut self, address: u16, val: u8) {
        match address {
            0x0000 ..= 0x9fff => {
                self.control = val;
            },
            0xa000 ..= 0xbfff => {
                self.chr_bank0 = val & 0b1_1111;
            },
            0xc000 ..= 0xdfff => {
                self.chr_bank1 = val & 0b1_1111;
            },
            0xe000 ..= 0xffff => {
                self.prg_bank = val & 0b1111;
            },
        }
    }
}

impl Mapper for Mapper1 {
    fn mirror_mode(&self) -> &MirrorMode {
        &self.mirror_mode
    }

    fn read(&mut self, address: u16) -> Result<u8, String> {
        match address {
            // CHR-ROM
            0x0000 ..= 0x0fff => {
                let bank = match self.chr_mode() {
                    0 => self.chr_bank0,
                    1 => self.chr_bank0,
                    _ => panic!("bad chr_mode"),
                } as usize;

                let index = (4096 * bank) | (address as usize & 0x3fff);
                Ok(self.chr_rom[index])
            },
            0x1000 ..= 0x1fff => {
                let bank = match self.chr_mode() {
                    0 => self.chr_bank0 + 1,
                    1 => self.chr_bank1,
                    _ => panic!("bad chr_mode"),
                } as usize;

                let index = (4096 * bank) | ((address as usize - 0x1000) & 0x3fff);
                Ok(self.chr_rom[index])
            },

            // SRAM
            0x6000 ..= 0x7fff => {
                Ok(self.sram[address as usize - 0x6000])
            },

            // PRG-ROM
            0x8000 ..= 0xbfff => {
                let bank = match self.prg_mode() {
                    0 | 1 => self.prg_bank as usize & 0xfe,
                    2     => 0,
                    3     => self.prg_bank as usize,
                    _     => panic!("bad prg_mode"),
                };
                let index = (16384 * bank) | (address as usize & 0x3fff);

                let val = self.prg_rom[index];
                Ok(val)
            },
            0xc000 ..= 0xffff => {
                let bank = match self.prg_mode() {
                    0 | 1 => (self.prg_bank as usize & 0xfe) | 1,
                    2     => self.prg_bank as usize,
                    3     => self.n_banks - 1,
                    _     => panic!("bad prg_mode"),
                };
                let index = (16384 * bank) | (address as usize & 0x3fff);

                let val = self.prg_rom[index];
                Ok(val)
            },

            _ => Ok(0),
        }
    }

    fn write(&mut self, address: u16, val: u8) -> Result<u8, String> {
        match address {
            // CHR-ROM
            0x0000 ..= 0x0fff => {
                let bank = match self.chr_mode() {
                    0 => self.chr_bank0,
                    1 => self.chr_bank0,
                    _ => panic!("bad chr_mode"),
                } as usize;

                let index = (4096 * bank) | address as usize;
                self.chr_rom[index] = val;
                Ok(val)
            },
            0x1000 ..= 0x1fff => {
                let bank = match self.chr_mode() {
                    0 => self.chr_bank0 + 1,
                    1 => self.chr_bank1,
                    _ => panic!("bad chr_mode"),
                } as usize;

                let index = (4096 * bank) | (address as usize - 0x1000);
                self.chr_rom[index] = val;
                Ok(val)
            },

            // SRAM
            0x6000 ..= 0x7fff => {
                self.sram[address as usize - 0x6000] = val;
                Ok(val)
            },

            // PRG-ROM
            0x8000 ..= 0xffff => self.load_register(address, val),

            _ => Ok(0),
        }
    }
}
