use crate::ppu::PPU;
use crate::controller::Controller;

pub trait Memory {
    fn read(&mut self, address: u16) -> Result<u8, String>;
    fn write(&mut self, address: u16, val: u8) -> Result<u8, String>;
}

pub struct NESMemory {
    pub ppu: PPU,
    pub controller: Controller,
    pub ram: [u8; 0x800],
    rom: Vec<u8>,
    sram: [u8; 0x2000],
}

impl Memory for NESMemory {
    fn read(&mut self, address: u16) -> Result<u8, String> {
        match address {
            // The first 0x2000 bytes are RAM, but there's only 2KB (0x800) of
            // actual RAM, and the rest is just a mirror of the first 2KB.
            0x0000 ... 0x1fff => Ok(self.ram[address as usize % 0x800]),

            // The PPU registers exist from 0x2000 to 0x2007, the rest of the
            // address space is just a mirror of these first eight bytes.
            0x2000 ... 0x3fff => self.ppu.read(address),

            // APU pulses
            0x4000 ... 0x4007 => Ok(0),

            // APU triangle
            0x4008 ... 0x400b => Ok(0),

            // APU noise
            0x400c ... 0x400f => Ok(0),

            // APU DMC
            0x4010 ... 0x4013 => Ok(0),

            // OAM DMA
            0x4014            => Ok(0),  // TODO is this right?

            // APU sound channel
            0x4015            => Ok(0),

            // Controller 1
            0x4016            => self.controller.read(address),

            // Controller 2
            0x4017            => Ok(0),

            // Expansion ROM
            //0x4000 ... 0x5fff => Ok(0),

            // SRAM
            0x6000 ... 0x7fff => Ok(self.rom[address as usize % 0x2000]),

            // PRG-ROM
            0x8000 ... 0xffff => Ok(self.rom[address as usize % self.rom.len()]),

            _ => Err(format!("read out of bounds 0x{:04X}", address)),
        }
    }

    fn write(&mut self, address: u16, val: u8) -> Result<u8, String> {
        match address {
            // See comments in read() for explanations of the address ranges
            0 ... 0x1fff => {
                self.ram[(address as usize) % 0x800] = val;
                Ok(val)
            },

            0x2000 ... 0x3fff => self.ppu.write(address, val),

            // APU pulses
            0x4000 ... 0x4007 => Ok(0),

            // APU triangle
            0x4008 ... 0x400b => Ok(0),

            // APU noise
            0x400c ... 0x400f => Ok(0),

            // APU DMC
            0x4010 ... 0x4013 => Ok(0),

            // OAM DMA
            0x4014            => self.dma(val),

            // APU sound channel
            0x4015            => Ok(0),

            // Controller 1
            0x4016            => self.controller.write(address, val),

            // Controller 2
            0x4017            => Ok(0),

            // SRAM
            0x6000 ... 0x7fff => {
                self.sram[(address as usize) % 0x2000] = val;
                Ok(val)
            },

            0x8000 ... 0xffff => Err(String::from("cannot write to ROM")),

            _ => Err(format!("write out of bounds 0x{:04X}", address)),
        }
    }
}

impl NESMemory {
    pub fn new_nes_mem(ppu: PPU, controller: Controller) -> Self {
        Self {
            ppu: ppu,
            controller: controller,
            ram: [0; 0x800],
            rom: vec![],
            sram: [0; 0x2000],
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        self.rom.clone_from(data)
    }

    fn dma(&mut self, val: u8) -> Result<u8, String> {
        let addr_base = (val as u16) << 8;

        for lo_nyb in 0x00 ..= 0xff {
            let addr = addr_base | lo_nyb;
            let val = self.read(addr)?;
            self.ppu.write(0x2004, val)?;
        }

        Ok(val)
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
        assert_eq!(mem.write(0x8000, 1), Err(String::from("cannot write to ROM")));
        assert_eq!(mem.write(0xffff, 1), Err(String::from("cannot write to ROM")));
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
