use super::bootrom;
use registers;

pub struct CPU {
    boot_rom: bootrom::BootRom,
    registers: registers::Registers,
}

impl CPU {
    pub fn new(boot_rom: bootrom::BootRom) -> CPU {
        CPU {
            boot_rom: boot_rom,
            registers: registers::Registers::new(),
        }
    }

    pub fn run_instruction(&mut self) {
        println!("0x{:x}", self.boot_rom.rb(self.registers.pc));
        self.registers.pc += 1;
    }
}
