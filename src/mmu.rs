use super::bootrom;
use super::cartridge;

const WRAM_SIZE: usize = 0x8000;
const ZRAM_SIZE: usize = 0x7F;


pub struct MMU {
    boot_rom_mapped: bool,
    
    boot_rom: bootrom::BootRom,
    cart: cartridge::Cartridge,
    wram: [u8; WRAM_SIZE],
    cart_ram: [u8; 0x2000],
    zram: [u8; ZRAM_SIZE],
}

impl MMU {

    pub fn new(boot_rom: bootrom::BootRom, cart: cartridge::Cartridge) -> MMU {
        MMU {
            wram: [0; WRAM_SIZE],
            zram: [0; ZRAM_SIZE],
            cart_ram: [1; 0x2000],
            cart: cart,
            boot_rom: boot_rom,
            boot_rom_mapped: true,
        }
    }

    pub fn rb(&mut self, address: u16) -> u8 {
        match address {
            // Boot ROM
            0x0000 ... 0x00FF => return self.boot_rom.rb(address),
            // Cart ROM
            0x0000 ... 0x7FFF => return *self.cart.rom.get((address - 0x00FF) as usize).unwrap(),
            // println!("Reading Graphics"),
            0x8000 ... 0x9FFF => return self.boot_rom.rb(address),
            // println!("Reading Cartridge RAM"),
            0xA000 ... 0xBFFF => return self.boot_rom.rb(address),
            // println!("Reading RAM"),
            0xC000 ... 0xDFFF => return self.boot_rom.rb(address),
            // println!("Reading Shadow RAM"),
            0xE000 ... 0xFDFF => return self.boot_rom.rb(address),
            // println!("Reading Graphics Sprites"),
            0xFE00 ... 0xFE9F => return self.boot_rom.rb(address),
            // println!("Reading MM/IO"),
            0xFF00 ... 0xFF7F => return self.boot_rom.rb(address),
            // println!("Reading Zero page RAM"),
            0xFF80 ... 0xFFFE => return self.boot_rom.rb(address),
            x => panic!("Read from unrecognized memory address"),
        }
    }

    pub fn rw(&mut self, address: u16) -> u16 {
        ((self.rb(address) as u16) | ((self.rb(address + 1)) as u16) << 8)
    }

    pub fn wb(&mut self, address: u16, value: u8) {
        match address {
            0x8000 ... 0x9FFF => { self.cart_ram[(address - 0x8000) as usize] = value },
            0xFF00 ... 0xFF0F => { println!("Writing to IO mapped memory. address: {}, value: {}", address, value)},
            0xFF10 ... 0xFF3F => { println!("Writing to sound IO. Address: 0x{:x}, value 0x{:x}", address, value)},
            0xFF47 ... 0xFF49 => { println!("Writing to LCD Palettes. Address: 0x{:x}, value 0x{:x}", address, value)},
            0xFF80 ... 0xFFFE => { self.zram[address as usize - 0xFF80] = value }
            _ => panic!("Write to unrecognized memory address 0x{:x}, value: 0x{:x}", address, value),
        };
    }

    pub fn ww(&mut self, address:u16, value: u16) {
        self.wb(address, (value & 0x00FF) as u8);
        self.wb(address + 1, (value << 8) as u8)
    }


}
