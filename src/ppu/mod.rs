mod ctrl;
mod mask;
mod status;
mod oam;

use mem::Memory;
use ppu::ctrl::PPUCtrl;
use ppu::mask::PPUMask;
use ppu::status::PPUStatus;
use ppu::oam::OAM;

pub struct PPU {
    ctrl: PPUCtrl,
    mask: PPUMask,
    status: PPUStatus,
    oam_addr: u8,
    oam: OAM,
}

impl Memory for PPU {
    fn read(&self, address: u16) -> Result<u8, String> {
        match address {
            0x2000 => {
                let PPUCtrl(n) = self.ctrl;
                Ok(n)
            },
            0x2001 => {
                let PPUMask(n) = self.mask;
                Ok(n)
            },
            0x2002 => {
                let PPUStatus(n) = self.status;

                // TODO reset the latch, whatever that means

                Ok(n)
            },
            0x2003 => Ok(0), // OAMADDR is write-only
            0x2004 => panic!("OAMData is unreadable... I think. Double check if this panic happens."),
            _ => panic!("bad PPU address 0x{:04X}", address)
        }
    }

    fn write(&mut self, address: u16, val: u8) -> Result<u8, String> {
        match address {
            0x2000 => {
                self.ctrl = PPUCtrl(val);

                // TODO bits 0 and 1 update the X and Y scroll positions

                Ok(val)
            },
            0x2001 => {
                self.mask = PPUMask(val);
                Ok(val)
            },
            0x2002 => Err(String::from("PPUStatus is readonly")),
            0x2003 => {
                self.oam_addr = val;
                Ok(val)
            },
            0x2004 => {
                self.oam.write(self.oam_addr as u16, val);
                self.oam_addr += 1;
                Ok(val)
            },
            _ => panic!("bad PPU address 0x{:04X}", address)
        }
    }
}

impl PPU {
    pub fn new_nes_ppu() -> PPU {
        PPU {
            ctrl: PPUCtrl(0),
            mask: PPUMask(0),
            status: PPUStatus(0),
            oam: OAM::new_nes_oam(),
            oam_addr: 0,
        }
    }

    pub fn step(&self) {}
}
