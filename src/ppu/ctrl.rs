pub struct PPUCtrl(pub u8);

enum SpriteSize {
    Small, // 8x8
    Large, // 8x16
}

impl PPUCtrl {
    pub fn generate_nmi(&self) -> bool {
        let PPUCtrl(val) = *self;

        (val & 0b10000000) != 0
    }

    pub fn sprite_size(&self) -> SpriteSize {
        let PPUCtrl(val) = *self;

        if (val & 0b00100000) == 0 {
            SpriteSize::Small
        }
        else {
            SpriteSize::Large
        }
    }

    pub fn background_pattern_table_addr(&self) -> u16 {
        let PPUCtrl(val) = *self;

        if (val & 0b00010000) == 0 {
            0x0000
        }
        else {
            0x1000
        }
    }

    pub fn sprite_pattern_table_addr(&self) -> u16 {
        let PPUCtrl(val) = *self;

        if (val & 0b00001000) == 0 {
            0x0000
        }
        else {
            0x1000
        }
    }

    pub fn vram_addr_increment(&self) -> u16 {
        let PPUCtrl(val) = *self;

        if (val & 0b00000100) == 0 {
            1
        }
        else {
            32
        }
    }

    pub fn base_nametable_addr(&self) -> u16 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_thing() {
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
