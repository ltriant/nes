use std::cell::RefCell;
use std::io;
use std::fs::File;
use std::rc::Rc;

use crate::cpu::CPU;
use crate::apu::channel::Voice;
use crate::mem::Memory;

const TIMER_TABLE: [u16; 16] = [
    0x01AC, 0x017C, 0x0154, 0x0140,
    0x011E, 0x00FE, 0x00E2, 0x00D6,
    0x00BE, 0x00A0, 0x008E, 0x0080,
    0x006A, 0x0054, 0x0048, 0x0036
];

pub struct DMC {
    pub enabled: bool,

    pub buffer: u8,

    irq_enabled: bool,
    irq_flag: bool,

    dmc_loop: bool,
    bit_count: u8,
    shift_register: u8,

    sample_address: u16,
    current_address: u16,
    sample_length: u16,
    pub current_length: u16,

    pub cpu: Option<Rc<RefCell<CPU>>>,

    timer_period: u16,
    timer_value: u16,
}

impl Voice for DMC {
    fn signal(&self) -> u8 {
        if !self.enabled {
            return 0;
        }

        self.buffer
    }
}

impl Memory for DMC {
    fn save(&self, _output: &mut File) -> io::Result<()> { Ok(()) }
    fn load(&mut self, _input: &mut File) -> io::Result<()> { Ok(()) }
}

impl DMC {
    pub fn new_dmc_channel() -> Self {
        Self {
            enabled: false,

            buffer: 0,

            irq_enabled: false,
            irq_flag: false,

            dmc_loop: false,
            bit_count: 0,
            shift_register: 0,

            sample_address: 0,
            current_address: 0,
            sample_length: 0,
            current_length: 0,

            timer_period: 0,
            timer_value: 0,

            cpu: None,
        }
    }

    pub fn irq_flag(&self) -> bool { self.irq_flag }
    pub fn clear_irq_flag(&mut self) { self.irq_flag = false }

    pub fn reset(&mut self) {
        debug!("reset: current_address={:04X}, current_length={}", self.sample_address, self.current_length);
        self.current_address = self.sample_address;
        self.current_length = self.sample_length;
    }

    // Register $4010 sets the interrupt enable, loop, and timer period. If the
    // new interrupt enabled status is clear, the interrupt flag is cleared.
    //
    //     il-- ffff       interrupt enabled, loop, frequency index
    pub fn write_control(&mut self, val: u8) {
        let irq      = (val & 0b1000_0000) != 0;
        let dmc_loop = (val & 0b0100_0000) != 0;
        let f_index  =  val & 0b0000_1111;

        self.irq_enabled = irq;

        if !irq {
            self.irq_flag = false;
        }

        self.dmc_loop = dmc_loop;
        self.timer_period = TIMER_TABLE[f_index as usize];
    }

    // A write to $4011 sets the counter and DAC to a new value:
    //
    //     -ddd dddd       new DAC value
    pub fn write_dac(&mut self, val: u8) {
        self.buffer = val & 0b0111_1111
    }

    pub fn write_address(&mut self, val: u8) {
        debug!("write_address: addr={:02X}", val);
        // Sample address = %11AAAAAA.AA000000
        self.sample_address = 0xC000 | ((val as u16) << 6);
    }

    pub fn write_length(&mut self, val: u8) {
        // Sample length = %LLLL.LLLL0001
        debug!("write_length: length={}", val);
        self.sample_length = ((val as u16) << 4) + 1;
    }

    fn step_reader(&mut self) {
        // When the sample buffer is in an empty state and the bytes counter is non-zero,
        // the following occur: The sample buffer is filled with the next sample byte read
        // from memory at the current address, subject to whatever mapping hardware is
        // present (the same as CPU memory accesses). The address is incremented; if it
        // exceeds $FFFF, it is wrapped around to $8000. The bytes counter is decremented;
        // if it becomes zero and the loop flag is set, the sample is restarted (see
        // above), otherwise if the bytes counter becomes zero and the interrupt enabled
        // flag is set, the interrupt flag is set.

        if self.current_length == 0 || self.bit_count != 0 {
            return;
        }

        if let Some(cpu) = &self.cpu {
            // TODO this is up to 4 extra cycles, but could be fewer
            cpu.borrow_mut().stall(4);

            self.shift_register = cpu.borrow_mut().read(self.current_address);
            debug!("shift_register={:02X}", self.shift_register);
        } else {
            error!("No CPU configured. This breaks the DMC.");
            return;
        }

        self.bit_count = 8;

        // The address is incremented; if it exceeds $FFFF, it is wrapped around to $8000.
        let (new_address, overflowed) = self.current_address.overflowing_add(1);
        self.current_address = if overflowed {
            0x8000
        } else {
            new_address
        };

        self.current_length -= 1;
        if self.current_length == 0 {
            if self.dmc_loop {
                self.reset();
            }
            
            if self.irq_enabled {
                self.irq_flag = true;
            }
        }
    }

    fn step_shifter(&mut self) {
        if self.bit_count == 0 {
            return;
        }

        if self.shift_register & 0x01 != 0 {
            if self.buffer <= 125 {
                self.buffer += 2;
            }
        } else {
            if self.buffer >= 2 {
                self.buffer -= 2;
            }
        }

        self.shift_register >>= 1;
        self.bit_count -= 1;
    }

    pub fn step_timer(&mut self) {
        if !self.enabled {
            return
        }

        //debug!("timer_value={}, timer_period={}", self.timer_value, self.timer_period);

        self.step_reader();
        if self.timer_value == 0 {
            self.timer_value = self.timer_period;
            self.step_shifter();
        } else {
            self.timer_value -= 1;
        }
    }
}
