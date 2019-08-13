mod ctrl;
mod mask;
mod status;
mod oam;
mod scroll;
mod addr;
mod data;

use crate::mem::Memory;
use crate::ppu::ctrl::PPUCtrl;
use crate::ppu::mask::PPUMask;
use crate::ppu::status::PPUStatus;
use crate::ppu::scroll::PPUScroll;
use crate::ppu::addr::PPUAddr;
use crate::ppu::oam::OAM;
use crate::ppu::data::PPUData;

pub struct PPU {
    // PPU registers
    ctrl: PPUCtrl,
    mask: PPUMask,
    status: PPUStatus,
    oam_addr: u8,
    oam: OAM,
    scroll: PPUScroll,
    ppu_addr: PPUAddr,
    data: PPUData,

    // State for frame timing
    dot: u16,
    scanline: u16,

    // Rendering data
    nametable_byte: u8,
    attrtable_byte: u8,
    low_tile_byte: u8,
    high_tile_byte: u8,
    tile_data: u64,
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

            nametable_byte: 0,
            attrtable_byte: 0,
            low_tile_byte: 0,
            high_tile_byte: 0,
            tile_data: 0,
        }
    }

    pub fn load_vrom(&mut self, data: &Vec<u8>) {
        self.data.load_vrom(data);
    }

    fn rendering_enabled(&self) -> bool {
        self.mask.show_background() || self.mask.show_sprites()
    }

    fn inc_dot(&mut self) {
        self.dot += 1;
        if self.dot == 341 {
            self.dot = 0;
            self.scanline += 1;
            self.scanline %= 262;
        }
    }

    fn background_pixel(&self) -> u8 {
        if !self.mask.show_background() {
            return 0;
        }

        (self.fetch_tile_data() & 0x0f) as u8
    }

    fn render_pixel(&self) {
        let x = self.dot;
        let y = self.scanline;
        let background_pixel = self.background_pixel();

        // TODO sprite logic
    }

    fn fetch_nametable_byte(&mut self) {
        let v = self.ppu_addr.address();
        let addr = self.ctrl.base_nametable_addr() | (v & 0x0fff);
        self.nametable_byte = self.read(addr)
            .expect("unable to fetch nametable byte");
    }

    fn fetch_attrtable_byte(&mut self) {
        let v = self.ppu_addr.address();
        let addr = 0x23c0 | (v & 0x0c00) | ((v >> 4) & 0x38) | ((v >> 2) & 0x07);
        let shift = ((v >> 4) & 4) | (v & 2);
        let attrbyte = self.read(addr)
            .expect("unable to fetch attrtable byte");
        self.attrtable_byte = ((attrbyte >> shift) & 3) << 2;
    }

    fn fetch_low_tile_byte(&mut self) {
        let fine_y = (self.ppu_addr.address() >> 12) & 7;
        let tile = self.nametable_byte;
        let addr = self.ctrl.background_pattern_table_addr()
            + (tile as u16) * 16
            + fine_y;
        self.low_tile_byte = self.read(addr)
            .expect("unable to fetch low tile byte");
    }

    fn fetch_high_tile_byte(&mut self) {
        let fine_y = (self.ppu_addr.address() >> 12) & 7;
        let tile = self.nametable_byte;
        let addr = self.ctrl.background_pattern_table_addr()
            + (tile as u16) * 16
            + fine_y;
        self.high_tile_byte = self.read(addr + 8)
            .expect("unable to fetch high tile byte");
    }

    fn fetch_tile_data(&self) -> u32 {
        (self.tile_data >> 32) as u32
    }

    fn store_tile_data(&mut self) {
        let mut data: u32 = 0;
        for _ in 0 .. 8 {
            let a = self.attrtable_byte;
            let p1 = (self.low_tile_byte & 0x80) >> 7;
            let p2 = (self.high_tile_byte & 0x80) >> 6;
            self.low_tile_byte <<= 1;
            self.high_tile_byte <<= 1;
            data <<= 4;
            data |= (a | p1 | p2) as u32;
        }
        self.tile_data |= data as u64;
    }

    pub fn step(&mut self) -> StepResult {
        // http://wiki.nesdev.com/w/index.php/PPU_rendering#Line-by-line_timing
        //
        // There are a total of 262 scanlines per frame
        //   Scanlines 0 to 239 are for display (i.e. the NES is 256 x _240_)
        //   Scanline  240 is a post-render scanline (idle)
        //   Scanlines 241 to 260 are the vblank interval
        //   Scanline  261 is a pre-render scanline
        //
        // There are a total of 341 dots per scanline
        //   The first 256 dots are displayable (i.e. the NES is _256_ x 240)

        let mut res = StepResult{vblank_nmi: false};

        // All of this logic has been borrowed from github.com/foglemen/nes

        let pre_line        = self.scanline == 261;
        let visible_line    = self.scanline <= 239;
        let render_line     = pre_line || visible_line;

        let pre_fetch_cycle = self.dot >= 321 && self.dot <= 336;
        let visible_cycle   = self.dot >= 1   && self.dot <= 256;
        let fetch_cycle     = pre_fetch_cycle || visible_cycle;

        // background logic
        if self.rendering_enabled() {
            if visible_line && visible_cycle {
                self.render_pixel();
            }

            if render_line && fetch_cycle {
                self.tile_data <<= 4;

                match self.dot % 8 {
                    1 => self.fetch_nametable_byte(),
                    3 => self.fetch_attrtable_byte(),
                    5 => self.fetch_low_tile_byte(),
                    7 => self.fetch_high_tile_byte(),
                    0 => self.store_tile_data(),
                    _ => { }, // do nothing
                }
            }

            if pre_line && self.dot >= 280 && self.dot <= 304 {
                // copyY
            }

            if render_line {
                if fetch_cycle && self.dot % 8 == 0 {
                    // incrementX
                }

                if self.dot == 256 {
                    // incrementY
                }

                if self.dot == 257 {
                    // copyX
                }
            }
        }

        // sprite logic
        if self.rendering_enabled() && self.dot == 257 {
            if visible_line {
                // evaluateSprites
            }
            else {
                // sprite_count = 0
            }

        }

        // vblank logic
        if self.scanline == 241 && self.dot == 1 {
            debug!("vblank started");
            self.status.set_vblank();

            if self.ctrl.generate_nmi() {
                res.vblank_nmi = true;
            }

            self.inc_dot();
            return res;
        }

        if pre_line && self.dot == 1 {
            debug!("vblank ended");
            self.status.clear_vblank();
            self.status.clear_sprite_zero_hit();
            self.status.clear_sprite_overflow();
        }

        self.inc_dot();
        return res;
    }
}
