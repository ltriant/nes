use std::cell::RefCell;
use std::fs::File;
use std::io::{Read, Write};
use std::io;
use std::rc::Rc;

use crate::apu::APU;
use crate::controller::Controller;
use crate::ppu::PPU;

pub trait Memory {
    fn read(&mut self, _address: u16) -> u8 { 0 }
    fn write(&mut self, _address: u16, _val: u8) { }
    fn save(&self, _output: &mut File) -> io::Result<()> { Ok(()) }
    fn load(&mut self, _input: &mut File) -> io::Result<()> { Ok(()) }
}

pub struct NESMemory {
    ppu:        Rc<RefCell<PPU>>,
    apu:        Rc<RefCell<APU>>,
    controller: Rc<RefCell<Controller>>,
    ram:        [u8; 0x800],
}

impl Memory for NESMemory {
    fn read(&mut self, address: u16) -> u8 {
        match address {
            // The first 0x2000 bytes are RAM, but there's only 2KB (0x800) of
            // actual RAM, and the rest is just a mirror of the first 2KB.
            0x0000 ..= 0x1fff => self.ram[address as usize % 0x800],

            // The PPU registers exist from 0x2000 to 0x2007, the rest of the
            // address space is just a mirror of these first eight bytes.
            0x2000 ..= 0x3fff => self.ppu.borrow_mut().read(address),

            // APU registers
            0x4000 ..= 0x4013 => self.apu.borrow_mut().read(address),

            // OAM DMA
            0x4014            => 0,

            // APU registers
            0x4015            => self.apu.borrow_mut().read(address),

            // Controller 1
            0x4016            => self.controller.borrow_mut().read(address),

            // Controller 2
            0x4017            => 0,

            // Expansion ROM
            0x4020 ..= 0x5fff => 0,

            // SRAM
            0x6000 ..= 0x7fff => self.ppu.borrow_mut().data.mapper.borrow_mut().read(address),

            // PRG-ROM
            0x8000 ..= 0xffff => self.ppu.borrow_mut().data.mapper.borrow_mut().read(address),

            _ => unreachable!("read out of bounds 0x{:04X}", address),
        }
    }

    fn write(&mut self, address: u16, val: u8) {
        match address {
            // RAM
            0x0000 ..= 0x1fff => { self.ram[(address as usize) % 0x800] = val; },

            // PPU registers
            0x2000 ..= 0x3fff => self.ppu.borrow_mut().write(address, val),

            // APU registers
            0x4000 ..= 0x4013 => self.apu.borrow_mut().write(address, val),

            // OAM DMA
            0x4014            => unreachable!("this should've been intercepted by the CPU"),

            // APU registers
            0x4015            => self.apu.borrow_mut().write(address, val),

            // Controller 1
            0x4016            => self.controller.borrow_mut().write(address, val),

            // Controller 2
            0x4017            => { },

            // Expansion ROM
            0x4020 ..= 0x5fff => { },

            // SRAM
            0x6000 ..= 0x7fff => self.ppu.borrow_mut().data.mapper.borrow_mut().write(address, val),

            // PRG-ROM
            0x8000 ..= 0xffff => self.ppu.borrow_mut().data.mapper.borrow_mut().write(address, val),

            _ => unreachable!("write out of bounds 0x{:04X}", address),
        }
    }

    fn save(&self, output: &mut File) -> io::Result<()> {
        output.write(&self.ram)?;
        Ok(())
    }

    fn load(&mut self, input: &mut File) -> io::Result<()> {
        input.read(&mut self.ram)?;
        Ok(())
    }
}

impl NESMemory {
    pub fn new_nes_mem(ppu: Rc<RefCell<PPU>>,
                       apu: Rc<RefCell<APU>>,
                       controller: Rc<RefCell<Controller>>)
        -> Self
    {
        Self {
            ppu: ppu,
            apu: apu,
            controller: controller,
            ram: [0; 0x800],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write() {
        let ppu = PPU::new_nes_ppu();
        let ctrl = Controller::new_controller();
        let mut mem = NESMemory::new_nes_mem(ppu, ctrl);

        // RAM
        assert_eq!(mem.read(0x1000), Ok(0));
        assert_eq!(mem.write(0x1000, 5), Ok(5));
        assert_eq!(mem.read(0x1000), Ok(5));

        // ROM
        mem.load_rom(&vec![0; 0x8000]);
        assert_eq!(mem.read(0x8000), Ok(0));
        assert_eq!(mem.read(0x8001), Ok(0));
        assert_eq!(mem.read(0xffff), Ok(0));
    }

    #[test]
    fn test_load_rom() {
        let ppu = PPU::new_nes_ppu();
        let ctrl = Controller::new_controller();
        let mut mem = NESMemory::new_nes_mem(ppu, ctrl);
        mem.load_rom(&vec![0; 0x8000]);
        assert_eq!(mem.read(0x8000), Ok(0));
        assert_eq!(mem.read(0xffff), Ok(0));
        mem.load_rom(&vec![1; 0x8000]);
        assert_eq!(mem.read(0x8000), Ok(1));
        assert_eq!(mem.read(0xffff), Ok(1));
    }
}
