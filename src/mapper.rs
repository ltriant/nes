mod mapper0;
mod mapper1;
mod mapper2;
mod mapper3;
mod mapper4;
mod mapper7;
mod mapper34;
mod mapper66;
mod mapper69;

use std::io;
use std::fs::File;

pub use mapper0::Mapper0;
pub use mapper1::Mapper1;
pub use mapper2::Mapper2;
pub use mapper3::Mapper3;
pub use mapper4::Mapper4;
pub use mapper7::Mapper7;
pub use mapper34::Mapper34;
pub use mapper66::Mapper66;
pub use mapper69::Mapper69;

#[derive(Clone, Copy, Debug)]
pub enum MirrorMode {
    Horizontal = 0,
    Vertical   = 1,
    Single0    = 2,
    Single1    = 3,
    Four       = 4,
}

impl MirrorMode {
    pub fn coefficients(&self) -> Vec<usize> {
        match *self {
            MirrorMode::Horizontal => vec![0, 0, 1, 1],
            MirrorMode::Vertical   => vec![0, 1, 0, 1],
            MirrorMode::Single0    => vec![0, 0, 0, 0],
            MirrorMode::Single1    => vec![1, 1, 1, 1],
            MirrorMode::Four       => vec![0, 1, 2, 3],
        }
    }

    pub fn from_hv01(mode: u8) -> Self {
        match mode {
            0 => MirrorMode::Horizontal,
            1 => MirrorMode::Vertical,
            2 => MirrorMode::Single0,
            3 => MirrorMode::Single1,
            4 => MirrorMode::Four,
            _ => panic!("bad mirror mode: {}", mode)
        }
    }

    pub fn from_vh01(mode: u8) -> Self {
        match mode {
            0 => MirrorMode::Vertical,
            1 => MirrorMode::Horizontal,
            2 => MirrorMode::Single0,
            3 => MirrorMode::Single1,
            4 => MirrorMode::Four,
            _ => panic!("bad mirror mode: {}", mode)
        }
    }
}

pub enum MapperEvent {
    CPUTick(u64),
    HBlank,
    VRAMAddressChange(u16),
}

pub trait Mapper {
    // The mirroring mode to use
    fn mirror_mode(&self) -> &MirrorMode { &MirrorMode::Vertical }

    // Memory read/write
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, val: u8);

    // Called after every PPU execution, to determine whether or not an
    // interrupt should be raised.
    fn irq_flag(&self) -> bool { false }

    // Called on particular events, resulting in an observer-like pattern.
    fn notify(&mut self, _event: MapperEvent) { }

    // Serialisation and deserialisation to save states
    fn save(&self, output: &mut File) -> io::Result<()>;
    fn load(&mut self, input: &mut File) -> io::Result<()>;
}
