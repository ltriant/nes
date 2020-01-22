#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;

mod apu;
mod console;
mod controller;
mod cpu;
mod opcode;
mod inst;
mod mapper;
mod mem;
mod ines;
mod addr;
mod ppu;
mod palette;
mod serde;

use std::env;
use std::process;

use crate::console::Console;
use crate::ines::CartridgeError;

fn main() {
    env_logger::init();

    if let Some(rom) = env::args().skip(1).next() {
        let mut console = Console::new_nes_console();
        if let Err(e) = console.insert_cartridge(&rom) {
            match e {
                CartridgeError::IO(io_e) => {
                    println!("There was an error reading ROM data from {}: {}",
                             rom, io_e);
                },
                CartridgeError::InvalidMagic => {
                    println!("File {} is invalid. Expected iNES formatted ROM.", rom);
                },
                CartridgeError::UnsupportedCartridge => {
                    println!("Unsupported cartridge type. Only supports NTSC for now.");
                },
                CartridgeError::UnsupportedMapper(m) => {
                    println!("Unsupported mapper type: {}", m);
                },
            }

            process::exit(1);
        }

        console.power_up();
    } else {
        println!("Missing required parameter: a path to a ROM file.");
        process::exit(1);
    }
}
