use num::FromPrimitive;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process;


enum_from_primitive! {
#[derive(Debug)]
enum ROMSize {
    K32 = 0,
    K64 = 1,
    K128 = 2,
    K256 = 3,
    K512 = 4,
    M1 = 5,
    M2 = 6,
    M3 = 7,
}
}

enum_from_primitive! {
#[derive(Debug)]
enum RAMSize {
    NoRam = 0,
    K2 = 1,
    K8 = 2,
    K32 = 3,    // Banks of 8 KBytes
    K128 = 4,   // Banks of 8 KBytes
    K64 = 5,    // Banks of 8 KBytes
}
}

pub struct Cartridge {
    // The ROM data
    pub rom: Vec<u8>,
    rom_size: ROMSize,
    ram_size: RAMSize,
}
   
impl Cartridge {
    pub fn new(rom_path: &Path) -> Cartridge {
        let mut rom_file = File::open(rom_path).unwrap_or_else( |err| {
            println!("Could not load ROM file from {}, error: {}", rom_path.display(), err);
            process::exit(1)
        });

        let mut rom = Vec::new();
        rom_file.read_to_end(&mut rom).unwrap();

        Cartridge {
            rom: rom,
            rom_size: ROMSize::K32,
            ram_size: RAMSize::NoRam,
        }
    }
}
        

