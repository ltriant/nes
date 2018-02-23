mod console;
mod cpu;
mod opcode;
mod inst;
mod mem;

use console::Console;

fn main() {
    let mut console = Console::new_nes_console();

    console.insert_cartridge("roms/nestest.nes")
        .expect("could not insert nestest");

    console.disassemble();
}
