pub struct Memory {
    pub ram: [u8; 0x800],
    rom: Vec<u8>,
}

impl Memory {
    pub fn new_nes_mem() -> Memory {
        Memory {
            ram: [0; 0x800],
            rom: vec![],
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        self.rom.clone_from(data)
    }

    pub fn read(&self, address: u16) -> Result<u8, String> {
        match address {
            // The first 0x2000 bytes are RAM, but there's only 2KB (0x800) of
            // actual RAM, and the rest is just a mirror of the first 2KB.
            0 ... 0x1fff => Ok(self.ram[address as usize % 0x800]),

            // IO registers
            // 0x2000 ... 0x401f

            // Expansion ROM
            // 0x4020 ... 0x5fff

            // SRAM
            // 0x6000 ... 0x7fff

            // PRG-ROM
            // TODO this will depend on which mapper is being used
            0x8000 ... 0xffff => Ok(self.rom[address as usize % self.rom.len()]),

            _ => Err(format!("out of bounds 0x{:04X}", address)),
        }
    }

    pub fn write(&mut self, address: u16, val: u8) -> Result<u8, String> {
        match address {
            // See comments in read() for explanations of the address ranges
            0 ... 0x1fff => {
                self.ram[(address as usize) % 0x800] = val;
                Ok(val)
            },

            0x8000 ... 0xffff => Err(String::from("cannot write to ROM")),

            _ => Err(format!("out of bounds 0x{:04X}", address)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write() {
        let mut mem = Memory::new_nes_mem();

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
        let mut mem = Memory::new_nes_mem();
        mem.load_rom(&vec![0; 0x8000]);
        assert_eq!(mem.read(0x8000), Ok(0));
        assert_eq!(mem.read(0xffff), Ok(0));
        mem.load_rom(&vec![1; 0x8000]);
        assert_eq!(mem.read(0x8000), Ok(1));
        assert_eq!(mem.read(0xffff), Ok(1));
    }
}
