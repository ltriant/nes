use mem::Memory;

pub struct OAM {
    data: [u8; 0x100],
}

impl Memory for OAM {
    fn read(&self, address: u16) -> Result<u8, String> {
        Ok(self.data[address as usize])
    }

    fn write(&mut self, address: u16, val: u8) -> Result<u8, String> {
        // TODO do I need to mod this address value?
        self.data[address as usize] = val;
        Ok(val)
    }
}

impl OAM {
    pub fn new_nes_oam() -> OAM {
        OAM {
            data: [0; 0x100],
        }
    }
}
