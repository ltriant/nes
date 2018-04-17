use mem::Memory;

pub struct PPU;

impl Memory for PPU {
    fn read(&self, _address: u16) -> Result<u8, String> {
        Err(String::from("unimplemented"))
    }

    fn write(&mut self, _address: u16, _val: u8) -> Result<u8, String> {
        Err(String::from("unimplemented"))
    }
}

impl PPU {
    pub fn new_nes_ppu() -> PPU {
        PPU { }
    }
}
