use std::f32::consts::PI;

pub trait Filter {
    fn process(&mut self, signal: f32) -> f32;
}

pub struct LowPassFilter {
    b0: f32,
    b1: f32,
    a1: f32,
    prev_x: f32,
    prev_y: f32,
}

impl Filter for LowPassFilter {
    fn process(&mut self, signal: f32) -> f32 {
        let y = self.b0 * signal + self.b1 * self.prev_x - self.a1 * self.prev_y;
        self.prev_y = y;
        self.prev_x = signal;
        y
    }
}

impl LowPassFilter {
    pub fn new_filter(sample_rate: f32, cutoff: f32) -> Self {
        let c = sample_rate / PI / cutoff;
        let a0i = 1.0 / (1.0 + c);

        Self {
            b0: a0i,
            b1: a0i,
            a1: (1.0 - c) * a0i,
            prev_x: 0.0,
            prev_y: 0.0,
        }
    }
}

pub struct HighPassFilter {
    b0: f32,
    b1: f32,
    a1: f32,
    prev_x: f32,
    prev_y: f32,
}

impl Filter for HighPassFilter {
    fn process(&mut self, signal: f32) -> f32 {
        let y = self.b0 * signal + self.b1 * self.prev_x - self.a1 * self.prev_y;
        self.prev_y = y;
        self.prev_x = signal;
        y
    }
}

impl HighPassFilter {
    pub fn new_filter(sample_rate: f32, cutoff: f32) -> Self {
        let c = sample_rate / PI / cutoff;
        let a0i = 1.0 / (1.0 + c);

        Self {
            b0: c * a0i,
            b1: -c * a0i,
            a1: (1.0 - c) * a0i,
            prev_x: 0.0,
            prev_y: 0.0,
        }
    }
}
