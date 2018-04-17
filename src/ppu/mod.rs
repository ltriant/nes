mod ctrl;

use mem::Memory;
use ppu::ctrl::PPUCtrl;

pub struct PPU {
    ctrl: PPUCtrl,
}

impl Memory for PPU {
    fn read(&self, address: u16) -> Result<u8, String> {
        match address % 0x08 {
            0 => {
                let PPUCtrl(n) = self.ctrl;
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
            _ => panic!("bad PPU address {}", address)
        }
    }
}

impl PPU {
    pub fn new_nes_ppu() -> PPU {
        PPU {
            ctrl: PPUCtrl(0)
        }
    }

    pub fn step(&self) {}
}
