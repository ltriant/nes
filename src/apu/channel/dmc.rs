// TODO
// This is incomplete, because I need to figure out how to let this module
// access VRAM from the mapper.

use crate::apu::channel::Voice;

const TIMER_TABLE: [u16; 16] = [
    0x01AC, 0x017C, 0x0154, 0x0140,
    0x011E, 0x00FE, 0x00E2, 0x00D6,
    0x00BE, 0x00A0, 0x008E, 0x0080,
    0x006A, 0x0054, 0x0048, 0x0036
];

pub struct DMC {
    pub enabled: bool,

    pub buffer: u8,

    timer_period: u16,
    //timer_value: u16,
}

impl Voice for DMC {
    fn signal(&self) -> u8 {
        if !self.enabled {
            return 0;
        }

        self.buffer
    }
}

impl DMC {
    pub fn new_dmc_channel() -> Self {
        Self {
            enabled: false,

            buffer: 0,

            timer_period: 0,
            //timer_value: 0,
        }
    }

    pub fn reset(&mut self) { }

    // Register $4010 sets the interrupt enable, loop, and timer period. If the
    // new interrupt enabled status is clear, the interrupt flag is cleared.
    //
    //     il-- ffff       interrupt enabled, loop, frequency index
    pub fn write_control(&mut self, val: u8) {
        let _irq      = (val & 0b1000_0000) != 0;
        let _dmc_loop = (val & 0b0100_0000) != 0;
        let f_index  =  val & 0b0000_1111;

        self.timer_period = TIMER_TABLE[f_index as usize];
    }

    // A write to $4011 sets the counter and DAC to a new value:
    //
    //     -ddd dddd       new DAC value
    pub fn write_dac(&mut self, _val: u8) { }

    pub fn write_address(&mut self, _val: u8) { }

    pub fn write_length(&mut self, _val: u8) { }
}
