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
