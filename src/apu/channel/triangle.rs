use crate::apu::channel::Voice;

const LENGTH_TABLE: [u8; 32] = [
    10,  254, 20, 2,  40, 4,  80, 6,
    160, 8,   60, 10, 14, 12, 26, 14,
    12,  16,  24, 18, 48, 20, 96, 22,
    192, 24,  72, 26, 16, 28, 32, 30,
];

// These volume values form the triangle shape
const TRIANGLE_WAVEFORM: [u8; 32] = [
    15, 14, 13, 12, 11, 10, 9,  8,
    7,  6,  5,  4,  3,  2,  1,  0,
    0,  1,  2,  3,  4,  5,  6,  7,
    8,  9,  10, 11, 12, 13, 14, 15,
];

pub struct TriangleWave {
    pub enabled: bool,

    length_enabled: bool,
    pub length_value: u8,

    counter_reload: bool,
    counter_period: u8,
    counter_value: u8,

    timer_value: u16,
    timer_period: u16,
    duty_value: u8,
}

impl Voice for TriangleWave {
    fn signal(&self) -> u8 {
        if !self.enabled {
            return 0;
        }

        if self.length_value == 0 {
            return 0;
        }

        if self.counter_value == 0 {
            return 0;
        }

        TRIANGLE_WAVEFORM[self.duty_value as usize]
    }
}

impl TriangleWave {
    pub fn new_triangle_wave() -> Self {
        Self {
            enabled: false,

            length_enabled: false,
            length_value: 0,

            counter_reload: false,
            counter_period: 0,
            counter_value: 0,

            timer_value: 0,
            timer_period: 0,
            duty_value: 0,
        }
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.length_enabled = false;
        self.length_value = 0;
        self.counter_reload = false;
        self.counter_period = 0;
        self.counter_value = 0;
        self.timer_value = 0;
        self.timer_period = 0;
        self.duty_value = 0;
    }

    pub fn step_length(&mut self) {
        if self.length_enabled && self.length_value > 0 {
            self.length_value -= 1;
        }
    }

    pub fn step_timer(&mut self) {
        if self.timer_value == 0 {
            self.timer_value = self.timer_period + 1;

            if self.length_value > 0 && self.counter_value > 0 {
                self.duty_value = (self.duty_value + 1) % 32;
            }
        }
        else {
            self.timer_value -= 1;
        }
    }

    pub fn step_counter(&mut self) {
        if self.counter_reload {
            self.counter_value = self.counter_period;
        }
        else if self.counter_value > 0 {
            self.counter_value -= 1;
        }

        if self.length_enabled {
            self.counter_reload = false;
        }
    }

    // $4008
    pub fn write_control(&mut self, val: u8) {
        // clll llll   control, linear counter load
        self.length_enabled = (val & 0b1000_0000) != 0;
        self.counter_period =  val & 0b0111_1111;
    }

    // $400a
    //
    // For the square and triangle channels, the third and fourth registers form
    // an 11-bit value and the divider's period is set to this value *plus one*.
    //
    // We add this *plus one* to the period in the step function.
    pub fn write_timer_low(&mut self, val: u8) {
        // pppp pppp   period low
        self.timer_period = (self.timer_period & 0xff00) | val as u16;
    }

    // $400b
    pub fn write_timer_high(&mut self, val: u8) {
        // llll lppp   length index, period high
        let length_index = (val & 0b1111_1000) >> 3;
        let period_high  = (val & 0b0000_0111) as u16;

        self.length_value = LENGTH_TABLE[length_index as usize];
        self.timer_period = (self.timer_period & 0x00ff) | (period_high << 8);
        self.counter_reload = true;
    }
}
