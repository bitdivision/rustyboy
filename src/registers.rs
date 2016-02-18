bitflags!(
    flags Flags: u8 {
        const ZERO         = 0b_1000_0000,
        const ADD_SUBTRACT = 0b_0100_0000,
        const HALF_CARRY   = 0b_0010_0000,
        const CARRY        = 0b_0001_0000
    }
);


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
            f: ZERO,
            h: 0,
            l: 0 
        }
    }
}
