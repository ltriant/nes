use std::io::{Read, Write};
use std::io;
use std::fs::File;

use crate::mem::Memory;

pub struct OAM {
    data: [u8; 0x100],
}

impl Memory for OAM {
    fn read(&mut self, address: u16) -> u8 {
        self.data[address as usize % 0x100]
    }

    fn write(&mut self, address: u16, val: u8) {
        let i = address as usize % 0x100;

        // http://wiki.nesdev.com/w/index.php/PPU_OAM#Byte_2
        // The three unimplemented bits of each sprite's byte 2 do not exist in
        // the PPU and always read back as 0 on PPU revisions that allow reading
        // PPU OAM through OAMDATA ($2004).
        if i % 4 == 2 {
            let v = val & 0xe3;
            self.data[i] = v;
        } else {
            self.data[i] = val;
        }
    }

    fn save(&self, output: &mut File) -> io::Result<()> {
        output.write(&self.data)?;
        Ok(())
    }

    fn load(&mut self, input: &mut File) -> io::Result<()> {
        input.read(&mut self.data)?;
        Ok(())
    }
}

impl OAM {
    pub fn new_nes_oam() -> Self {
        Self {
            data: [0; 0x100],
        }
    }
}
