pub struct PPUStatus(pub u8);

impl PPUStatus {
    pub fn vblank_started(&self) -> bool {
        let &PPUStatus(val) = self;
        (val & 0x80) != 0
    }

    pub fn set_vblank_started(&mut self, val: bool) {
        let PPUStatus(old) = *self;

        if val {
            *self = PPUStatus(old | 0x80);
        }
        else {
            *self = PPUStatus(old & !0x80);
        }
    }

    pub fn sprite_zero_hit(&self) -> bool {
        let &PPUStatus(val) = self;
        (val & 0x40) != 0
    }

    pub fn set_sprite_zero_hit(&mut self, val: bool) {
        let PPUStatus(old) = *self;

        if val {
            *self = PPUStatus(old | 0x40);
        }
        else {
            *self = PPUStatus(old & !0x40);
        }
    }

    pub fn sprite_overflow(&self) -> bool {
        let &PPUStatus(val) = self;
        (val & 0x20) != 0
    }

    pub fn set_sprite_overflow(&mut self, val: bool) {
        let PPUStatus(old) = *self;

        if val {
            *self = PPUStatus(old | 0x20);
        }
        else {
            *self = PPUStatus(old & !0x20);
        }
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
        status.set_vblank_started(true);
        assert!(status.vblank_started());
        status.set_vblank_started(false);
        assert!(!status.vblank_started());
    }

    #[test]
    fn test_sprite_zero() {
        let status = PPUStatus(0b01000000);
        assert!(status.sprite_zero_hit());

        let mut status = PPUStatus(0x00);
        assert!(!status.sprite_zero_hit());
        status.set_sprite_zero_hit(true);
        assert!(status.sprite_zero_hit());
        status.set_sprite_zero_hit(false);
        assert!(!status.sprite_zero_hit());
    }

    #[test]
    fn test_sprite_overflow() {
        let status = PPUStatus(0b00100000);
        assert!(status.sprite_overflow());

        let mut status = PPUStatus(0x00);
        assert!(!status.sprite_overflow());
        status.set_sprite_overflow(true);
        assert!(status.sprite_overflow());
        status.set_sprite_overflow(false);
        assert!(!status.sprite_overflow());
    }
}
