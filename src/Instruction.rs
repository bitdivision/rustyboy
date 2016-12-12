pub struct Instruction<'a> {
    name: &'a str,
    bytes: u8,
    cycles: u8,
}

impl<'a> Instruction<'a> {
    pub fn new(&str name, u8 bytes, u8 cycles) -> Instruction {
        Instruction {
            name: name, 
            bytes: bytes,
            cycles: cycles,
    }
}
