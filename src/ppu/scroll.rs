enum PPUScrollDir {
    X,
    Y,
}

pub struct PPUScroll {
    x: u8,
    y: u8,
    dir: PPUScrollDir,
}

impl PPUScroll {
    pub fn new_ppu_scroll() -> PPUScroll {
        PPUScroll {
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
                self.x = val;
                self.dir = PPUScrollDir::Y;
            },
            PPUScrollDir::Y => {
                self.y = val;
                self.dir = PPUScrollDir::X;
            },
        }
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
