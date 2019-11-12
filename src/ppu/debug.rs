use crate::mem::Memory;
use crate::palette::PALETTE;
use crate::ppu::PPU;
use crate::ppu::data::{
    BACKGROUND_PALETTE_ADDRESSES,
    SPRITE_PALETTE_ADDRESSES,
    PATTERN_TABLE_ADDRESSES,
};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

impl PPU {
    // For debugging purposes. Renders a pattern table at `x' and `y'.
    fn render_pattern_table(&mut self,
                            canvas: &mut Canvas<Window>,
                            pattern_table: u16,
                            x: i32,
                            y: i32)
    {
        let mut y = y;
        let mut temp_x = 0;

        for tile in 0 .. 256 {
            for row in 0 ..= 7 {
                let addr = pattern_table + (tile * 16) + row;
                let mut low_byte = self.data.read(addr);
                let mut high_byte = self.data.read(addr + 8);

                for col in 0 .. 8 {
                    let p1 = (low_byte & 0x80) >> 7;
                    let p2 = (high_byte & 0x80) >> 6;
                    low_byte <<= 1;
                    high_byte <<= 1;

                    let palette_index = p1 | p2;
                    let color = match palette_index {
                        0 => Color::RGB(30, 30, 30),
                        1 => Color::RGB(128, 128, 128),
                        2 => Color::RGB(255, 255, 255),
                        _ => Color::RGB(0, 0, 0),
                    };

                    canvas.set_draw_color(color);

                    let rect = Rect::new(x + temp_x + 2 * col,
                                         y + 2 * row as i32,
                                         2, 2);
                    canvas.fill_rect(rect).unwrap();
                }
            }

            temp_x += 16;

            if temp_x == 128 {
                temp_x = 0;
                y += 16;
            }
        }
    }

    // For debugging purposes. Displays the palettes and CHR data on the right
    // side of the screen.
    pub fn render_tile_data(&mut self, canvas: &mut Canvas<Window>) {
        let mut x = 256 * 3 + 20;
        let mut y = 10;

        //
        // Palettes
        //

        let width = 12;
        let height = 8;

        for base in BACKGROUND_PALETTE_ADDRESSES.iter() {
            for offset in 0 ..= 3 {
                let i = self.data.read(*base + offset as u16) as usize;
                canvas.set_draw_color(PALETTE[i % 64]);

                let rect = Rect::new(x + (width as i32) * offset, y, width, height);
                canvas.fill_rect(rect).unwrap();
            }

            y += 10;
        }

        y = 10;
        x = 256 * 3 + 20 + 48 + 16;
        for base in SPRITE_PALETTE_ADDRESSES.iter() {
            for offset in 0 ..= 3 {
                let i = self.data.read(*base + offset as u16) as usize;
                canvas.set_draw_color(PALETTE[i % 64]);

                let rect = Rect::new(x + (width as i32) * offset, y, width, height);
                canvas.fill_rect(rect).unwrap();
            }

            y += 10;
        }

        y += 20;

        //
        // CHR
        //
        x = 256 * 3 + 20;
        self.render_pattern_table(canvas, PATTERN_TABLE_ADDRESSES[0], x, y);
        self.render_pattern_table(canvas, PATTERN_TABLE_ADDRESSES[1], x + 144, y);
    }

    pub fn render_tile_borders(&mut self, canvas: &mut Canvas<Window>) {
        let scale = 3;
        canvas.set_draw_color(Color::RGB(200, 200, 200));

        for x in 0 .. 32 {
            for y in 0 .. 30 {
                let rect = Rect::new(8 * x * scale,
                                     8 * y * scale,
                                     8 * scale as u32,
                                     8 * scale as u32);
                canvas.draw_rect(rect).unwrap();
            }
        }
    }
}
