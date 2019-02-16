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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vblank() {
        let status = PPUStatus(0b10000000);
        assert!(status.vblank_started());

        // TODO how to set vblank status?
    }

    #[test]
    fn test_sprite_zero() {
        let status = PPUStatus(0b01000000);
        assert!(status.sprite_zero_hit());

        // TODO how to set sprite zero hit?
    }

    #[test]
    fn test_sprite_overflow() {
        let status = PPUStatus(0b00100000);
        assert!(status.sprite_overflow());

        // TODO how to set sprite overflow?
    }
}
