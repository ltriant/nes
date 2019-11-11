mod square;
mod triangle;
mod noise;
mod dmc;

pub use crate::apu::channel::square::SquareWave;
pub use crate::apu::channel::triangle::TriangleWave;
pub use crate::apu::channel::noise::Noise;
pub use crate::apu::channel::dmc::DMC;

pub trait Voice {
    fn signal(&self) -> u8;
}
