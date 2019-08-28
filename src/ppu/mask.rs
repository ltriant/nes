pub struct PPUMask(pub u8);

impl PPUMask {
    #[allow(dead_code)]
    pub fn emphasize_blue(&self) -> bool {
        let &PPUMask(val) = self;
        (val & 0x80) != 0
    }

    #[allow(dead_code)]
    pub fn emphasize_green(&self) -> bool {
        let &PPUMask(val) = self;
        (val & 0x40) != 0
    }

    #[allow(dead_code)]
    pub fn emphasize_red(&self) -> bool {
        let &PPUMask(val) = self;
        (val & 0x20) != 0
    }

    pub fn show_sprites(&self) -> bool {
        let &PPUMask(val) = self;
        (val & 0x10) != 0
    }

    pub fn show_background(&self) -> bool {
        let &PPUMask(val) = self;
        (val & 0x08) != 0
    }

    pub fn show_sprites_leftmost(&self) -> bool {
        let &PPUMask(val) = self;
        (val & 0x04) != 0
    }

    pub fn show_background_leftmost(&self) -> bool {
        let &PPUMask(val) = self;
        (val & 0x02) != 0
    }

    #[allow(dead_code)]
    pub fn greyscale(&self) -> bool {
        let &PPUMask(val) = self;
        (val & 0x01) != 0
    }
}
