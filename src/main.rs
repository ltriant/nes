#[macro_use] extern crate log;

mod console;
mod cpu;
mod opcode;
mod inst;
mod mem;
mod ines;
mod addr;
mod ppu;

use std::env;

use crate::console::Console;

fn main() {
    env_logger::init();

    let rom = env::args().skip(1).next()
        .unwrap_or(String::from("roms/nestest.nes"));

    let mut console = Console::new_nes_console();
    console.insert_cartridge(rom).expect("could not insert nestest");
    console.power_up();
}
