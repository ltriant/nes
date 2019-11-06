mod channel;
mod filter;

use crate::apu::channel::{SquareWave, Voice};
use crate::apu::filter::{Filter, HighPassFilter, LowPassFilter};
use crate::mem::Memory;

lazy_static!{
    static ref PULSE_TABLE: Vec<f32> = (0 .. 31)
        .map(|i| 95.52 / (8128.0 / i as f32 + 100.0))
        .collect::<Vec<_>>();
}

pub struct APU {
    square1: SquareWave,
    square2: SquareWave,

    cycles: u64,

    frame_mode: bool, // false = 4-step, true = 5-step
    frame_value: u8,
    irq: bool, // true = generates IRQ on the last tick of a 4-step sequence

    filters: [Box<dyn Filter>; 3],
}

impl Memory for APU {
    fn read(&mut self, address: u16) -> u8 {
        match address {
            // Channel enable, length counter status
            0x4015 => self.read_status(),
            _      => 0,
        }
    }

    fn write(&mut self, address: u16, val: u8) {
        match address {
            // Square 1
            0x4000 => { self.square1.write_control(val) },
            0x4001 => { self.square1.write_sweep(val) },
            0x4002 => { self.square1.write_timer_low(val) },
            0x4003 => { self.square1.write_timer_high(val) },

            // Square 2
            0x4004 => { self.square2.write_control(val) },
            0x4005 => { self.square2.write_sweep(val) },
            0x4006 => { self.square2.write_timer_low(val) },
            0x4007 => { self.square2.write_timer_high(val) },

            // Triangle
            0x4008 ..= 0x400b => { },

            // Noise
            0x400c ..= 0x400f => { },

            // DMC
            0x4010 ..= 0x4013 => { },

            // Channel enable, length counter status
            0x4015            => { self.write_control(val) },

            // Frame counter
            0x4017            => { self.write_frame_counter(val) },

            _                 => panic!("bad APU address: 0x{:04X}", address),
        }
    }
}

pub struct StepResult {
    pub trigger_irq: bool,
    pub signal:      Option<f32>,
}

impl APU {
    pub fn new_nes_apu() -> Self {
        Self {
            square1: SquareWave::new_square_wave(1),
            square2: SquareWave::new_square_wave(2),

            cycles: 0,

            frame_mode: false,
            frame_value: 0,
            irq: false,

            // The NES hardware follows the DACs with a surprisingly involved
            // circuit that adds several low-pass and high-pass filters:
            //
            // * A first-order high-pass filter at 90 Hz
            // * Another first-order high-pass filter at 440 Hz
            // * A first-order low-pass filter at 14 kHz
            filters: [
                Box::new(HighPassFilter::new_filter(44_100.0, 90.0)),
                Box::new(HighPassFilter::new_filter(44_100.0, 440.0)),
                Box::new(LowPassFilter::new_filter(44_100.0, 14_000.0)),
            ],
        }
    }

    fn read_status(&mut self) -> u8 {
        let mut rv = 0;

        if self.square1.length_value > 0 {
            rv |= 1;
        }

        if self.square2.length_value > 0 {
            rv |= 2;
        }

        debug!("read_status: {:08b}", rv);

        rv
    }

    fn write_frame_counter(&mut self, val: u8) {
        // MI-- ----       mode, IRQ disable

        // Mode (0 = 4-step, 1 = 5-step)
        self.frame_mode = (val & 0b1000_0000) != 0;

        // IRQ inhibit flag. If this is set, we DON'T want to generate an IRQ.
        // Hello, double-negatives.
        self.irq = (val & 0b0100_0000) == 0;

        // If the mode flag is clear, the 4-step sequence is selected,
        // otherwise the 5-step sequence is selected and the sequencer is
        // immediately clocked once.
        if self.frame_mode {
            // TODO is this right?
            self.step_envelopes();
            self.step_sweeps();
            self.step_lengths();
        }
    }

    fn write_control(&mut self, val: u8) {
        self.square1.enabled = (val & 0b0000_0001) != 0;
        self.square2.enabled = (val & 0b0000_0010) != 0;

        if !self.square1.enabled {
            self.square1.length_value = 0;
        }

        if !self.square2.enabled {
            self.square2.length_value = 0;
        }
    }

    fn signal(&mut self) -> f32 {
        // Digital-to-Analog conversion

        let sq1 = self.square1.signal() as usize;
        let sq2 = self.square2.signal() as usize;

        let signal = PULSE_TABLE[sq1 + sq2];

        self.filters.iter_mut().fold(signal, |sig, filter| { filter.process(sig) } )
        //signal
    }

    fn step_envelopes(&mut self) {
        self.square1.step_envelope();
        self.square2.step_envelope();
    }

    fn step_sweeps(&mut self) {
        self.square1.step_sweep();
        self.square2.step_sweep();
    }

    fn step_lengths(&mut self) {
        self.square1.step_length();
        self.square2.step_length();
    }

    fn step_timers(&mut self) {
        self.square1.step_timer();
        self.square2.step_timer();
    }

    fn step_frame_counter(&mut self, res: &mut StepResult) {
        if self.frame_mode {
            self.frame_value = (self.frame_value + 1) % 5;

            // mode 1: 5-step
            // ---------------------------------------
            //     - - - - -   IRQ flag (never set)
            //     l - l - -   length counter + sweep
            //     e e e e -   envelope + linear counter
            match self.frame_value {
                0 | 2 => {
                    self.step_envelopes();
                    self.step_sweeps();
                    self.step_lengths();
                },
                1 | 3 => {
                    self.step_envelopes();
                },
                _ => { },
            }
        }
        else {
            self.frame_value = (self.frame_value + 1) % 4;

            // mode 0: 4-step
            // ---------------------------------------
            //     - - - f     IRQ flag
            //     - l - l     length counter + sweep
            //     e e e e     envelope + linear counter
            match self.frame_value {
                0 | 2 => {
                    self.step_envelopes();
                },
                1 => {
                    self.step_envelopes();
                    self.step_sweeps();
                    self.step_lengths();
                },
                3 => {
                    self.step_envelopes();
                    self.step_sweeps();
                    self.step_lengths();

                    if self.irq {
                        res.trigger_irq = true;
                    }
                },
                _ => { },
            }
        }
    }

    pub fn step(&mut self, cpu_cycles: u64) -> StepResult {
        let mut res = StepResult{
            trigger_irq: false,
            signal:      None,
        };

        let cycle1 = self.cycles as f32;
        self.cycles += 1;
        let cycle2 = self.cycles as f32;
        self.step_timers();
        
        let frame_counter_rate = 1789773.0 / 240.0;
        let f1 = (cycle1 / frame_counter_rate) as u32;
        let f2 = (cycle2 / frame_counter_rate) as u32;
        //if cpu_cycles % 2 == 1 {
        if f1 != f2 {
            self.step_frame_counter(&mut res);
        }

        let sample_rate = 1789773.0 / 44100.0;
        let s1 = (cycle1 / sample_rate) as u32;
        let s2 = (cycle2 / sample_rate) as u32;
        if self.cycles % 18 == 0 {  // TODO why 18?
        //if cpu_cycles % 40 == 0 {
        //if s1 != s2 {
            res.signal = Some(self.signal());
        }

        return res;
    }
}
