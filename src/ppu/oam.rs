use crate::mem::Memory;

pub struct OAM {
    data: [u8; 0x100],
}

impl Memory for OAM {
    fn read(&mut self, address: u16) -> Result<u8, String> {
        Ok(self.data[address as usize % 0x100])
    }

    fn write(&mut self, address: u16, val: u8) -> Result<u8, String> {
        let i = address as usize % 0x100;

        // http://wiki.nesdev.com/w/index.php/PPU_OAM#Byte_2
        // The three unimplemented bits of each sprite's byte 2 do not exist in
        // the PPU and always read back as 0 on PPU revisions that allow reading
        // PPU OAM through OAMDATA ($2004).
        if i % 4 == 2 {
            let v = val & 0xe3;
            self.data[i] = v;
            Ok(v)
        }
        else {
            self.data[i] = val;
            Ok(val)
        }
    }
}

impl OAM {
    pub fn new_nes_oam() -> Self {
        Self {
            data: [0; 0x100],
        }
    }
}
