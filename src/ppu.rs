use mem::Memory;

struct PPUCtrl(u8);

enum SpriteSize {
    Small, // 8x8
    Large, // 8x16
}

impl PPUCtrl {
    fn generate_nmi(&self) -> bool {
        let PPUCtrl(val) = *self;

        (val & 0b10000000) != 0
    }

    fn sprite_size(&self) -> SpriteSize {
        let PPUCtrl(val) = *self;

        if (val & 0b00100000) == 0 {
            SpriteSize::Small
        }
        else {
            SpriteSize::Large
        }
    }

    fn background_pattern_table_addr(&self) -> u16 {
        let PPUCtrl(val) = *self;

        if (val & 0b00010000) == 0 {
            0x0000
        }
        else {
            0x1000
        }
    }

    fn sprite_pattern_table_addr(&self) -> u16 {
        let PPUCtrl(val) = *self;

        if (val & 0b00001000) == 0 {
            0x0000
        }
        else {
            0x1000
        }
    }

    fn vram_addr_increment(&self) -> u16 {
        let PPUCtrl(val) = *self;

        if (val & 0b00000100) == 0 {
            1
        }
        else {
            32
        }
    }

    fn base_nametable_addr(&self) -> u16 {
        let PPUCtrl(val) = *self;

        match val & 0b00000011 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2c00,
            _ => panic!("bad value {}", val)
        }
    }
}

pub struct PPU {
    ctrl: PPUCtrl,
}

impl Memory for PPU {
    fn read(&self, address: u16) -> Result<u8, String> {
        match address % 0x08 {
            0 => {
                let PPUCtrl(n) = self.ctrl;
                Ok(n)
            },
            _ => panic!("bad PPU address {}", address)
        }
    }

    fn write(&mut self, address: u16, val: u8) -> Result<u8, String> {
        match address % 0x08 {
            0 => {
                self.ctrl = PPUCtrl(val);
                Ok(val)
            },
            _ => panic!("bad PPU address {}", address)
        }
    }
}

impl PPU {
    pub fn new_nes_ppu() -> PPU {
        PPU {
            ctrl: PPUCtrl(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ppuctrl() {
        let ctrl = PPUCtrl(1);
        assert_eq!(ctrl.base_nametable_addr(), 0x2400);

        let ctrl = PPUCtrl(3);
        assert_eq!(ctrl.base_nametable_addr(), 0x2c00);

        let ctrl = PPUCtrl(5);
        assert_eq!(ctrl.base_nametable_addr(), 0x2400);
        assert_eq!(ctrl.vram_addr_increment(), 32);

        let ctrl = PPUCtrl(0xff);
        assert_eq!(ctrl.generate_nmi(), true);
    }
}
