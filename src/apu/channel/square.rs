use crate::apu::channel::Voice;

const LENGTH_TABLE: [u8; 32] = [
    10, 254, 20, 2, 40, 4, 80, 6,
    160, 8, 60, 10, 14, 12, 26, 14,
    12, 16, 24, 18, 48, 20, 96, 22,
    192, 24, 72, 26, 16, 28, 32, 30,
];

const DUTY_TABLE: [[u8; 8]; 4] = [
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 1, 1, 1],
];

pub struct SquareWave {
    pub enabled: bool,
    channel: u8,

    // TODO move length unit into submodule
    length_enabled: bool,
    pub length_value: u8,

    // TODO move timer and duty into submodule
    timer_period: u16,
    timer_value: u16,
    duty_mode: u8,
    duty_value: u8,

    // TODO move the sweep unit into a submodule
    sweep_enabled: bool,
    sweep_negate: bool,
    sweep_reload: bool,
    sweep_period: u8,
    sweep_shift: u8,
    sweep_value: u8,

    // TODO move the envelope unit into a submodule
    envelope_enabled: bool,
    envelope_start: bool,
    envelope_loop: bool,
    envelope_volume: u8,
    envelope_period: u8,
    envelope_value: u8,
    constant_volume: u8,
}

impl Voice for SquareWave {
    fn signal(&self) -> u8 {
        // The mixer receives the current envelope volume except when

        // The sequencer output is zero, or
        if !self.enabled {
            return 0;
        }

        // overflow from the sweep unit's adder is silencing the channel, or
        if self.timer_period > 0x7ff {
            return 0;
        }

        // the length counter is zero, or
        if self.length_value == 0 {
            return 0;
        }

        // the timer has a value less than eight.
        if self.timer_period < 8 {
            return 0;
        }

        if DUTY_TABLE[self.duty_mode as usize][self.duty_value as usize] == 0 {
            return 0;
        }

        if self.envelope_enabled {
            return self.envelope_volume;
        }
        else {
            return self.constant_volume;
        }
    }
}

impl SquareWave {
    pub fn new_square_wave(channel: u8) -> Self {
        Self {
            enabled: false,
            channel: channel,

            length_enabled: false,
            length_value: 0,

            timer_period: 0,
            timer_value: 0,
            duty_mode: 0,
            duty_value: 0,

            sweep_enabled: false,
            sweep_negate: false,
            sweep_reload: false,
            sweep_period: 0,
            sweep_shift: 0,
            sweep_value: 0,

            envelope_enabled: false,
            envelope_start: false,
            envelope_loop: false,
            envelope_volume: 0,
            envelope_period: 0,
            envelope_value: 0,
            constant_volume: 0,
        }
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.length_enabled = false;
        self.length_value = 0;
        self.timer_period = 0;
        self.timer_value = 0;
        self.duty_mode = 0;
        self.duty_value = 0;
        self.sweep_enabled = false;
        self.sweep_negate = false;
        self.sweep_reload = false;
        self.sweep_period = 0;
        self.sweep_shift = 0;
        self.sweep_value = 0;
        self.envelope_enabled = false;
        self.envelope_start = false;
        self.envelope_loop = false;
        self.envelope_volume = 0;
        self.envelope_period = 0;
        self.envelope_value = 0;
        self.constant_volume = 0;
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

    fn sweep(&mut self) {
        let delta = self.timer_period >> self.sweep_shift;

        if self.sweep_negate {
            self.timer_period -= delta;

            if self.channel == 1 {
                self.timer_period -= 1;
            }
        }
        else {
            self.timer_period += delta;
        }
    }

    pub fn step_sweep(&mut self) {
        if self.sweep_reload {
            if self.sweep_enabled && self.sweep_value == 0 {
                self.sweep();
            }

            self.sweep_value = self.sweep_period + 1;
            self.sweep_reload = false;
        }
        else if self.sweep_value > 0 {
            self.sweep_value -= 1;
        }
        else {
            if self.sweep_enabled {
                self.sweep();
            }

            self.sweep_value = self.sweep_period + 1;
        }
    }

    pub fn step_length(&mut self) {
        if self.length_enabled && self.length_value > 0 {
            self.length_value -= 1;
        }
    }

    pub fn step_timer(&mut self) {
        if self.timer_value == 0 {
            self.timer_value = self.timer_period + 1;
            self.duty_value = (self.duty_value + 1) % 8;
        }
        else {
            self.timer_value -= 1;
        }
    }

    // A channel's first register controls the envelope:
    //
    //     ddle nnnn   duty, loop env/disable length, env disable, vol/env period
    //
    // Note that the bit position for the loop flag is also mapped to a flag in
    // the Length Counter.
    //
    // The divider's period is set to n + 1.
    //
    // In addition to the envelope, the first register controls the duty cycle of the
    // square wave, without resetting the position of the sequencer:
    pub fn write_control(&mut self, val: u8) {
        self.duty_mode        = (val & 0b1100_0000) >> 6;
        self.envelope_loop    = (val & 0b0010_0000) != 0;
        self.length_enabled   = (val & 0b0010_0000) == 0;
        self.envelope_enabled = (val & 0b0001_0000) == 0;
        self.envelope_period  =  val & 0b0000_1111;
        self.constant_volume  =  val & 0b0000_1111;
        self.envelope_start   = true;
    }

    // A channel's second register configures the sweep unit:
    //
    //     eppp nsss   enable sweep, period, negative, shift
    //
    // The divider's period is set to p + 1.
    pub fn write_sweep(&mut self, val: u8) {
        self.sweep_enabled = (val & 0b1000_0000) != 0;
        self.sweep_period  = (val & 0b0111_0000) >> 4;
        self.sweep_negate  = (val & 0b0000_1000) != 0;
        self.sweep_shift   =  val & 0b0000_0111;

        self.sweep_reload = true;
    }

    // For the square and triangle channels, the third and fourth registers form
    // an 11-bit value and the divider's period is set to this value *plus one*.
    //
    // We add this *plus one* to the period in the step function.
    pub fn write_timer_low(&mut self, val: u8) {
        // pppp pppp   period low
        self.timer_period = (self.timer_period & 0xff00) | val as u16;
    }

    pub fn write_timer_high(&mut self, val: u8) {
        // llll lppp   length index, period high
        let length_index = (val & 0b1111_1000) >> 3;
        let period_high  = (val & 0b0000_0111) as u16;

        self.length_value = LENGTH_TABLE[length_index as usize];
        self.timer_period = (self.timer_period & 0x00ff) | (period_high << 8);
        self.envelope_start = true;
        self.duty_value = 0;
    }
}
