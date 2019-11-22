use crate::apu::channel::Voice;

const LENGTH_TABLE: [u8; 32] = [
    10, 254, 20, 2, 40, 4, 80, 6,
    160, 8, 60, 10, 14, 12, 26, 14,
    12, 16, 24, 18, 48, 20, 96, 22,
    192, 24, 72, 26, 16, 28, 32, 30,
];

const TIMER_TABLE: [u16; 16] = [
    0x004, 0x008, 0x010, 0x020,
    0x040, 0x060, 0x080, 0x0a0,
    0x0ca, 0x0fe, 0x17c, 0x1fc,
    0x2fa, 0x3f8, 0x7f2, 0xfe4,
];

enum ShiftRegisterMode {
    One,
    Six,
}

pub struct Noise {
    pub enabled: bool,
    mode: ShiftRegisterMode,

    length_enabled: bool,
    pub length_value: u8,

    envelope_enabled: bool,
    envelope_start: bool,
    envelope_loop: bool,
    envelope_volume: u8,
    envelope_period: u8,
    envelope_value: u8,
    constant_volume: u8,

    timer_period: u16,
    timer_value: u16,

    shift_register: u16,
}

impl Voice for Noise {
    fn signal(&self) -> u8 {
        if !self.enabled {
            return 0;
        }

        if self.length_value == 0 {
            return 0;
        }

        // When bit 0 of the shift register is set, the DAC receives 0.
        if self.shift_register & 1 == 1 {
            return 0;
        }

        if self.envelope_enabled {
            self.envelope_volume
        }
        else {
            self.constant_volume
        }
    }
}

impl Noise {
    pub fn new_noise_channel() -> Self {
        Self {
            enabled: false,
            mode: ShiftRegisterMode::One,

            length_enabled: false,
            length_value: 0,

            envelope_enabled: false,
            envelope_start: false,
            envelope_loop: false,
            envelope_volume: 0,
            envelope_period: 0,
            envelope_value: 0,
            constant_volume: 0,

            timer_period: 0,
            timer_value: 0,

            // On power-up, the shift register is loaded with the value 1.
            shift_register: 1,
        }
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.mode = ShiftRegisterMode::One;
        self.length_enabled = false;
        self.length_value = 0;
        self.envelope_enabled = false;
        self.envelope_start = false;
        self.envelope_loop = false;
        self.envelope_volume = 0;
        self.envelope_period = 0;
        self.envelope_value = 0;
        self.constant_volume = 0;
        self.timer_period = 0;
        self.timer_value = 0;
        self.shift_register = 1;
    }

    pub fn step_envelope(&mut self) {
        if self.envelope_start {
            self.envelope_volume = 15;
            // The divider's period is set to n + 1.
            self.envelope_value = self.envelope_period + 1;
            self.envelope_start = false;
        }
        else if self.envelope_value > 0 {
            self.envelope_value -= 1;
        }
        else {
            if self.envelope_volume > 0 {
                self.envelope_volume -= 1;
            }
            else if self.envelope_loop {
                self.envelope_volume = 15;
            }

            self.envelope_value = self.envelope_period + 1;
        }
    }

    pub fn step_length(&mut self) {
        if self.length_enabled && self.length_value > 0 {
            self.length_value -= 1;
        }
    }

    pub fn step_timer(&mut self) {
        if self.timer_value == 0 {
            self.timer_value = self.timer_period;

            let shift = match self.mode {
                ShiftRegisterMode::One => 1,
                ShiftRegisterMode::Six => 6,
            };

            // When the timer clocks the shift register, the following actions
            // occur in order:
            //
            // 1. Feedback is calculated as the exclusive-OR of bit 0 and one
            //    other bit: bit 6 if Mode flag is set, otherwise bit 1.
            // 2. The shift register is shifted right by one bit.
            // 3. Bit 14, the leftmost bit, is set to the feedback calculated
            //    earlier.
            let feedback = (self.shift_register & 1) ^ ((self.shift_register >> shift) & 1);
            self.shift_register >>= 1;
            self.shift_register |= feedback << 14;
        }
        else {
            self.timer_value -= 1;
        }
    }

    // $400c
    pub fn write_control(&mut self, val: u8) {
        // --le nnnn   loop env/disable length, env disable, vol/env period
        self.envelope_loop    = (val & 0b0010_0000) != 0;
        self.length_enabled   = (val & 0b0010_0000) == 0;
        self.envelope_enabled = (val & 0b0001_0000) == 0;
        self.envelope_period  =  val & 0b0000_1111;
        self.constant_volume  =  val & 0b0000_1111;
        self.envelope_start   = true;
    }

    // $400e
    //
    // The noise channel and DMC use lookup tables to set the timer's period.
    pub fn write_mode(&mut self, val: u8) {
        // s--- pppp   short mode, period index
        self.mode = if (val & 0x80) != 0 {
            ShiftRegisterMode::Six
        }
        else {
            ShiftRegisterMode::One
        };

        let period_index = val & 0b0000_1111;
        self.timer_period = TIMER_TABLE[period_index as usize];
    }

    // $400f
    pub fn write_length_index(&mut self, val: u8) {
        // llll l---   length index
        let length_index = val >> 3;
        self.length_value = LENGTH_TABLE[length_index as usize];
    }
}
