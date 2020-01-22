mod debug;
mod regs;

use std::io;
use std::fs::File;

use crate::console::NES_PPU_DEBUG;
use crate::palette::PALETTE;
use crate::mem::Memory;
use crate::ppu::regs::PPUCtrl;
use crate::ppu::regs::PPUMask;
use crate::ppu::regs::PPUStatus;
use crate::ppu::regs::OAM;
use crate::ppu::regs::PPUData;
use crate::serde;
use crate::serde::Storeable;

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
    ppu_addr: u16,
    pub data: PPUData,

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
    sprite_indexes: [usize; 8],

    // Odd/even frame state
    odd_frame: bool,

    // NMI stuff
    nmi_occurred: bool,
    nmi_output: bool,
    nmi_previous: bool,
    nmi_delay: usize,

    // PPUSCROLL registers
    t: u16,
    x: u8,
    w: bool,

    // PPUDATA read buffer
    buffered_data: u8,

    // The last written value to any PPU register
    // For use when reading the PPUSTATUS
    last_value: u8,
}

impl Memory for PPU {
    fn read(&mut self, address: u16) -> u8 {
        // The PPU registers exist from 0x2000 to 0x2007, the rest of the
        // address space is just a mirror of these first eight bytes.
        let address = address % 8 + 0x2000;
        match address {
            0x2000 => {
                let PPUCtrl(n) = self.ctrl;
                n
            },
            0x2001 => {
                let PPUMask(n) = self.mask;
                n
            },
            0x2002 => {
                let PPUStatus(mut n) = self.status;

                // Whatever the last value was written to the PPU (to any
                // register), set the first 5 bits of the PPUSTATUS value to
                // the first 5 bits of _that_ last value.
                n &= ! 0x1f;
                n |= self.last_value & 0x1f;

                if self.nmi_occurred {
                    n |= 1 << 7;
                }
                self.nmi_occurred = false;
                self.nmi_change();

                // w:                  = 0
                self.w = false;

                n
            },
            0x2003 => 0, // OAMADDR is write-only
            0x2004 => self.oam.read(self.oam_addr as u16),
            0x2005 => 0, // PPUSCROLL is write-only
            0x2006 => 0, // PPUADDR is write-only
            0x2007 => {
                let rv;

                // Emulate 1-byte delayed read
                // Palette reads aren't buffered
                if self.ppu_addr % 0x4000 <= 0x3eff {
                    rv = self.buffered_data;
                    self.buffered_data = self.data.read(self.ppu_addr);
                } else {
                    // TODO why do we subtract 0x1000 ?
                    self.buffered_data = self.data.read(self.ppu_addr - 0x1000);
                    rv = self.data.read(self.ppu_addr);
                }

                self.ppu_addr = self.ppu_addr.wrapping_add(
                    self.ctrl.vram_addr_increment());

                rv
            },

            _ => panic!("bad PPU address 0x{:04X}", address)
        }
    }

    fn write(&mut self, address: u16, val: u8) {
        self.last_value = val;

        let address = address % 8 + 0x2000;
        match address {
            0x2000 => {
                self.ctrl = PPUCtrl(val);

                // t: ...BA.. ........ = d: ......BA
                self.t = (self.t & 0xf3ff)
                       | (((val as u16) & 0x03) << 10);

                self.nmi_output = (val >> 7) & 1 == 1;
                self.nmi_change();
            },
            0x2001 => { self.mask = PPUMask(val) },
            0x2002 => { },
            0x2003 => { self.oam_addr = val },
            0x2004 => {
                self.oam.write(self.oam_addr as u16, val);
                self.oam_addr = self.oam_addr.wrapping_add(1);
            },
            0x2005 => {
                if self.w {
                    // t: CBA..HG FED..... = d: HGFEDCBA
                    // w:                  = 0
                    self.t = (self.t & 0x8fff)
                           | (((val as u16) & 0x07) << 12);
                    self.t = (self.t & 0xfc1f)
                           | (((val as u16) & 0xf8) << 2);
                    self.w = false;
                } else {
                    // t: ....... ...HGFED = d: HGFED...
                    // x:              CBA = d: .....CBA
                    // w:                  = 1
                    self.t = (self.t & 0xffe0)
                           | ((val as u16) >> 3);
                    self.x = val & 0x07;
                    self.w = true;
                }
            },
            0x2006 => {
                if self.w {
                    // t: ....... HGFEDCBA = d: HGFEDCBA
                    // v                   = t
                    // w:                  = 0
                    self.t = (self.t & 0xff00)
                           | (val as u16);
                    self.ppu_addr = self.t;
                    self.w = false;
                } else {
                    // t: .FEDCBA ........ = d: ..FEDCBA
                    // t: X...... ........ = 0
                    // w:                  = 1
                    self.t = (self.t & 0x80ff)
                           | (((val as u16) & 0x3f) << 8);
                    self.w = true;
                }
            },
            0x2007 => {
                let rv = self.data.write(self.ppu_addr, val);
                self.ppu_addr = self.ppu_addr.wrapping_add(
                    self.ctrl.vram_addr_increment());
                rv
            },

            _ => panic!("bad PPU address 0x{:04X}", address)
        }
    }
}

impl Storeable for PPU {
    fn save(&self, output: &mut File) -> io::Result<()> {
        let PPUCtrl(v) = self.ctrl;
        serde::encode_u8(output, v)?;

        let PPUMask(v) = self.mask;
        serde::encode_u8(output, v)?;

        let PPUStatus(v) = self.status;
        serde::encode_u8(output, v)?;

        serde::encode_u8(output, self.oam_addr)?;
        self.oam.save(output)?;
        serde::encode_u16(output, self.ppu_addr)?;

        self.data.save(output)?;

        serde::encode_u16(output, self.dot)?;
        serde::encode_u16(output, self.scanline)?;

        serde::encode_u8(output, self.nametable_byte)?;
        serde::encode_u8(output, self.attrtable_byte)?;
        serde::encode_u8(output, self.low_tile_byte)?;
        serde::encode_u8(output, self.high_tile_byte)?;
        serde::encode_u64(output, self.tile_data)?;

        serde::encode_usize(output, self.sprite_count)?;
        for i in 0 .. 7 {
            serde::encode_u32(output, self.sprite_patterns[i])?;
            serde::encode_u8(output, self.sprite_positions[i])?;
            serde::encode_u8(output, self.sprite_priorities[i])?;
            serde::encode_usize(output, self.sprite_indexes[i])?;
        }

        serde::encode_u8(output, self.odd_frame as u8)?;
        serde::encode_u8(output, self.nmi_occurred as u8)?;
        serde::encode_u8(output, self.nmi_output as u8)?;
        serde::encode_usize(output, self.nmi_delay)?;

        serde::encode_u16(output, self.t)?;
        serde::encode_u8(output, self.x)?;
        serde::encode_u8(output, self.w as u8)?;

        serde::encode_u8(output, self.buffered_data)?;
        serde::encode_u8(output, self.last_value)?;

        Ok(())
    }

    fn load(&mut self, input: &mut File) -> io::Result<()> {
        self.ctrl = PPUCtrl(serde::decode_u8(input)?);
        self.mask = PPUMask(serde::decode_u8(input)?);
        self.status = PPUStatus(serde::decode_u8(input)?);
        self.oam_addr = serde::decode_u8(input)?;
        self.oam.load(input)?;
        self.ppu_addr = serde::decode_u16(input)?;

        self.data.load(input)?;

        self.dot = serde::decode_u16(input)?;
        self.scanline = serde::decode_u16(input)?;

        self.nametable_byte = serde::decode_u8(input)?;
        self.attrtable_byte = serde::decode_u8(input)?;
        self.low_tile_byte = serde::decode_u8(input)?;
        self.high_tile_byte = serde::decode_u8(input)?;
        self.tile_data = serde::decode_u64(input)?;

        self.sprite_count = serde::decode_usize(input)?;
        for i in 0 .. 7 {
            self.sprite_patterns[i] = serde::decode_u32(input)?;
            self.sprite_positions[i] = serde::decode_u8(input)?;
            self.sprite_priorities[i] = serde::decode_u8(input)?;
            self.sprite_indexes[i] = serde::decode_usize(input)?;
        }

        self.odd_frame = serde::decode_u8(input)? != 0;
        self.nmi_occurred = serde::decode_u8(input)? != 0;
        self.nmi_output = serde::decode_u8(input)? != 0;
        self.nmi_previous = serde::decode_u8(input)? != 0;
        self.nmi_delay = serde::decode_usize(input)?;

        self.t = serde::decode_u16(input)?;
        self.x = serde::decode_u8(input)?;
        self.w = serde::decode_u8(input)? != 0;

        self.buffered_data = serde::decode_u8(input)?;
        self.last_value = serde::decode_u8(input)?;

        Ok(())
    }
}

pub struct StepResult {
    pub trigger_nmi: bool,
    pub trigger_irq: bool,
    pub frame_finished: bool,
    pub signal_scanline: bool,
}

impl PPU {
    pub fn new_nes_ppu() -> Self {
        Self {
            ctrl: PPUCtrl(0),
            mask: PPUMask(0),
            status: PPUStatus(0),
            oam: OAM::new_nes_oam(),
            oam_addr: 0,
            ppu_addr: 0,
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

            buffered_data: 0,

            last_value: 0,
        }
    }

    fn rendering_enabled(&self) -> bool {
        self.mask.show_background() || self.mask.show_sprites()
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

        // if coarse X == 31
        if self.ppu_addr & 0x001f == 31 {
            // coarse X = 0
            self.ppu_addr &= !0x001f;

            // switch horizontal nametable
            self.ppu_addr ^= 0x0400;
        } else {
            // increment coarse X
            self.ppu_addr += 1;
        }
    }

    fn increment_y(&mut self) {
        // https://wiki.nesdev.com/w/index.php/PPU_scrolling

        // if fine Y < 7
        if self.ppu_addr & 0x7000 != 0x7000 {
            // increment fine Y
            self.ppu_addr += 0x1000;
        } else {
            // fine Y = 0
            self.ppu_addr &= !0x7000;

            // let y = coarse Y
            let mut y = (self.ppu_addr & 0x03e0) >> 5;

            if y == 29 {
                // coarse Y = 0
                y = 0;

                // switch vertical nametable
                self.ppu_addr ^= 0x0800;
            } else if y == 31 {
                // coarse Y = 0, nametable not switched
                y = 0;
            } else {
                // increment coarse Y
                y += 1;
            }

            // put coarse Y back into v
            self.ppu_addr = (self.ppu_addr & !0x03e0) | (y << 5);
        }
    }

    fn background_pixel(&self) -> Option<u8> {
        if !self.mask.show_background() {
            return None;
        }

        let tile_data = self.fetch_tile_data() >> ((7 - self.x) * 4);
        let color = (tile_data & 0x0f) as u8;

        Some(color)
    }

    fn sprite_pixel(&self) -> Option<(usize, u8)> {
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

            return Some((i, color));
        }

        None
    }

    // Fetches the sprite pattern for a single row of a tile. If you wanted the
    // pattern tables for every row of a sprite, you would call this with the
    // `row' parameter being the values from 0 to 7 (inclusive).
    fn fetch_sprite_pattern(&mut self, i: u16, row: i16) -> u32 {
        let mut tile = self.oam.read(i * 4 + 1) as u16;
        let attributes = self.oam.read(i * 4 + 2);

        let address: u16;
        let mut row = row;

        if self.ctrl.sprite_size() == 8 {
            if attributes & 0x80 == 0x80 {
                row = 7 - row;
            }

            address = self.ctrl.sprite_pattern_table_addr()
                + (tile * 16)
                + row as u16;
        } else {
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

        let a = ((attributes & 3) << 2) as u32;
        let mut low_tile_byte = self.data.read(address) as u32;
        let mut high_tile_byte = self.data.read(address + 8) as u32;

        // Now we need to return a 32-bit unsigned value, representing the 8
        // pixels of this row of the sprite. This means we have 4 bits per
        // pixel. The 4 bits are made up of 2 bits of attribute data, and 2 bits
        // of palette data.
        //
        // The attribute data contains the palette offset that this pixel
        // selects.
        //
        // The low tile byte contains one bit of the pattern table data. And the
        // high tile byte contains the other bit of pattern table data. These
        // two bits are OR'd together and values 0, 1, and 2 map to an index
        // within the palette.

        (0 .. 8).fold(0, |acc, _| {
            let p1;
            let p2;

            // Flip the sprite vertically, so we read from the other end of the
            // low and high tile bytes
            if attributes & 0x40 == 0x40 {
                p1 = (low_tile_byte & 1) << 0;
                p2 = (high_tile_byte & 1) << 1;
                low_tile_byte >>= 1;
                high_tile_byte >>= 1;
            } else {
                p1 = (low_tile_byte & 0x80) >> 7;
                p2 = (high_tile_byte & 0x80) >> 6;
                low_tile_byte <<= 1;
                high_tile_byte <<= 1;
            }

            (acc << 4) | (a | p1 | p2)
        } )
    }

    fn evaluate_sprites(&mut self) {
        let sz = self.ctrl.sprite_size() as i16;

        let mut count = 0;

        for i in 0 .. 64 {
            let sprite = i as u16;
            let y = self.oam.read(sprite * 4 + 0);
            let a = self.oam.read(sprite * 4 + 2);
            let x = self.oam.read(sprite * 4 + 3);

            let row: i16 = (self.scanline as i16) - (y as i16);

            if row < 0 || row >= sz {
                continue
            }

            if count < 8 {
                self.sprite_patterns[count] = self.fetch_sprite_pattern(sprite, row);
                self.sprite_positions[count] = x;
                self.sprite_priorities[count] = (a >> 5) & 1;
                self.sprite_indexes[count] = i;
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

        let mut background  = self.background_pixel().unwrap_or(0);
        let     (i, sprite) = self.sprite_pixel().unwrap_or((0, 0));
        let mut sprite = sprite;

        if x < 8 && !self.mask.show_background_leftmost() {
            background = 0;
        }

        if x < 8 && !self.mask.show_sprites_leftmost() {
            sprite = 0;
        }

        // https://wiki.nesdev.com/w/index.php/PPU_palettes#The_background_palette_hack
        // Palette addresses $3f04, $3f08, and $3f0c should retrieve the
        // transparent background value at $3f00
        let b = background % 4 != 0;
        let s = sprite % 4 != 0;

        let address_low_nyb = match (b, s) {
            (false, false) => 0,
            (false, true) => {
                sprite as u16 | 0x10
            },
            (true, false) => {
                background as u16
            },
            (true, true) => {
                if self.sprite_indexes[i] == 0 && x < 255 {
                    self.status.set_sprite_zero_hit();
                }

                if self.sprite_priorities[i] == 0 {
                    sprite as u16 | 0x10
                } else {
                    background as u16
                }
            },
        };

        // Set the base palette address
        let address = 0x3f00 | address_low_nyb;

        let palette_index = self.data.read(address) % 64;
        let color = PALETTE[palette_index as usize];
        let rect = Rect::new((x as i32) * 3, (y as i32) * 3, 3, 3);

        canvas.set_draw_color(color);
        canvas.fill_rect(rect).unwrap();
    }

    fn fetch_nametable_byte(&mut self) -> u8 {
        let v = self.ppu_addr;
        // https://wiki.nesdev.com/w/index.php/PPU_scrolling#Tile_and_attribute_fetching
        let addr = 0x2000 | (v & 0x0fff);
        debug!("fetching NT byte from 0x{:04X}", addr);
        self.data.read(addr)
    }

    fn fetch_attrtable_byte(&mut self) -> u8 {
        let v = self.ppu_addr;

        // https://wiki.nesdev.com/w/index.php/PPU_scrolling#Tile_and_attribute_fetching
        let addr = 0x23c0
                 | (v & 0x0c00)
                 | ((v >> 4) & 0x38)
                 | ((v >> 2) & 0x07);

        debug!("fetching AT byte from 0x{:04X}", addr);
        let attrbyte = self.data.read(addr);

        let shift = ((v >> 4) & 4) | (v & 2);
        ((attrbyte >> shift) & 3) << 2
    }

    fn fetch_low_tile_byte(&mut self) -> u8 {
        let fine_y = (self.ppu_addr >> 12) & 0x07;
        let tile = self.nametable_byte as u16;
        let addr = self.ctrl.background_pattern_table_addr()
            + fine_y
            + (16 * tile);

        debug!("fetching low tile byte from 0x{:04X}", addr);
        self.data.read(addr)
    }

    fn fetch_high_tile_byte(&mut self) -> u8 {
        let fine_y = (self.ppu_addr >> 12) & 0x07;
        let tile = self.nametable_byte as u16;
        let addr = self.ctrl.background_pattern_table_addr()
            + fine_y
            + (16 * tile);

        debug!("fetching high tile byte from 0x{:04X}", addr + 8);
        self.data.read(addr + 8)
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

    fn tick(&mut self, res: &mut StepResult) {
        if self.nmi_delay > 0 {
            self.nmi_delay -= 1;

            if self.nmi_delay == 0 && self.nmi_output && self.nmi_occurred {
                res.trigger_nmi = true;
            }
        }

        if self.rendering_enabled() {
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
            trigger_nmi: false,
            trigger_irq: false,
            frame_finished: false,
            signal_scanline: false,
        };

        self.tick(&mut res);

        // All of this logic has been borrowed from github.com/fogleman/nes

        let pre_line          = self.scanline == 261;
        let visible_line      = self.scanline <= 239;
        let render_line       = pre_line || visible_line;
        let _post_render_line = self.scanline == 240;

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
                // v: IHGF.ED CBA..... = t: IHGF.ED CBA.....
                self.ppu_addr = (self.ppu_addr & 0x841f)
                              | (self.t & 0x7be0);
            }

            if render_line {
                if fetch_cycle && self.dot % 8 == 0 {
                    self.increment_x();
                }

                if self.dot == 256 {
                    self.increment_y();
                }

                if self.dot == 257 {
                    // v: ....F.. ...EDCBA = t: ....F.. ...EDCBA
                    self.ppu_addr = (self.ppu_addr & 0xfbe0)
                                  | (self.t & 0x041f);
                }
            }
        }

        // sprite logic
        if self.rendering_enabled() && self.dot == 257 {
            if visible_line {
                self.evaluate_sprites();
            } else {
                self.sprite_count = 0;
            }

        }

        // vblank logic
        if self.scanline == 241 && self.dot == 1 {
            debug!("vblank started");

            self.nmi_occurred = true;
            self.nmi_change();

            if *NES_PPU_DEBUG {
                self.render_tile_data(canvas);
                self.render_tile_borders(canvas);
            }

            res.frame_finished = true;
            res.trigger_irq = self.data.mapper.irq_flag();
            return res;
        }

        if (pre_line || visible_line) && self.rendering_enabled() && self.dot == 280 {
            debug!("scanline ended");
            res.signal_scanline = true;
        }

        if pre_line && self.dot == 1 {
            debug!("vblank ended");
            self.status.clear_sprite_zero_hit();
            self.status.clear_sprite_overflow();

            self.nmi_occurred = false;
            self.nmi_change();
        }

        res.trigger_irq = self.data.mapper.irq_flag();
        return res;
    }
}
