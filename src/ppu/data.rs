use crate::mapper::{Mapper, MapperEmpty};
use crate::mem::Memory;

pub struct PPUData {
    pub mapper: Box<dyn Mapper>,
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
            0x0000 ..= 0x1fff => self.mapper.read(address),
            0x2000 ..= 0x3eff => {
                let address = (address - 0x2000) % 0x1000;
                let table = address / 0x0400;
                let offset = address % 0x0400;
                let index = 0x2000
                    + self.mapper.nametable_offset(table as usize) * 0x400
                    + offset as usize;
                Ok(self.nametables[index % 2048])
            }
            0x3f00 ..= 0x3fff => {
                let mut i = address as usize % 0x20;

                match i & 0x00ff {
                    // Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of
                    // $3F00/$3F04/$3F08/$3F0C
                    0x10 | 0x14 | 0x18 | 0x1c => { i &= 0xff0f; },
                    _ => { },
                }

                Ok(self.palette[i])
            },
            _ => Err(format!("PPUData out of bounds 0x{:04X}", address))
        }
    }

    fn write(&mut self, address: u16, val: u8) -> Result<u8, String> {
        let address = address % 0x4000;
        match address {
            0x0000 ..= 0x1fff => self.mapper.write(address, val),
            0x2000 ..= 0x3eff => {
                debug!("writing 0x{:02X} to nametable 0x{:04X}", val, address);
                let address = (address - 0x2000) % 0x1000;
                let table = address / 0x0400;
                let offset = address % 0x0400;
                let index = 0x2000
                    + self.mapper.nametable_offset(table as usize) * 0x400
                    + offset as usize;
                self.nametables[index % 2048] = val;
                Ok(val)
            },
            0x3f00 ..= 0x3fff => {
                debug!("writing 0x{:02X} to palette 0x{:04X}", val, address);
                let mut i = address as usize % 0x20;

                match i & 0x00ff {
                    // Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of
                    // $3F00/$3F04/$3F08/$3F0C
                    0x10 | 0x14 | 0x18 | 0x1c => { i &= 0xff0f; },
                    _ => { },
                }

                self.palette[i] = val;
                Ok(val)
            },
            _ => Err(format!("PPUData out of bounds 0x{:04X}", address))
        }
    }
}

impl PPUData {
    pub fn new_ppu_data() -> Self {
        Self {
            mapper: Box::new(MapperEmpty{}),
            nametables: [0; 0x1000],
            palette: [
                // These are the start-up palette values to pass blarrg's PPU tests
                0x09,0x01,0x00,0x01,
                0x00,0x02,0x02,0x0D,
                0x08,0x10,0x08,0x24,
                0x00,0x00,0x04,0x2C,
                0x09,0x01,0x34,0x03,
                0x00,0x04,0x00,0x14,
                0x08,0x3A,0x00,0x02,
                0x00,0x20,0x2C,0x08
            ],
        }
    }
}
