pub struct PPUStatus(pub u8);

impl PPUStatus {
    // Vertical blank status
    #[allow(dead_code)]
    pub fn vblank_started(&self) -> bool {
        let &PPUStatus(val) = self;
        (val & 0x80) != 0
    }

    #[allow(dead_code)]
    pub fn set_vblank(&mut self) {
        let PPUStatus(old) = *self;
        *self = PPUStatus(old | 0x80);
    }

    #[allow(dead_code)]
    pub fn clear_vblank(&mut self) {
        let PPUStatus(old) = *self;
        *self = PPUStatus(old & !0x80);
    }

    // Sprite zero hit status
    #[allow(dead_code)]
    pub fn sprite_zero_hit(&self) -> bool {
        let &PPUStatus(val) = self;
        (val & 0x40) != 0
    }

    pub fn set_sprite_zero_hit(&mut self) {
        let PPUStatus(old) = *self;
        *self = PPUStatus(old | 0x40);
    }

    pub fn clear_sprite_zero_hit(&mut self) {
        let PPUStatus(old) = *self;
        *self = PPUStatus(old & !0x40);
    }

    // Sprite overflow status
    #[allow(dead_code)]
    pub fn sprite_overflow(&self) -> bool {
        let &PPUStatus(val) = self;
        (val & 0x20) != 0
    }

    pub fn set_sprite_overflow(&mut self) {
        let PPUStatus(old) = *self;
        *self = PPUStatus(old | 0x20);
    }

    pub fn clear_sprite_overflow(&mut self) {
        let PPUStatus(old) = *self;
        *self = PPUStatus(old & !0x20);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vblank() {
        let status = PPUStatus(0b10000000);
        assert!(status.vblank_started());

        let mut status = PPUStatus(0x00);
        assert!(!status.vblank_started());
        status.set_vblank();
        assert!(status.vblank_started());
        status.clear_vblank();
        assert!(!status.vblank_started());
    }

    #[test]
    fn test_sprite_zero() {
        let status = PPUStatus(0b01000000);
        assert!(status.sprite_zero_hit());

        let mut status = PPUStatus(0x00);
        assert!(!status.sprite_zero_hit());
        status.set_sprite_zero_hit();
        assert!(status.sprite_zero_hit());
        status.clear_sprite_zero_hit();
        assert!(!status.sprite_zero_hit());
    }

    #[test]
    fn test_sprite_overflow() {
        let status = PPUStatus(0b00100000);
        assert!(status.sprite_overflow());

        let mut status = PPUStatus(0x00);
        assert!(!status.sprite_overflow());
        status.set_sprite_overflow();
        assert!(status.sprite_overflow());
        status.clear_sprite_overflow();
        assert!(!status.sprite_overflow());
    }
}
