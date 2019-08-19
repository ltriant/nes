use crate::mem::Memory;

pub struct PPUData {
    chr_rom: Vec<u8>,
    nametables: [u8; 0x1000],
    palette: [u8; 0x20],
}

pub const PALETTE_ADDRESSES: [u16; 8] =
    [0x3f01, 0x3f05, 0x3f09, 0x3f0d,   // these are the background palettes
     0x3f11, 0x3f15, 0x3f19, 0x3f1d];  // these are the sprite palettes

impl Memory for PPUData {
    fn read(&mut self, address: u16) -> Result<u8, String> {
        let address = address % 0x4000;
        match address {
            0x0000 ... 0x1fff => {
                Ok(self.chr_rom[address as usize])
            },
            0x2000 ... 0x2fff => {
                Ok(self.nametables[address as usize % 0x1000])
            },
            0x3000 ... 0x3eff => {
                // mirrors 0x2000 ... 0x2eff
                Ok(self.nametables[address as usize % 0x1000])
            }
            0x3f00 ... 0x3fff => {
                let mut address = address as usize % 0x20;

                match address & 0x00ff {
                    // Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of
                    // $3F00/$3F04/$3F08/$3F0C
                    0x10 | 0x14 | 0x18 | 0x1c => { address &= 0xff0f; },
                    _ => { },
                }

                Ok(self.palette[address])
            },
            _ => Err(format!("PPUData out of bounds 0x{:04X}", address))
        }
    }

    fn write(&mut self, address: u16, val: u8) -> Result<u8, String> {
        let address = address % 0x4000;
        match address {
            0 ... 0x1fff => {
                //debug!("writing to CHR-ROM");
                self.chr_rom[address as usize] = val;
                Ok(val)
            },
            0x2000 ... 0x2fff => {
                debug!("writing 0x{:02X} to nametable 0x{:04X}", val, address);
                self.nametables[address as usize - 0x2000] = val;
                Ok(val)
            },
            0x3000 ... 0x3eff => {
                debug!("writing 0x{:02X} to nametable 0x{:04X}", val, address);
                self.nametables[address as usize - 0x3000] = val;
                Ok(val)
            },
            0x3f00 ... 0x3fff => {
                debug!("writing 0x{:02X} to palette 0x{:04X}", val, address);

                let mut address = address as usize % 0x20;

                match address & 0x00ff {
                    // Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of
                    // $3F00/$3F04/$3F08/$3F0C
                    0x10 | 0x14 | 0x18 | 0x1c => { address &= 0xff0f; },
                    _ => { },
                }

                self.palette[address] = val;
                Ok(val)
            },
            _ => Err(format!("PPUData out of bounds 0x{:04X}", address))
        }
    }
}

impl PPUData {
    pub fn new_ppu_data() -> PPUData {
        PPUData {
            chr_rom: vec![],
            nametables: [0; 0x1000],
            palette: [0; 0x20],
        }
    }

    pub fn load_vrom(&mut self, data: &Vec<u8>) {
        self.chr_rom.clone_from(data);
    }
}
