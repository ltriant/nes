mod console;
mod cpu;
mod opcode;
mod inst;
mod mem;
mod ines;
mod addr;
mod ppu;

use console::Console;

fn main() {
    let mut console = Console::new_nes_console();

    console.insert_cartridge("roms/nestest.nes")
        .expect("could not insert nestest");
    //console.insert_cartridge("roms/instr_test-v5/official_only.nes")
    //    .expect("could not insert nestest");


    console.power_up();
}
