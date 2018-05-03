pub struct PPUStatus(pub u8);

impl PPUStatus {
    pub fn vblank_started(&self) -> bool {
        let &PPUStatus(val) = self;
        (val & 0x80) != 0
    }

    pub fn sprite_zero_hit(&self) -> bool {
        let &PPUStatus(val) = self;
        (val & 0x40) != 0
    }

    pub fn sprite_overflow(&self) -> bool {
        let &PPUStatus(val) = self;
        (val & 0x20) != 0
    }
}
