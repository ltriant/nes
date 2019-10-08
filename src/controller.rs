// Controller code
//
// http://wiki.nesdev.com/w/index.php/Standard_controller
//
// Essentially there are 8 bits, set to a 1 if the button is pressed down, and
// set to 0 if not pressed down.
//
// bit    |   7   |   6   |   5   |  4   |   3   |   2    |   1   |   0   |
// button | right | left  | down  |  up  | start | select |   b   |   a   |

use crate::mem::Memory;

pub struct Controller {
    buttons: [bool; 8],
    index: usize,
    strobe: u8,
}

impl Memory for Controller {
    fn read(&mut self, _address: u16) -> u8 {
        let mut value = 0;

        if self.index < 8 && self.buttons[self.index] {
            value = 1;
        }

        self.index += 1;
        if self.strobe & 1 == 1 {
            self.index = 0;
        }

        value
    }

    fn write(&mut self, _address: u16, val: u8) {
        self.strobe = val;

        if self.strobe & 1 == 1 {
            self.index = 0;
        }
    }
}

impl Controller {
    pub fn new_controller() -> Self {
        Self {
            buttons: [false; 8],
            index: 0,
            strobe: 0,
        }
    }

    pub fn a(&mut self, v: bool) { self.buttons[0] = v; }
    pub fn b(&mut self, v: bool) { self.buttons[1] = v; }
    pub fn select(&mut self, v: bool) { self.buttons[2] = v; }
    pub fn start(&mut self, v: bool) { self.buttons[3] = v; }
    pub fn up(&mut self, v: bool) { self.buttons[4] = v; }
    pub fn down(&mut self, v: bool) { self.buttons[5] = v; }
    pub fn left(&mut self, v: bool) { self.buttons[6] = v; }
    pub fn right(&mut self, v: bool) { self.buttons[7] = v; }
}
