mod ctrl;
mod mask;

use mem::Memory;
use ppu::ctrl::PPUCtrl;
use ppu::mask::PPUMask;

pub struct PPU {
    ctrl: PPUCtrl,
    mask: PPUMask,
}

impl Memory for PPU {
    fn read(&self, address: u16) -> Result<u8, String> {
        match address % 0x08 {
            0 => {
                let PPUCtrl(n) = self.ctrl;
                Ok(n)
            },
            1 => {
                let PPUMask(n) = self.mask;
                Ok(n)
            },
            _ => panic!("bad PPU address {}", address)
        }
    }

    fn write(&mut self, address: u16, val: u8) -> Result<u8, String> {
        match address % 0x08 {
            0 => {
                self.ctrl = PPUCtrl(val);

                // TODO bits 0 and 1 update the X and Y scroll positions

                Ok(val)
            },
            1 => {
                self.mask = PPUMask(val);
                Ok(val)
            },
            _ => panic!("bad PPU address {}", address)
        }
    }
}

impl PPU {
    pub fn new_nes_ppu() -> PPU {
        PPU {
            ctrl: PPUCtrl(0),
            mask: PPUMask(0),
        }
    }

    pub fn step(&self) {}
}
