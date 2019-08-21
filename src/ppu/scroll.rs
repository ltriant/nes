enum PPUScrollDir {
    X,
    Y,
}

pub struct PPUScroll {
    pub x: u16,
    pub y: u16,
    dir: PPUScrollDir,
}

impl PPUScroll {
    pub fn new_ppu_scroll() -> Self {
        Self {
            x: 0,
            y: 0,
            dir: PPUScrollDir::X,
        }
    }

    pub fn reset_latch(&mut self) {
        self.dir = PPUScrollDir::X;
    }

    pub fn write(&mut self, val: u8) {
        match self.dir {
            PPUScrollDir::X => {
                self.x = val as u16;
                self.dir = PPUScrollDir::Y;
            },
            PPUScrollDir::Y => {
                self.y = val as u16;
                self.dir = PPUScrollDir::X;
            },
        }
    }

    pub fn incr_x(&mut self) {
        let old_x = self.x;
        self.x = (old_x + 256) & 0xff;
    }

    pub fn incr_y(&mut self) {
        let old_y = self.y;
        self.y = (old_y + 240) & 0xff;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ppu_scroll() {
        let mut ppu_scroll = PPUScroll::new_ppu_scroll();

        ppu_scroll.write(0x20);
        assert!(ppu_scroll.x == 0x20);
        assert!(ppu_scroll.y == 0x00);

        ppu_scroll.write(0x4a);
        assert!(ppu_scroll.x == 0x20);
        assert!(ppu_scroll.y == 0x4a);

        ppu_scroll.write(0x32);
        assert!(ppu_scroll.x == 0x32);
        assert!(ppu_scroll.y == 0x4a);
    }
}
