use num::FromPrimitive;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process;
use std::ops::Index;


pub struct BootRom {
    // The ROM data
    boot_rom: Vec<u8>,
}
   
impl Index<u16> for BootRom {
  type Output = u8;
  fn index(&self, index: u16) -> &u8 {
    &self.boot_rom[index as usize]
  }
}

impl BootRom{
    pub fn new(boot_rom_path: &Path) -> BootRom{
        let mut boot_rom_file = File::open(boot_rom_path).unwrap_or_else( |err| {
            println!("Could not load Boot ROM file from {}, error: {}", boot_rom_path.display(), err);
            process::exit(1)
        });

        let mut boot_rom = Vec::new();
        boot_rom_file.read_to_end(&mut boot_rom).unwrap();

        BootRom {
            boot_rom: boot_rom,
        }
    }
    pub fn rb(&mut self, address: u16) -> u8 {
        self[address]
    }

    pub fn rw(&mut self, address: u16) -> u16 {
        ((self[address] as u16) | ((self[address + 1]) as u16) << 8)
    }
}

        

