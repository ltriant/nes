mod ctrl;
mod mask;
mod status;
mod oam;
mod scroll;
mod addr;
mod data;

use mem::Memory;
use ppu::ctrl::PPUCtrl;
use ppu::mask::PPUMask;
use ppu::status::PPUStatus;
use ppu::scroll::PPUScroll;
use ppu::addr::PPUAddr;
use ppu::oam::OAM;
use ppu::data::PPUData;

pub struct PPU {
    ctrl: PPUCtrl,
    mask: PPUMask,
    status: PPUStatus,
    oam_addr: u8,
    oam: OAM,
    scroll: PPUScroll,
    ppu_addr: PPUAddr,
    data: PPUData,

    dot: u16,
    scanline: u16,
}

impl Memory for PPU {
    fn read(&mut self, address: u16) -> Result<u8, String> {
        // The PPU registers exist from 0x2000 to 0x2007, the rest of the
        // address space is just a mirror of these first eight bytes.
        match address % 8 + 0x2000 {
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

                // reset the latch
                self.scroll.reset_latch();
                self.ppu_addr.reset_latch();

                Ok(n)
            },
            0x2003 => Ok(0), // OAMADDR is write-only
            0x2004 => panic!("OAMData is unreadable... I think. Double check if this panic happens."),
            0x2005 => Ok(0), // PPUSCROLL is write-only
            0x2006 => Ok(0), // PPUADDR is write-only
            0x2007 => {
                let rv = self.data.read(self.ppu_addr.address())?;
                self.ppu_addr.increment(self.ctrl.vram_addr_increment());
                Ok(rv)
            },
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
                self.oam.write(self.oam_addr as u16, val)?;
                self.oam_addr += 1;
                Ok(val)
            },
            0x2005 => {
                self.scroll.write(val);
                Ok(val)
            },
            0x2006 => {
                self.ppu_addr.write(val);
                Ok(val)
            },
            0x2007 => {
                let rv = self.data.write(self.ppu_addr.address(), val)?;
                self.ppu_addr.increment(self.ctrl.vram_addr_increment());
                Ok(rv)
            },
            _ => panic!("bad PPU address 0x{:04X}", address)
        }
    }
}

pub struct StepResult {
    pub vblank_nmi: bool,
}

impl PPU {
    pub fn new_nes_ppu() -> PPU {
        PPU {
            ctrl: PPUCtrl(0),
            mask: PPUMask(0),
            status: PPUStatus(0),
            oam: OAM::new_nes_oam(),
            oam_addr: 0,
            scroll: PPUScroll::new_ppu_scroll(),
            ppu_addr: PPUAddr::new_ppu_addr(),
            data: PPUData::new_ppu_data(),

            dot: 0,
            scanline: 0,
        }
    }

    pub fn load_vrom(&mut self, data: &Vec<u8>) {
        self.data.load_vrom(data);
    }

    fn rendering_enabled(&self) -> bool {
        self.mask.show_background() || self.mask.show_sprites()
    }

    pub fn step(&mut self) -> StepResult {
        // http://wiki.nesdev.com/w/index.php/PPU_rendering#Line-by-line_timing
        //
        // There are a total of 262 scanlines per frame
        //   Scanlines 0 to 239 are for display (i.e. the NES is 256 x _240_)
        //   Scanline  240 is a post-render scanline (idle)
        //   Scanlines 241 to 260 are the vblank interval
        //   Scanline  261 is a pre-render scanline (idle?)
        //
        // There are a total of 341 dots per scanline
        //   The first 256 dots are displayable (i.e. the NES is _256_ x 240)

        let mut res = StepResult{vblank_nmi: false};

        self.dot += 1;
        if self.dot == 341 {
            self.scanline += 1;
            self.dot = 0;
        }

        if self.scanline <= 239 && self.rendering_enabled() {
            // render something?
        }

        if self.scanline == 240 {
            // do nothing... this is an idle scanline
        }

        if self.scanline == 241 && self.dot == 1 {
            debug!("vblank started");
            self.status.set_vblank();

            if self.ctrl.generate_nmi() {
                res.vblank_nmi = true;
            }
        }

        if self.scanline == 261 && self.dot == 1 {
            debug!("vblank ended");
            self.scanline = 0;
            self.status.clear_vblank();
            self.status.clear_sprite_zero_hit();
            self.status.clear_sprite_overflow();
        }

        res
    }
}
