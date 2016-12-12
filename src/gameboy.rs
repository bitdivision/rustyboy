use super::cpu;
use super::cartridge;
use super::bootrom;

// TODO: Really this should have the boot rom involved
pub struct Gameboy {
    cpu: cpu::CPU,
}

impl Gameboy {
    pub fn new(boot_rom: bootrom::BootRom, cart: cartridge::Cartridge) -> Gameboy {
        Gameboy {
            cpu: cpu::CPU::new(boot_rom, cart),
        }
    }
    
    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction();
    }
}
