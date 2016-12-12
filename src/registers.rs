use std::fmt;

#[derive(Debug)]
pub struct Flags {
    pub zero: bool,
    pub add_subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            zero: false,
            add_subtract: false,
            half_carry: false,
            carry: false,
        }
    }

    pub fn clear(&mut self) {
        self.zero = false;
        self.add_subtract = false;
        self.half_carry = false;
        self.carry = false;
    }

    pub fn set(&mut self, zero: bool, add_subtract: bool, half_carry: bool, carry: bool) {
        self.zero = zero;
        self.add_subtract = add_subtract;
        self.half_carry = half_carry;
        self.carry = carry;
    }
}


pub struct Registers {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: Flags,
    pub h: u8,
    pub l: u8
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pc:0x{:04X} sp:0x{:04X} a:0x{:02X} b:0x{:02X} c:0x{:02X} d:0x{:02X} e:0x{:02X} flags:{:?} h:0x{:02X} l:0x{:02X}",
               self.pc, self.sp, self.a, self.b, self.c, self.d, self.e, self.f, self.h, self.l)
    }
}


impl Registers {
    pub fn new() -> Registers {
        Registers {
            pc: 0,
            sp: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: Flags::new(),
            h: 0,
            l: 0 
        }
    }

    pub fn set_hl(&mut self, hl: u16) {
        self.h = (hl >> 8) as u8;
        self.l = (hl & 0x00FF) as u8;
    }

    pub fn get_hl(&mut self) -> u16 {
        (((self.h as u16) << 8) | self.l as u16)
    }

    pub fn set_de(&mut self, de: u16) {
        self.d = (de >> 8) as u8;
        self.e = (de & 0x00FF) as u8;
    }

    pub fn get_de(&mut self) -> u16 {
        (((self.d as u16) << 8) | self.e as u16)
    }

}
