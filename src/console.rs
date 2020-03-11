use std::cell::RefCell;
use std::env;
use std::fs;
use std::fs::File;
use std::process;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};

use crate::apu::APU;
use crate::controller::Controller;
use crate::cpu::CPU;
use crate::mapper::{Mapper, MapperEvent};
use crate::mem::{Memory, NESMemory};
use crate::ppu::PPU;
use crate::ines::CartridgeError;
use crate::ines;
use crate::serde::Storeable;

use sdl2::audio::AudioSpecDesired;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

lazy_static!{
    pub static ref NES_CPU_DEBUG: bool = match env::var("NES_CPU_DEBUG") {
        Ok(val) => val != "" && val != "0",
        Err(_)  => false,
    };

    pub static ref NES_CPU_NESTEST: bool = match env::var("NES_CPU_NESTEST") {
        Ok(val) => val != "" && val != "0",
        Err(_)  => false,
    };

    pub static ref NES_PPU_DEBUG: bool = match env::var("NES_PPU_DEBUG") {
        Ok(val) => val != "" && val != "0",
        Err(_)  => false,
    };

    pub static ref NES_APU_CHANNELS: u8 = match env::var("NES_APU_CHANNELS") {
        Ok(val) => val.parse().expect("invalid NES_APU_CHANNELS value"),
        Err(_)  => std::u8::MAX,
    };
}

const NES_FPS: f64 = 60.0;
const FRAME_DURATION: Duration = Duration::from_millis(((1.0 / NES_FPS) * 1000.0) as u64);

// The queue is full of f32s, and we want to maintain roughly 16384 samples in
// the queue at all times, so 4 * 16384 is the goal size.
const AUDIO_QUEUE_HIGH_WATER_MARK: u32 = 4 * 16384;

pub struct Console {
    // NES components
    cpu:        CPU,
    ppu:        Rc<RefCell<PPU>>,
    apu:        Rc<RefCell<APU>>,
    cartridge:  Rc<RefCell<Box<dyn Mapper>>>,
    controller: Rc<RefCell<Controller>>,

    // The absolute path on disk to save state to
    save_path:  String,
}

impl Console {
    pub fn new_nes_console(rom_path: &String) -> Result<Self, CartridgeError> {
        let full_path = fs::canonicalize(rom_path).map_err(CartridgeError::IO)?;
        info!("loading cartridge: {}", full_path.display());
        let basename_path = full_path.file_name().unwrap().to_str().unwrap();
        let save_path = format!("{:x}.data", md5::compute(basename_path)).into();

        let mut fh = File::open(full_path).map_err(CartridgeError::IO)?;
        let cartridge = ines::load_file_into_memory(&mut fh)?;

        let ppu = Rc::new(RefCell::new(PPU::new_nes_ppu(cartridge.clone())));
        let apu = Rc::new(RefCell::new(APU::new_nes_apu()));
        let controller = Rc::new(RefCell::new(Controller::new_controller()));
        let mem = NESMemory::new_nes_mem(
            ppu.clone(),
            apu.clone(),
            controller.clone()
        );

        Ok(Self {
            cpu:        CPU::new_nes_cpu(mem),
            ppu:        ppu,
            apu:        apu,
            cartridge:  cartridge,
            controller: controller,
            save_path:  save_path,
        })
    }

    // Reads a null-terminated string starting at `addr'
    fn read_string(&mut self, addr: u16) -> String {
        let mut addr = addr;

        let mut rv = String::new();

        loop {
            let b = self.cpu.read(addr);

            if b == 0 {
                break;
            }

            rv.push(b as char);

            addr += 1;
        }

        rv
    }

    // Detects if we're running a instr_test-v5 rom, and if so, it will output
    // the test results.
    fn debug_tests(&mut self) {
        let a = self.cpu.read(0x6001);
        let b = self.cpu.read(0x6002);
        let c = self.cpu.read(0x6003);

        if a == 0xDE && b == 0xB0 && c == 0x61 {
            let result = self.cpu.read(0x6000);

            if result <= 0x7F {
                let result_string = self.read_string(0x6004);
                println!("{}", result_string);

                println!("Emulator test complete, final status: 0x{:02X}", result);
                process::exit(0);
            }
        }
    }

    fn save(&mut self) {
        let mut fh = File::create(&self.save_path).unwrap();
        self.cpu.save(&mut fh).expect("unable to save CPU state");
        self.ppu.borrow().save(&mut fh).expect("unable to save PPU state");
        self.apu.borrow().save(&mut fh).expect("unable to save APU state");
        println!("saved state to {}", self.save_path);
    }

    fn load(&mut self) {
        if let Ok(mut fh) = File::open(&self.save_path) {
            self.cpu.load(&mut fh).expect("unable to load CPU state");
            self.ppu.borrow_mut().load(&mut fh).expect("unable to load PPU state");
            //self.apu.borrow_mut().reset();
            //self.apu.borrow_mut().load(&mut fh).expect("unable to laod APU state");
            println!("loaded state from {}", self.save_path);
        }
    }

    pub fn power_up(&mut self) {
        info!("powering up");

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let mut width = 256 * 3;
        let height = 240 * 3;

        if *NES_PPU_DEBUG {
            // Make room for the two pattern tables, side by side
            width += 2 * 144 + 20;
        }

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

        let audio_subsystem = sdl_context.audio().unwrap();
        debug!("audio driver: {}", audio_subsystem.current_audio_driver());

        let desired_spec = AudioSpecDesired {
            freq:     Some(44_100),
            channels: Some(2),
            samples:  Some(1024),
        };
        let audio_device = audio_subsystem.open_queue(None, &desired_spec).unwrap();
        audio_device.resume();
        let mut samples = Vec::new();
        let mut audio_sampling = true;

        self.cpu.reset();

        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut fps_start = Instant::now();
        let mut paused = false;

        'running: loop {
            let mut poll_keyboard = false;
            self.debug_tests();

            if paused {
                poll_keyboard = true;
                thread::sleep(Duration::from_millis(200));
            } else {
                let cpu_cycles = self.cpu.step();
                let ppu_cycles = cpu_cycles * 3;
                let apu_cycles = cpu_cycles;

                self.cartridge.borrow_mut()
                    .notify(MapperEvent::CPUTick(cpu_cycles));

                let mut frame_finished = false;
                for _ in 0 .. ppu_cycles {
                    let res = self.ppu.borrow_mut().step(&mut canvas);

                    if self.cartridge.borrow().irq_flag() {
                        self.cpu.trigger_irq();
                    }

                    if res.trigger_nmi {
                        self.cpu.trigger_nmi();
                    }

                    if res.frame_finished {
                        frame_finished = true;
                    }
                }

                for _ in 0 .. apu_cycles {
                    let res = self.apu.borrow_mut().step();

                    if res.trigger_irq {
                        self.cpu.trigger_irq();
                    }

                    if let Some(signal) = res.signal {
                        if audio_sampling {
                            samples.push(signal);
                            samples.push(signal);
                        }
                    }
                }

                // Super basic dynamic sampling implementation.
                //
                // If the number of samples is too low, we'll end up with
                // crackling and popping because the audio backend is consuming
                // the samples faster than we can produce them, but if we have
                // too many samples, the audio will get more and more out of
                // sync with the video.
                //
                // We want to keep the audio queue full of samples, and we want
                // to maintain at roughly AUDIO_QUEUE_HIGH_WATER_MARK samples.
                // So if we've got more than that many in the queue, we stop
                // sampling, and if we drop below, we start sampling again.
                //
                // This is much better than past attempts, and only
                // occasionally results in some cracking and popping. I can
                // live with this for now :)
                if audio_sampling && audio_device.size() > AUDIO_QUEUE_HIGH_WATER_MARK {
                    audio_sampling = false;
                }

                if !audio_sampling && audio_device.size() < AUDIO_QUEUE_HIGH_WATER_MARK {
                    audio_sampling = true;
                }

                if frame_finished {
                    canvas.present();
                    audio_device.queue(&samples);
                    samples.clear();
                    audio_sampling = true;

                    if let Some(delay) = FRAME_DURATION.checked_sub(fps_start.elapsed()) {
                        thread::sleep(delay);
                    }

                    fps_start = Instant::now();

                    // Polling for events once per loop slows the emulator
                    // right the fuck down, so I've moved to when a frame has
                    // finished instead.
                    poll_keyboard = true;
                }

            }

            if poll_keyboard {
                // I feel like this shouldn't be so damned slow...
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } => { break 'running },

                        Event::KeyDown { keycode: Some(key), .. } => {
                            match key {
                                Keycode::W => { self.controller.borrow_mut().up(true) },
                                Keycode::A => { self.controller.borrow_mut().left(true) },
                                Keycode::S => { self.controller.borrow_mut().down(true) },
                                Keycode::D => { self.controller.borrow_mut().right(true) },

                                Keycode::Return => { self.controller.borrow_mut().start(true) },
                                Keycode::Space  => { self.controller.borrow_mut().select(true) },

                                Keycode::N => { self.controller.borrow_mut().a(true) },
                                Keycode::M => { self.controller.borrow_mut().b(true) },

                                Keycode::P => { paused = ! paused },

                                Keycode::F2  => { self.save() },
                                Keycode::F3  => { self.load() },

                                Keycode::F12 => {
                                    self.cpu.reset();
                                    self.apu.borrow_mut().reset();
                                },

                                _ => {},
                            }
                        },

                        Event::KeyUp { keycode: Some(key), .. } => {
                            match key {
                                Keycode::W => { self.controller.borrow_mut().up(false) },
                                Keycode::A => { self.controller.borrow_mut().left(false) },
                                Keycode::S => { self.controller.borrow_mut().down(false) },
                                Keycode::D => { self.controller.borrow_mut().right(false) },

                                Keycode::Return => { self.controller.borrow_mut().start(false) },
                                Keycode::Space  => { self.controller.borrow_mut().select(false) },

                                Keycode::N => { self.controller.borrow_mut().a(false) },
                                Keycode::M => { self.controller.borrow_mut().b(false) },

                                _ => {},
                            }
                        },

                        _ => {},
                    }
                }

            }
        }

        info!("powering down");
    }
}
