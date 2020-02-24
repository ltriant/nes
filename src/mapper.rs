mod mapper0;
mod mapper1;
mod mapper2;
mod mapper3;
mod mapper4;
mod mapper66;

use std::convert::From;
use std::io;
use std::fs::File;

pub use mapper0::Mapper0;
pub use mapper1::Mapper1;
pub use mapper2::Mapper2;
pub use mapper3::Mapper3;
pub use mapper4::Mapper4;
pub use mapper66::Mapper66;

#[derive(Clone, Copy)]
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
}

impl From<u8> for MirrorMode {
    fn from(mode: u8) -> Self {
        match mode {
            0 => MirrorMode::Horizontal,
            1 => MirrorMode::Vertical,
            2 => MirrorMode::Single0,
            3 => MirrorMode::Single1,
            4 => MirrorMode::Four,
            _ => panic!("bad mirror mode: {}", mode)
        }
    }
}

pub trait Mapper {
    fn mirror_mode(&self) -> &MirrorMode { &MirrorMode::Vertical }

    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, val: u8);

    fn irq_flag(&self) -> bool { false }
    fn signal_scanline(&mut self) {}

    fn save(&self, output: &mut File) -> io::Result<()>;
    fn load(&mut self, input: &mut File) -> io::Result<()>;
}


//
// This is an empty mapper that implements the Mapper trait, because I need to
// initialise the mapper to _something_ when I create the Console object.
//

pub struct MapperEmpty;
impl Mapper for MapperEmpty {
    fn read(&mut self, _address: u16) -> u8 { 0 }
    fn write(&mut self, _address: u16, _val: u8) { }
    fn save(&self, _output: &mut File) -> io::Result<()> { Ok(()) }
    fn load(&mut self, _input: &mut File) -> io::Result<()> { Ok(()) }
}

