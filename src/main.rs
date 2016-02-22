#![cfg_attr(feature = "assignment_operators", feature(augmented_assignments, op_assign_traits))]

#[macro_use] extern crate enum_primitive;
extern crate num;

#[macro_use]
extern crate bitflags;

mod cartridge;
mod cpu;
mod bootrom;
mod gameboy;
mod registers;
mod mmu;

use std::path::Path;
use std::env;
use std::process;


fn cmd_help() {
    println!("Usage: 
    gameboyrust <boot-rom-file> <rom-file>");
}


fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 =>  {
        },
        _ => {
            cmd_help()
        },
    }
    
    let boot_rom_path = Path::new(&args[1]);
    let rom_path = Path::new(&args[2]);

    println!("Welcome to RustyBoy");

    let boot_rom = bootrom::BootRom::new(boot_rom_path);
    let cart = cartridge::Cartridge::new(rom_path);

    println!("Loaded Boot ROM ({})", boot_rom_path.display());
    println!("Loaded ROM file ({})", rom_path.display());

    let mut gameboy = gameboy::Gameboy::new(boot_rom, cart);

    println!("Running");
    
    loop {
        gameboy.run_instruction();
    }

}
