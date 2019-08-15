use crate::mem::Memory;

pub struct PPUData {
    chr_rom: Vec<u8>,
    nametables: [u8; 0x800],
    palette: [u8; 0xff],
}

impl Memory for PPUData {
    fn read(&mut self, address: u16) -> Result<u8, String> {
        match address {
            0 ... 0x1fff => {
                Ok(self.chr_rom[address as usize])
            },
            0x2000 ... 0x3eff => {
                Ok(self.nametables[address as usize & 0x7ff])
            },
            0x3f00 ... 0x3fff => {
                Ok(self.palette[address as usize & 0x1f])
            },
            _ => Err(format!("out of bounds 0x{:04X}", address))
        }
    }

    fn write(&mut self, address: u16, val: u8) -> Result<u8, String> {
        match address {
            0 ... 0x1fff => {
                //debug!("writing to CHR-ROM");
                self.chr_rom[address as usize] = val;
                Ok(val)
            },
            0x2000 ... 0x3eff => {
                //debug!("writing to nametable");
                self.nametables[address as usize & 0x7ff] = val;
                Ok(val)
            },
            0x3f00 ... 0x3fff => {
                let i = address as usize & 0x1f;
                debug!("writing {:02X} to palette {}", val, i);

                self.palette[i] = val;
                Ok(val)
            },
            _ => Err(format!("out of bounds 0x{:04X}", address))
        }
    }
}

impl PPUData {
    pub fn new_ppu_data() -> PPUData {
        PPUData {
            chr_rom: vec![],
            nametables: [0; 0x800],
            palette: [0; 0xff],
        }
    }

    pub fn load_vrom(&mut self, data: &Vec<u8>) {
        self.chr_rom.clone_from(data);
    }
}
