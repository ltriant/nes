use std::collections::HashSet;
use std::convert::From;
use std::io::{Read, Write};
use std::io;
use std::fs::File;

use crate::mapper::{Mapper, MapperEvent};
use crate::mapper::MirrorMode;
use crate::serde;

const PRG_BANK_SIZE: usize = 8192;
const CHR_BANK_SIZE: usize = 1024;

#[derive(Debug)]
enum Command {
    CHRBank(u8),
    PRGBank(u8),
    Mirror,
    IRQ,
    IRQLo,
    IRQHi,
}

impl From<u8> for Command {
    fn from(val: u8) -> Self {
        match val {
            0x00 ..= 0x07 => Command::CHRBank(val),
            0x08 ..= 0x0b => Command::PRGBank(val),
            0x0c          => Command::Mirror,
            0x0d          => Command::IRQ,
            0x0e          => Command::IRQLo,
            0x0f          => Command::IRQHi,
            _ => unreachable!("bad command"),
        }
    }
}

//
// Sunsoft FME-7/5A (mapper 69)
//
pub struct Mapper69 {
    chr_rom: Vec<u8>,
    prg_rom: Vec<u8>,
    sram: [u8; 0x2000],
    mirror_mode: MirrorMode,
    address_maps: HashSet<std::ops::RangeInclusive<u16>>,

    // A command to run
    cmd: Option<Command>,

    // Registers (set by the command above)
    chr_banks: [usize; 8],
    sram_bank: usize,
    prg_banks: [usize; 4],
    ram_select: bool,
    ram_enabled: bool,

    irq_enabled: bool,
    irq_counter_enabled: bool,
    irq_counter_value: u16,
    irq_flag: bool,
}


impl Mapper69 {
    pub fn new_mapper(rom: Vec<u8>,
                      vrom: Vec<u8>,
                      mirror_mode: u8)
        -> Self
    {
        let n_banks = rom.len() / PRG_BANK_SIZE;

        Self {
            chr_rom: vrom,
            prg_rom: rom,
            sram: [0; 0x2000],
            mirror_mode: MirrorMode::from_hv01(mirror_mode),
            address_maps: HashSet::new(),

            cmd: None,

            chr_banks: [0; 8],
            sram_bank: 0,
            prg_banks: [0, 0, 0, n_banks - 1],
            ram_select: false,
            ram_enabled: false,

            irq_enabled: false,
            irq_counter_enabled: false,
            irq_counter_value: 0,
            irq_flag: false,
        }
    }

    fn run_cmd(&mut self, parameter: u8) {
        match self.cmd {
            Some(Command::CHRBank(n)) => {
                self.chr_banks[n as usize] = parameter as usize;
            },
            Some(Command::PRGBank(n)) => {
                if n == 0x08 {
                    // 7  bit  0
                    // ---- ----
                    // ERbB BBBB
                    // |||| ||||
                    // ||++-++++- The bank number to select at CPU $6000 - $7FFF
                    // |+------- RAM / ROM Select Bit
                    // |         0 = PRG ROM
                    // |         1 = PRG RAM
                    // +-------- RAM Enable Bit (6264 +CE line)
                    //           0 = PRG RAM Disabled
                    //           1 = PRG RAM Enabled

                    // Despite there being 6 bits of data available for the
                    // bank number in the FME-7 board, only 5 bits were used
                    // by the 5A and 5B variants.
                    let bank         =  parameter & 0b0001_1111;
                    self.ram_select  = (parameter & 0b0100_0000) != 0;
                    self.ram_enabled = (parameter & 0b1000_0000) != 0;

                    self.sram_bank = bank as usize;
                } else {
                    // 7  bit  0
                    // ---- ----
                    // ..bB BBBB
                    //   || ||||
                    //   ++-++++- The bank number to select for the specified bank.
                    //
                    // Despite there being 6 bits of data available in the
                    // FME-7 board, only 5 bits were used by the 5A and 5B
                    // variants.
                    let bank = parameter & 0b0001_1111;

                    // n will be one of 0x09, 0x0a, 0x0b, and I want to map
                    // that to a number 0, 1, or 2, so we subtract 9
                    self.prg_banks[n as usize - 0x09] = bank as usize;
                }
            },
            Some(Command::Mirror) => {
                // 7  bit  0
                // ---- ----
                // .... ..MM
                //        ||
                //        ++- Mirroring Mode
                //             0 = Vertical
                //             1 = Horizontal
                //             2 = One Screen Mirroring from $2000 ("1ScA")
                //             3 = One Screen Mirroring from $2400 ("1ScB")
                let mode = parameter & 0b0000_0011;
                self.mirror_mode = MirrorMode::from_vh01(mode);
            },
            Some(Command::IRQ) => {
                // 7  bit  0
                // ---- ----
                // C... ...T
                // |       |
                // |       +- IRQ Enable
                // |           0 = Do not generate IRQs
                // |           1 = Do generate IRQs
                // +-------- IRQ Counter Enable
                //             0 = Disable Counter Decrement
                //             1 = Enable Counter Decrement
                self.irq_enabled         = (parameter & 0b0000_0001) != 0;
                self.irq_counter_enabled = (parameter & 0b1000_0000) != 0;
            },
            Some(Command::IRQLo) => {
                // 7  bit  0
                // ---- ----
                // LLLL LLLL
                // |||| ||||
                // ++++-++++- The low eight bits of the IRQ counter
                self.irq_counter_value = (self.irq_counter_value & 0xff00)
                    | parameter as u16;
            },
            Some(Command::IRQHi) => {
                // 7  bit  0
                // ---- ----
                // HHHH HHHH
                // |||| ||||
                // ++++-++++- The high eight bits of the IRQ counter
                self.irq_counter_value = (self.irq_counter_value & 0x00ff)
                    | ((parameter as u16) << 8);
            },

            None => { },
        }
    }

    fn step_irq_counter(&mut self, cycles: u64) {
        // The IRQ counter is clocked for every CPU cycle, rather than every
        // PPU scanline, as per other mappers.

        let mut trigger = false;

        // If the IRQ counter is enabled, it always ticks the counter
        if self.irq_counter_enabled {
            let (irq_count, underflowed) = self.irq_counter_value
                .overflowing_sub(cycles as u16); // TODO casting u64 down to u16

            self.irq_counter_value = irq_count;

            // When the IRQ counter wraps around from 0x0000 to 0xFFFF, an IRQ
            // is generated.
            trigger = underflowed;
        }

        // IRQ's will only trigger if IRQ is enabled, regardless of whether the
        // IRQ counter is enabled, or what the IRQ counter's value is.
        self.irq_flag = self.irq_enabled && trigger;
    }
}

impl Mapper for Mapper69 {
    fn mirror_mode(&self) -> &MirrorMode {
        &self.mirror_mode
    }

    fn address_maps(&self) -> &HashSet<std::ops::RangeInclusive<u16>> {
        &self.address_maps
    }

    fn notify(&mut self, event: MapperEvent) {
        match event {
            MapperEvent::CPUTick(cycles) => { self.step_irq_counter(cycles) },
            _ => { },
        }
    }

    fn irq_flag(&self) -> bool {
        self.irq_flag
    }

    fn read(&mut self, address: u16) -> u8 {
        match address {
            // CHR-ROM
            0x0000 ..= 0x1fff => {
                let reg = address as usize / CHR_BANK_SIZE;
                let bank = self.chr_banks[reg];
                let index = (bank * CHR_BANK_SIZE) | (address as usize & 0x03ff);
                self.chr_rom[index]
            },

            // SRAM
            0x6000 ..= 0x7fff => {
                match (self.ram_select, self.ram_enabled) {
                    (true, false) => 0,  // open bus
                    (true, true)  => self.sram[address as usize - 0x6000],
                    (false, _)    => {
                        let index = (self.sram_bank * PRG_BANK_SIZE)
                            | (address as usize & 0x1fff);
                        self.prg_rom[index & (self.prg_rom.len() - 1)]
                    },
                }
            },

            // PRG-ROM
            0x8000 ..= 0xffff => {
                let reg = (address as usize - 0x8000) / PRG_BANK_SIZE;
                let index = (self.prg_banks[reg] * PRG_BANK_SIZE)
                    | (address as usize & 0x1fff);
                self.prg_rom[index & (self.prg_rom.len() - 1)]
            },

            _ => 0,
        }
    }

    fn write(&mut self, address: u16, val: u8) {
        match address {
            // CHR-ROM
            0x0000 ..= 0x1fff => { },

            // SRAM
            0x6000 ..= 0x7fff => {
                if self.ram_select && self.ram_enabled {
                    self.sram[address as usize - 0x6000] = val;
                }
            },

            // PRG-ROM
            0x8000 ..= 0x9fff => {
                // 7  bit  0
                // ---- ----
                // .... CCCC
                //      ||||
                //      ++++- The command number to invoke when writing to the
                //            Parameter Register
                self.cmd = Some(Command::from(val & 0b0000_1111));
            },

            0xa000 ..= 0xbfff => {
                // 7  bit  0
                // ---- ----
                // PPPP PPPP
                // |||| ||||
                // ++++-++++- The parameter to use for this command. Writing to
                //            this register invokes the command in the Command
                //            Register.
                debug!("running cmd: {:?}, val: {:08b}", self.cmd, val);
                self.run_cmd(val);
            },

            _ => { },
        }
    }

    fn save(&self, output: &mut File) -> io::Result<()> {
        serde::encode_vec(output, &self.chr_rom)?;
        serde::encode_vec(output, &self.prg_rom)?;
        output.write(&self.sram)?;
        serde::encode_u8(output, self.mirror_mode as u8)?;

        match self.cmd {
            None => { serde::encode_u8(output, 0)? },

            Some(Command::CHRBank(n)) => {
                serde::encode_u8(output, 1)?;
                serde::encode_u8(output, n)?;
            },
            Some(Command::PRGBank(n)) => {
                serde::encode_u8(output, 2)?;
                serde::encode_u8(output, n)?;
            },
            Some(Command::Mirror) => { serde::encode_u8(output, 3)? },
            Some(Command::IRQ)    => { serde::encode_u8(output, 4)? },
            Some(Command::IRQLo)  => { serde::encode_u8(output, 5)? },
            Some(Command::IRQHi)  => { serde::encode_u8(output, 6)? },
        }

        for i in 0 .. 8 {
            serde::encode_usize(output, self.chr_banks[i])?;
        }

        serde::encode_usize(output, self.sram_bank)?;

        for i in 0 .. 4 {
            serde::encode_usize(output, self.prg_banks[i])?;
        }

        serde::encode_u8(output, self.ram_select as u8)?;
        serde::encode_u8(output, self.ram_enabled as u8)?;

        serde::encode_u8(output, self.irq_enabled as u8)?;
        serde::encode_u8(output, self.irq_counter_enabled as u8)?;
        serde::encode_u16(output, self.irq_counter_value)?;

        Ok(())
    }

    fn load(&mut self, input: &mut File) -> io::Result<()> {
        self.chr_rom = serde::decode_vec(input)?;
        self.prg_rom = serde::decode_vec(input)?;
        input.read(&mut self.sram)?;
        self.mirror_mode = MirrorMode::from_vh01(serde::decode_u8(input)?);

        let cmd = serde::decode_u8(input)?;
        self.cmd = match cmd {
            0 => None,
            1 => {
                let n = serde::decode_u8(input)?;
                Some(Command::CHRBank(n))
            },
            2 => {
                let n = serde::decode_u8(input)?;
                Some(Command::PRGBank(n))
            },
            3 => Some(Command::Mirror),
            4 => Some(Command::IRQ),
            5 => Some(Command::IRQLo),
            6 => Some(Command::IRQHi),
            _ => unreachable!("bad cmd"),
        };

        for i in 0 .. 8 {
            self.chr_banks[i] = serde::decode_usize(input)?;
        }

        self.sram_bank = serde::decode_usize(input)?;

        for i in 0 .. 4 {
            self.prg_banks[i] = serde::decode_usize(input)?;
        }

        self.ram_select = serde::decode_u8(input)? != 0;
        self.ram_enabled = serde::decode_u8(input)? != 0;

        self.irq_enabled = serde::decode_u8(input)? != 0;
        self.irq_counter_enabled = serde::decode_u8(input)? != 0;
        self.irq_counter_value = serde::decode_u16(input)?;

        Ok(())
    }
}
