pub struct Memory {
    ram: [u8; 0x800],
}

impl Memory {
    pub fn new_nes_mem() -> Memory {
        Memory {
            ram: [0; 0x800],
        }
    }

    pub fn read(&self, address: usize) -> Result<u8, &str> {
        match address {
            // The first 0x2000 bytes are RAM, but there's only 2KB (0x800) of
            // actual RAM, and the rest is just a mirror of the first 2KB.
            0 ... 0x1fff => Ok(self.ram[address % 0x800]),

            _ => Err("out of bounds"),
        }
    }

    pub fn write(&mut self, address: usize, val: u8) -> Result<u8, &str> {
        match address {
            0 ... 0x1fff => {
                self.ram[address % 0x800] = val;
                Ok(val)
            },

            _ => Err("out of bounds"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write() {
        let mut mem = Memory::new_nes_mem();
        assert_eq!(mem.read(0x1000), Ok(0));

        assert_eq!(mem.write(0x1000, 5), Ok(5));
        assert_eq!(mem.read(0x1000), Ok(5));

        assert_eq!(mem.write(0x2000, 1), Err("out of bounds"));
        assert_eq!(mem.read(0x2000), Err("out of bounds"));
    }
}
