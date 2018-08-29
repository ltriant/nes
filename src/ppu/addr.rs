enum PPUAddrNybble {
    Hi,
    Lo,
}

pub struct PPUAddr {
    val: u16,
    nyb: PPUAddrNybble,
}

impl PPUAddr {
    pub fn new_ppu_addr() -> PPUAddr {
        PPUAddr {
            val: 0,
            nyb: PPUAddrNybble::Hi,
        }
    }

    pub fn write(&mut self, val: u8) {
        match self.nyb {
            PPUAddrNybble::Lo => {
                let val = val as u16;
                self.val = (self.val & 0xff00) | val;
                self.nyb = PPUAddrNybble::Hi;
            },
            PPUAddrNybble::Hi => {
                let val = (val as u16) << 8;
                self.val = (self.val & 0x00ff) | val;
                self.nyb = PPUAddrNybble::Lo;
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ppu_addr() {
        let mut ppu_addr = PPUAddr::new_ppu_addr();

        ppu_addr.write(0x21);
        assert!(ppu_addr.val == 0x2100);

        ppu_addr.write(0x08);
        assert!(ppu_addr.val == 0x2108);

        ppu_addr.write(0x32);
        assert!(ppu_addr.val == 0x3208);

        ppu_addr.write(0x01);
        assert!(ppu_addr.val == 0x3201);
    }
}
