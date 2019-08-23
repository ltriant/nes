use crate::mem::Memory;

pub struct OAM {
    data: [u8; 0x100],
}

impl Memory for OAM {
    fn read(&mut self, address: u16) -> Result<u8, String> {
        Ok(self.data[address as usize % 0x100])
    }

    fn write(&mut self, address: u16, val: u8) -> Result<u8, String> {
        self.data[address as usize % 0x100] = val;
        Ok(val)
    }
}

impl OAM {
    pub fn new_nes_oam() -> Self {
        Self {
            data: [0; 0x100],
        }
    }
}
