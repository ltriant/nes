mod ctrl;
mod mask;
mod status;
mod oam;
mod scroll;
mod addr;
mod data;

use crate::console::NES_PPU_DEBUG;
use crate::palette::PALETTE;
use crate::mem::Memory;
use crate::ppu::ctrl::PPUCtrl;
use crate::ppu::mask::PPUMask;
use crate::ppu::status::PPUStatus;
use crate::ppu::scroll::PPUScroll;
use crate::ppu::addr::PPUAddr;
use crate::ppu::oam::OAM;
use crate::ppu::data::{PPUData, PALETTE_ADDRESSES};

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

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

    // Sprite data
    sprite_count: usize,
    sprite_patterns: [u32; 8],
    sprite_positions: [u8; 8],
    sprite_priorities: [u8; 8],
    sprite_indexes: [u8; 8],

    odd_frame: bool,

    // NMI stuff
    nmi_occurred: bool,
    nmi_output: bool,
    nmi_previous: bool,
    nmi_delay: usize,

    // Scrolling registers
    t: u16,
    x: u8,
    w: bool,
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
                let PPUStatus(mut n) = self.status;

                // reset the latch
                self.scroll.reset_latch();
                self.ppu_addr.reset_latch();

                if self.nmi_occurred {
                    n |= 1 << 7;
                }
                self.nmi_occurred = false;
                self.nmi_change();

                // w:                  = 0
                self.w = false;

                Ok(n)
            },
            0x2003 => Ok(0), // OAMADDR is write-only
            0x2004 => panic!("OAMData is unreadable... I think. Double check if this panic happens."),
            0x2005 => Ok(0), // PPUSCROLL is write-only
            0x2006 => Ok(0), // PPUADDR is write-only
            0x2007 => {
                let rv = self.data.read(self.ppu_addr.address())?;
                //self.ppu_addr.increment(self.ctrl.vram_addr_increment());
                Ok(rv)
            },
            _ => panic!("bad PPU address 0x{:04X}", address)
        }
    }

    fn write(&mut self, address: u16, val: u8) -> Result<u8, String> {
        match address {
            0x2000 => {
                self.ctrl = PPUCtrl(val);

                if (val & 0x01) != 0 {
                    self.scroll.incr_x();
                }

                if (val & 0x02) != 0 {
                    self.scroll.incr_y();
                }

                // t: ...BA.. ........ = d: ......BA
                self.t = (self.t & 0xf3ff)
                       | (((val as u16) & 0x03) << 10);

                self.nmi_output = (val >> 7) & 1 == 1;
                self.nmi_change();

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

                if self.w {
                    // t: CBA..HG FED..... = d: HGFEDCBA
                    // w:                  = 0
                    self.t = (self.t & 0x8fff)
                           | (((val as u16) & 0x07) << 12);
                    self.t = (self.t & 0xfc1f)
                           | (((val as u16) & 0xF8) << 2);
                    self.w = false;
                }
                else {
                    // t: ....... ...HGFED = d: HGFED...
                    // x:              CBA = d: .....CBA
                    // w:                  = 1
                    self.t = (self.t & 0xffe0)
                           | ((val as u16) >> 3);
                    self.x = val & 0x07;
                    self.w = true;
                }

                Ok(val)
            },
            0x2006 => {
                self.ppu_addr.write(val);

                if self.w {
                    // t: ....... HGFEDCBA = d: HGFEDCBA
                    // v                   = t
                    // w:                  = 0
                    self.t = (self.t & 0xff00)
                           | (val as u16);
                    self.ppu_addr.val = self.t;
                    self.w = false;
                }
                else {
                    // t: .FEDCBA ........ = d: ..FEDCBA
                    // t: X...... ........ = 0
                    // w:                  = 1
                    self.t = (self.t & 0x80ff)
                           | (((val as u16) & 0x3f) << 8);
                    self.w = true;
                }

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
    pub frame_finished: bool,
}

impl PPU {
    pub fn new_nes_ppu() -> Self {
        Self {
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

            sprite_count: 0,
            sprite_patterns: [0; 8],
            sprite_positions: [0; 8],
            sprite_priorities: [0; 8],
            sprite_indexes: [0; 8],

            odd_frame: false,

            nmi_occurred: false,
            nmi_output: false,
            nmi_previous: false,
            nmi_delay: 0,

            t: 0,
            x: 0,
            w: false,
        }
    }

    pub fn load_vrom(&mut self, data: &Vec<u8>) {
        self.data.load_vrom(data);
    }

    fn rendering_enabled(&self) -> bool {
        self.mask.show_background() || self.mask.show_sprites()
    }

    fn inc_dot(&mut self) {
        if self.mask.show_sprites() && self.mask.show_background() {
            if self.odd_frame && self.scanline == 261 && self.dot == 339 {
                self.dot = 0;
                self.scanline = 0;
                self.odd_frame = false;
                return;
            }
        }

        self.dot += 1;
        if self.dot == 341 {
            self.dot = 0;
            self.scanline += 1;

            if self.scanline > 261 {
                self.scanline = 0;
                self.odd_frame = ! self.odd_frame;
            }
        }
    }

    fn nmi_change(&mut self) {
        let nmi = self.nmi_output && self.nmi_occurred;
        if nmi && !self.nmi_previous {
            self.nmi_delay = 15;
        }
        self.nmi_previous = nmi;
    }

    fn increment_x(&mut self) {
        // https://wiki.nesdev.com/w/index.php/PPU_scrolling

        if self.ppu_addr.val & 0x001f == 31 {
            self.ppu_addr.val &= 0xffe0;
            self.ppu_addr.val ^= 0x0400;
        }
        else {
            self.ppu_addr.val += 1;
        }
    }

    fn increment_y(&mut self) {
        // https://wiki.nesdev.com/w/index.php/PPU_scrolling

        if self.ppu_addr.val & 0x7000 != 0x7000 {
            self.ppu_addr.val += 0x1000;
        }
        else {
            self.ppu_addr.val &= 0x8fff;

            let mut y = (self.ppu_addr.val & 0x03e0) >> 5;

            if y == 29 {
                y = 0;
                self.ppu_addr.val ^= 0x0800;
            }
            else if y == 31 {
                y = 0;
            }
            else {
                y += 1;
            }

            self.ppu_addr.val = (self.ppu_addr.val & 0xfc1f) | (y << 5);
        }
    }

    fn copy_x(&mut self) {
        // v: ....F.. ...EDCBA = t: ....F.. ...EDCBA
        self.ppu_addr.val = (self.ppu_addr.val & 0xfbe0)
                          | (self.t & 0x041f);
    }

    fn copy_y(&mut self) {
        // v: IHGF.ED CBA..... = t: IHGF.ED CBA.....
        self.ppu_addr.val = (self.ppu_addr.val & 0x841f)
                          | (self.t & 0x7be0);
    }

    fn background_pixel(&self) -> Option<u8> {
        if !self.mask.show_background() {
            return None;
        }

        let tile_data = self.fetch_tile_data() >> ((7 - self.scroll.x) * 4);
        let pixel = (tile_data & 0x0f) as u8;

        Some(pixel)
    }

    fn sprite_pixel(&self) -> Option<(u8, u8)> {
        if !self.mask.show_sprites() {
            return None;
        }

        for i in 0 .. self.sprite_count {
            let mut offset = (self.dot as i16 - 1) - self.sprite_positions[i] as i16;

            if offset < 0 || offset > 7 {
                continue;
            }

            offset = 7 - offset;

            let color = ((self.sprite_patterns[i] >> (offset * 4)) & 0x0f) as u8;
            if color % 4 == 0 {
                continue;
            }

            return Some((i as u8, color));
        }

        None
    }

    fn fetch_sprite_pattern(&mut self, i: u16, row: i16) -> u32 {
        let mut tile = self.oam.read(i * 4 + 1).unwrap() as u16;
        let attributes = self.oam.read(i * 4 + 2).unwrap();

        let mut address: u16 = 0;
        let mut row = row;

        if self.ctrl.sprite_size() == 8 {
            if attributes & 0x80 == 0x80 {
                row = 7 - row;
            }

            address = self.ctrl.sprite_pattern_table_addr()
                + (tile * 16)
                + row as u16;
        }
        else {
            if attributes & 0x80 == 0x80 {
                row = 15 - row;
            }

            let table = tile & 1;
            tile &= 0xfe;

            if row > 7 {
                tile += 1;
                row -= 8;
            }

            address = 0x1000 * table
                + (tile * 16)
                + row as u16;
        }

        let a = (attributes & 3) << 2;
        let mut low_tile_byte = self.data.read(address).unwrap();
        let mut high_tile_byte = self.data.read(address + 8).unwrap();

        let mut data = 0;

        for _ in 0 .. 8 {
            let mut p1 = 0;
            let mut p2 = 0;

            if attributes & 0x40 == 0x40 {
                p1 = (low_tile_byte & 1) << 0;
                p2 = (high_tile_byte & 1) << 1;
                low_tile_byte >>= 1;
                high_tile_byte >>= 1;
            }
            else {
                p1 = (low_tile_byte & 0x80) >> 7;
                p2 = (high_tile_byte & 0x80) >> 6;
                low_tile_byte <<= 1;
                high_tile_byte <<= 1;
            }

            data <<= 4;
            data |= (a | p1 | p2) as u32;
        }

        data
    }

    fn evaluate_sprites(&mut self) {
        let sz = self.ctrl.sprite_size() as i16;

        let mut count = 0;

        for i in 0 .. 64 {
            let y = self.oam.read(i * 4 + 0).unwrap();
            let a = self.oam.read(i * 4 + 2).unwrap();
            let x = self.oam.read(i * 4 + 3).unwrap();

            let row: i16 = (self.scanline as i16) - (y as i16);

            if row < 0 || row >= sz {
                continue
            }

            if count < 8 {
                self.sprite_patterns[count] = self.fetch_sprite_pattern(i, row);
                self.sprite_positions[count] = x;
                self.sprite_priorities[count] = (a >> 5) & 1;
                self.sprite_indexes[count] = i as u8;
            }

            count += 1;
        }

        if count > 8 {
            count = 8;
            self.status.set_sprite_overflow();
        }

        self.sprite_count = count;
    }

    fn render_pixel(&mut self, canvas: &mut Canvas<Window>) {
        let x = self.dot - 1;
        let y = self.scanline;

        let mut address = 0;

        let background = self.background_pixel();
        let sprite     = self.sprite_pixel();

        match (background, sprite) {
            (None, None) => {
                address = 0;
            },
            (Some(background), None) => {
                address = background as u16;

                if x < 8 && !self.mask.show_background_leftmost() {
                    address = 0;
                }
            },
            (None, Some((_, sprite))) => {
                address = sprite as u16 | 0x10;

                if x < 8 && !self.mask.show_sprites_leftmost() {
                    address = 0;
                }
            },
            (Some(background), Some((i, sprite))) => {
                if self.sprite_indexes[i as usize] == 0 && x < 255 {
                    self.status.set_sprite_zero_hit();
                }

                if self.sprite_priorities[i as usize] == 0 {
                    address = sprite as u16 | 0x10;
                }
                else {
                    address = background as u16;
                }
            }
        }

        // Set the base palette address
        address |= 0x3f00;

        let palette_index = self.data.read(address)
            .expect("unable to read palette index") % 64;
        let color = PALETTE[palette_index as usize];
        let rect = Rect::new((x as i32) * 2, (y as i32) * 2, 2, 2);

        /*
        debug!("color_addr = 0x{:04x}, palette_index = {}, color = {:?}",
               address, palette_index, color);
           */

        canvas.set_draw_color(color);
        canvas.fill_rect(rect).expect("unable to fill rectangle");
    }

    fn fetch_nametable_byte(&mut self) -> u8 {
        let v = self.ppu_addr.address();
        let addr = self.ctrl.base_nametable_addr() | (v & 0x0fff);
        debug!("fetching NT byte from 0x{:04X}", addr);
        self.data.read(addr).expect("unable to fetch NT byte")
    }

    fn fetch_attrtable_byte(&mut self) -> u8 {
        let v = self.ppu_addr.address();

        let addr =
            0x23c0 | (v & 0x0c00) | ((v >> 4) & 0x38) | ((v >> 2) & 0x07);
        debug!("fetching AT byte from 0x{:04X}", addr);
        let attrbyte = self.data.read(addr).expect("unable to fetch AT byte");

        let shift = ((v >> 4) & 4) | (v & 2);
        ((attrbyte >> shift) & 3) << 2
    }

    fn fetch_low_tile_byte(&mut self) -> u8 {
        let fine_y = (self.ppu_addr.address() >> 12) & 7;
        let tile = self.nametable_byte;
        let addr = self.ctrl.background_pattern_table_addr()
            + ((tile as u16) * 16)
            + fine_y;

        debug!("fetching low tile byte from 0x{:04X}", addr);
        self.data.read(addr).expect("unable to fetch low tile byte")
    }

    fn fetch_high_tile_byte(&mut self) -> u8 {
        let fine_y = (self.ppu_addr.address() >> 12) & 7;
        let tile = self.nametable_byte;
        let addr = self.ctrl.background_pattern_table_addr()
            + (tile as u16) * 16
            + fine_y;

        debug!("fetching high tile byte from 0x{:04X}", addr + 8);
        self.data.read(addr + 8).expect("unable to fetch high tile byte")
    }

    fn fetch_tile_data(&self) -> u32 {
        (self.tile_data >> 32) as u32
    }

    fn store_tile_data(&mut self) {
        let data: u32 = (0 .. 8).fold(0, |acc, _| {
            let p1 = (self.low_tile_byte & 0x80) >> 7;
            let p2 = (self.high_tile_byte & 0x80) >> 6;

            self.low_tile_byte <<= 1;
            self.high_tile_byte <<= 1;

            let a = self.attrtable_byte;
            let b = (a | p1 | p2) as u32;

            (acc << 4) | b
        } );

        self.tile_data |= data as u64;
    }

    fn render_palettes(&mut self, canvas: &mut Canvas<Window>) {
        let x = 256 * 2 + 1;
        let width = 12;
        let height = 12;

        let mut y = 0;

        for base in PALETTE_ADDRESSES.iter() {
            for offset in 0 ..= 3 {
                let i = self.data.read(*base + offset as u16).unwrap() as usize;
                canvas.set_draw_color(PALETTE[i]);

                let rect = Rect::new(x + (width as i32) * offset, y, width, height);
                canvas.fill_rect(rect).unwrap();
            }

            y += 20;
        }
    }

    pub fn step(&mut self, canvas: &mut Canvas<Window>) -> StepResult {
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

        let mut res = StepResult{
            vblank_nmi: false,
            frame_finished: false,
        };

        // All of this logic has been borrowed from github.com/foglemen/nes

        let pre_line         = self.scanline == 261;
        let visible_line     = self.scanline <= 239;
        let render_line      = pre_line || visible_line;
        let post_render_line = self.scanline == 240;

        let pre_fetch_cycle = self.dot >= 321 && self.dot <= 336;
        let visible_cycle   = self.dot >= 1   && self.dot <= 256;
        let fetch_cycle     = pre_fetch_cycle || visible_cycle;

        // background logic
        if self.rendering_enabled() {
            if visible_line && visible_cycle {
                self.render_pixel(canvas);
            }

            if render_line && fetch_cycle {
                self.tile_data <<= 4;

                match self.dot % 8 {
                    1 => {
                        let b = self.fetch_nametable_byte();
                        self.nametable_byte = b;
                    },
                    3 => {
                        let b = self.fetch_attrtable_byte();
                        self.attrtable_byte = b;
                    },
                    5 => {
                        let b = self.fetch_low_tile_byte();
                        self.low_tile_byte = b;
                    },
                    7 => {
                        let b = self.fetch_high_tile_byte();
                        self.high_tile_byte = b;
                    },
                    0 => self.store_tile_data(),
                    _ => { }, // do nothing
                }
            }

            if pre_line && self.dot >= 280 && self.dot <= 304 {
                self.copy_y();
            }

            if render_line {
                if fetch_cycle && self.dot % 8 == 0 {
                    self.increment_x();
                }

                if self.dot == 256 {
                    self.increment_y();
                }

                if self.dot == 257 {
                    self.copy_x();
                }
            }
        }

        // sprite logic
        if self.rendering_enabled() && self.dot == 257 {
            if visible_line {
                self.evaluate_sprites();
            }
            else {
                self.sprite_count = 0;
            }

        }

        // vblank logic
        if self.scanline == 241 && self.dot == 1 {
            debug!("vblank started");
            self.status.set_vblank();

            self.nmi_occurred = true;
            self.nmi_change();

            if self.nmi_delay > 0 {
                self.nmi_delay -= 1;

                if self.nmi_delay == 0 && self.nmi_output && self.nmi_occurred {
                    res.vblank_nmi = true;
                }
            }

            if self.ctrl.generate_nmi() {
                // TODO
                //res.vblank_nmi = true;
            }

            res.frame_finished = true;

            if *NES_PPU_DEBUG {
                self.render_palettes(canvas);
            }

            self.inc_dot();
            return res;
        }

        if pre_line && self.dot == 1 {
            debug!("vblank ended");
            self.status.clear_vblank();
            self.status.clear_sprite_zero_hit();
            self.status.clear_sprite_overflow();

            self.nmi_occurred = false;
            self.nmi_change();
        }

        if self.nmi_delay > 0 {
            self.nmi_delay -= 1;

            if self.nmi_delay == 0 && self.nmi_output && self.nmi_occurred {
                res.vblank_nmi = true;
            }
        }
        self.inc_dot();
        return res;
    }
}
