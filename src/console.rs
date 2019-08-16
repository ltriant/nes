use std::fs::File;

use crate::cpu::CPU;
use crate::mem::NESMemory;
use crate::ppu::PPU;
use crate::ines::{Cartridge, CartridgeError};

use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Console {
    sdl_ctx: Sdl,
    canvas:  Canvas<Window>,
    cpu:     CPU,
}

impl Console {
    pub fn new_nes_console() -> Console {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let width = 256 * 2 + 50;
        let height = 240 * 2;
        let window = video_subsystem.window("nes", width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        for _ in 0 .. 2 {
            canvas.clear();
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.fill_rect(Rect::new(0, 0, width, height)).unwrap();
            canvas.present();
        }

        let ppu = PPU::new_nes_ppu();
        let mem = NESMemory::new_nes_mem(ppu);

        Console {
            sdl_ctx: sdl_context,
            canvas:  canvas,
            cpu:     CPU::new_nes_cpu(mem),
        }
    }

    pub fn insert_cartridge(&mut self, filename: String)
        -> Result<(), CartridgeError>
    {
        debug!("loading cartridge: {}", filename);
        let mut fh = File::open(filename).map_err(CartridgeError::IO)?;
        Cartridge::load_file_into_memory(&mut fh, &mut self.cpu.mem)?;
        Ok(())
    }

    pub fn power_up(&mut self) {
        debug!("powering up");

        self.cpu.init();

        let mut event_pump = self.sdl_ctx.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                      Event::Quit    { .. }
                    | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => { break 'running },
                    _ => {},
                }
            }

            let cpu_cycles = self.cpu.step();
            let ppu_cycles = cpu_cycles * 3;

            for _ in 1 .. ppu_cycles {
                let res = self.cpu.mem.ppu.step(&mut self.canvas);

                if res.vblank_nmi {
                    self.cpu.nmi()
                }
            }
        }

        info!("powering down");
    }
}
