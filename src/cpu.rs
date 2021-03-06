use super::bootrom;
use super::cartridge;
use registers;
use mmu;

pub struct CPU {
    mmu: mmu::MMU,
    registers: registers::Registers,
}

impl CPU {
    pub fn new(boot_rom: bootrom::BootRom, cart: cartridge::Cartridge) -> CPU {
        CPU {
            mmu: mmu::MMU::new(boot_rom, cart),
            registers: registers::Registers::new(),
        }
    }

    pub fn run_instruction(&mut self) {
        println!("Registers {}", self.registers);
        println!("0x{:x}", self.mmu.rb(self.registers.pc));
        let instruction_length = self.do_instruction();
        self.registers.pc += instruction_length
    }

    pub fn do_instruction(&mut self) -> u16 {
        let opcode = self.mmu.rb(self.registers.pc);
        match opcode {
            // NOP | Bytes: 1  Cycles: 4 Flags: - - - -
            0x00 => { 1 },
            // LD BC,d | Bytes: 3  Cycles: 12 Flags: - - - -
            0x01 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD (BC),A | Bytes: 1  Cycle:8 Flags: - - - -	
            0x02 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // INC BC | Bytes: 1  Cycle:8 Flags: - - - -	
            0x03 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // INC B | Bytes: 1  Cycle:4 Flags: Z 0 H -	
            0x04 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // DEC B | Bytes: 1  Cycle:4 Flags: Z 1 H -	
            0x05 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD B,d8 | Bytes: 2  Cycle:8 Flags: - - - -	
            0x06 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // RLCA | Bytes: 1  Cycle:4 Flags: 0 0 0 C	
            0x07 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD (a16),SP | Bytes: 3  Cycle:2 Flags: - - - -	
            0x08 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADD HL,BC | Bytes: 1  Cycle:8 Flags: - 0 H C	
            0x09 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD A,(BC) | Bytes: 1  Cycle:8 Flags: - - - -	
            0x0a => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
           // DEC BC | Bytes: 1  Cycle:8 Flags: - - - -	
            0x0b => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // INC C | Bytes: 1  Cycle:4 Flags: Z 0 H -	
            0x0c => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // DEC C | Bytes: 1  Cycle:4 Flags: Z 1 H -	
            0x0d => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD C,d8 | Bytes: 2  Cycle:8 Flags: - - - -	
            0x0e => { self.registers.c = self.mmu.rb(self.registers.pc + 1); 2},
            // RRCA | Bytes: 1  Cycle:4 Flags: 0 0 0 C
            0x0f => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // STOP 0 | Bytes: 2  Cycle:4 Flags: - - - -	
            0x10 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD DE,d16 | Bytes: 3  Cycle:1 Flags: - - - -	
            0x11 => { self.ld_de_d16() },
            // LD (DE),A | Bytes: 1  Cycle:8 Flags: - - - -	
            0x12 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // INC DE | Bytes: 1  Cycle:8 Flags: - - - -	
            0x13 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // INC D | Bytes: 1  Cycle:4 Flags: Z 0 H -	
            0x14 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // DEC D | Bytes: 1  Cycle:4 Flags: Z 1 H -	
            0x15 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD D,d8 | Bytes: 2  Cycle:8 Flags: - - - -	
            0x16 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // RLA | Bytes: 1  Cycle:4 Flags: 0 0 0 C	
            0x17 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // JR r8 | Bytes: 2  Cycle:1 Flags: - - - -	
            0x18 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADD HL,DE | Bytes: 1  Cycle:8 Flags: - 0 H C	
            0x19 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD A,(DE) | Bytes: 1  Cycle:8 Flags: - - - -	
            0x1a => { self.ld_a_addr_de()},
            // DEC DE | Bytes: 1  Cycle:8 Flags: - - - -	
            0x1b => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // INC E | Bytes: 1  Cycle:4 Flags: Z 0 H -	
            0x1c => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // DEC E | Bytes: 1  Cycle:4 Flags: Z 1 H -	
            0x1d => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD E,d8 | Bytes: 2  Cycle:8 Flags: - - - -	
            0x1e => self.ld_e_d8(),
            // RRA | Bytes: 1  Cycle:4 Flags: 0 0 0 C
            0x1f => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // JR NZ,r8 | Bytes: 2  Cycle: 12/8 Flags: - - - -	
            // Load an 8 bit relative address, add it to the pc and return
            0x20 => { 
                self.jr_nz()
            },
            // LD HL,d16 | Bytes: 3  Cycle:1 Flags: - - - -	
            0x21 => self.ld_hl(),
            // LD (HL+),A | Bytes: 1  Cycle:8 Flags: - - - -	
            0x22 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // INC HL | Bytes: 1  Cycle:8 Flags: - - - -	
            0x23 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // INC H | Bytes: 1  Cycle:4 Flags: Z 0 H -	
            0x24 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // DEC H | Bytes: 1  Cycle:4 Flags: Z 1 H -	
            0x25 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD H,d8 | Bytes: 2  Cycle:8 Flags: - - - -	
            0x26 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // DAA | Bytes: 1  Cycle:4 Flags: Z - 0 C	
            0x27 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // JR Z,r8 | Bytes: 2  Cycles: 12/8 - - - -	
            0x28 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADD HL,HL | Bytes: 1  Cycle:8 Flags: - 0 H C	
            0x29 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD A,(HL+) | Bytes: 1  Cycle:8 Flags: - - - -	
            0x2a => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // DEC HL | Bytes: 1  Cycle:8 Flags: - - - -	
            0x2b => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // INC L | Bytes: 1  Cycle:4 Flags: Z 0 H -	
            0x2c => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // DEC L | Bytes: 1  Cycle:4 Flags: Z 1 H -	
            0x2d => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD L,d8 | Bytes: 2  Cycle:8 Flags: - - - -	
            0x2e => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // CPL | Bytes: 1  Cycle:4 Flags: - 1 1 -
            0x2f => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // JR NC,r8 | Bytes: 2  Cycle: 12/8 - - - -	
            0x30 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD SP,d16 | Bytes: 3  Cycle:1 Flags: - - - -	
            0x31 => { self.registers.sp = self.mmu.rw(self.registers.pc + 1); 3},
            // LD (HL-),A | Bytes: 1  Cycle:8 Flags: - - - -	
            // LD HL and then decrement
            0x32 => self.ld_hld(),
            // INC SP | Bytes: 1  Cycle:8 Flags: - - - -	
            0x33 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // INC (HL) | Bytes: 1  Cycle:1 Flags: Z 0 H -	
            0x34 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // DEC (HL) | Bytes: 1  Cycle:1 Flags: Z 1 H -	
            0x35 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD (HL),d8 | Bytes: 2  Cycle:1 Flags: - - - -	
            0x36 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // SCF | Bytes: 1  Cycle:4 Flags: - 0 0 1	
            0x37 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // JR C,r8 | Bytes: 2  Cycle:12/8 Flags: - - - -	
            0x38 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADD HL,SP | Bytes: 1  Cycle:8 Flags: - 0 H C	
            0x39 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD A,(HL-) | Bytes: 1  Cycle:8 Flags: - - - -	
            0x3a => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // DEC SP | Bytes: 1  Cycle:8 Flags: - - - -	
            0x3b => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // INC A | Bytes: 1  Cycle:4 Flags: Z 0 H -	
            0x3c => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // DEC A | Bytes: 1  Cycle:4 Flags: Z 1 H -	
            0x3d => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD A,d8 | Bytes: 2  Cycle:8 Flags: - - - -	
            0x3e => { self.registers.a = self.mmu.rb(self.registers.pc + 1); 2},
            // CCF | Bytes: 1  Cycle:4 Flags: - 0 0 C
            0x3f => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD B,B | Bytes: 1  Cycle:4 Flags: - - - -	
            0x40 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD B,C | Bytes: 1  Cycle:4 Flags: - - - -	
            0x41 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD B,D | Bytes: 1  Cycle:4 Flags: - - - -	
            0x42 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD B,E | Bytes: 1  Cycle:4 Flags: - - - -	
            0x43 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD B,H | Bytes: 1  Cycle:4 Flags: - - - -	
            0x44 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD B,L | Bytes: 1  Cycle:4 Flags: - - - -	
            0x45 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD B,(HL) | Bytes: 1  Cycle:8 Flags: - - - -	
            0x46 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD B,A | Bytes: 1  Cycle:4 Flags: - - - -	
            0x47 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD C,B | Bytes: 1  Cycle:4 Flags: - - - -	
            0x48 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD C,C | Bytes: 1  Cycle:4 Flags: - - - -	
            0x49 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD C,D | Bytes: 1  Cycle:4 Flags: - - - -	
            0x4a => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD C,E | Bytes: 1  Cycle:4 Flags: - - - -	
            0x4b => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD C,H | Bytes: 1  Cycle:4 Flags: - - - -	
            0x4c => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD C,L | Bytes: 1  Cycle:4 Flags: - - - -	
            0x4d => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD C,(HL) | Bytes: 1  Cycle:8 Flags: - - - -	
            0x4e => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD C,A | Bytes: 1  Cycle:4 Flags: - - - -
            0x4f => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD D,B | Bytes: 1  Cycle:4 Flags: - - - -	
            0x50 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD D,C | Bytes: 1  Cycle:4 Flags: - - - -	
            0x51 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD D,D | Bytes: 1  Cycle:4 Flags: - - - -	
            0x52 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD D,E | Bytes: 1  Cycle:4 Flags: - - - -	
            0x53 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD D,H | Bytes: 1  Cycle:4 Flags: - - - -	
            0x54 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD D,L | Bytes: 1  Cycle:4 Flags: - - - -	
            0x55 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD D,(HL) | Bytes: 1  Cycle:8 Flags: - - - -	
            0x56 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD D,A | Bytes: 1  Cycle:4 Flags: - - - -	
            0x57 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD E,B | Bytes: 1  Cycle:4 Flags: - - - -	
            0x58 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD E,C | Bytes: 1  Cycle:4 Flags: - - - -	
            0x59 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD E,D | Bytes: 1  Cycle:4 Flags: - - - -	
            0x5a => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD E,E | Bytes: 1  Cycle:4 Flags: - - - -	
            0x5b => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD E,H | Bytes: 1  Cycle:4 Flags: - - - -	
            0x5c => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD E,L | Bytes: 1  Cycle:4 Flags: - - - -	
            0x5d => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD E,(HL) | Bytes: 1  Cycle:8 Flags: - - - -	
            0x5e => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD E,A | Bytes: 1  Cycle:4 Flags: - - - -
            0x5f => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD H,B | Bytes: 1  Cycle:4 Flags: - - - -	
            0x60 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD H,C | Bytes: 1  Cycle:4 Flags: - - - -	
            0x61 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD H,D | Bytes: 1  Cycle:4 Flags: - - - -	
            0x62 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD H,E | Bytes: 1  Cycle:4 Flags: - - - -	
            0x63 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD H,H | Bytes: 1  Cycle:4 Flags: - - - -	
            0x64 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD H,L | Bytes: 1  Cycle:4 Flags: - - - -	
            0x65 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD H,(HL) | Bytes: 1  Cycle:8 Flags: - - - -	
            0x66 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD H,A | Bytes: 1  Cycle:4 Flags: - - - -	
            0x67 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD L,B | Bytes: 1  Cycle:4 Flags: - - - -	
            0x68 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD L,C | Bytes: 1  Cycle:4 Flags: - - - -	
            0x69 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD L,D | Bytes: 1  Cycle:4 Flags: - - - -	
            0x6a => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD L,E | Bytes: 1  Cycle:4 Flags: - - - -	
            0x6b => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD L,H | Bytes: 1  Cycle:4 Flags: - - - -	
            0x6c => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD L,L | Bytes: 1  Cycle:4 Flags: - - - -	
            0x6d => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD L,(HL) | Bytes: 1  Cycle:8 Flags: - - - -	
            0x6e => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD L,A | Bytes: 1  Cycle:4 Flags: - - - -
            0x6f => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD (HL),B | Bytes: 1  Cycle:8 Flags: - - - -	
            0x70 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD (HL),C | Bytes: 1  Cycle:8 Flags: - - - -	
            0x71 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD (HL),D | Bytes: 1  Cycle:8 Flags: - - - -	
            0x72 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD (HL),E | Bytes: 1  Cycle:8 Flags: - - - -	
            0x73 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD (HL),H | Bytes: 1  Cycle:8 Flags: - - - -	
            0x74 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD (HL),L | Bytes: 1  Cycle:8 Flags: - - - -	
            0x75 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // HALT | Bytes: 1  Cycle:4 Flags: - - - -	
            0x76 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD (HL),A | Bytes: 1  Cycle:8 Flags: - - - -	
            0x77 => self.ld_addr_hl_a(),
            // LD A,B | Bytes: 1  Cycle:4 Flags: - - - -	
            0x78 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD A,C | Bytes: 1  Cycle:4 Flags: - - - -	
            0x79 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD A,D | Bytes: 1  Cycle:4 Flags: - - - -	
            0x7a => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD A,E | Bytes: 1  Cycle:4 Flags: - - - -	
            0x7b => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD A,H | Bytes: 1  Cycle:4 Flags: - - - -	
            0x7c => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD A,L | Bytes: 1  Cycle:4 Flags: - - - -	
            0x7d => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD A,(HL) | Bytes: 1  Cycle:8 Flags: - - - -	
            0x7e => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD A,A | Bytes: 1  Cycle:4 Flags: - - - -
            0x7f => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADD A,B | Bytes: 1  Cycle:4 Flags: Z 0 H C	
            0x80 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADD A,C | Bytes: 1  Cycle:4 Flags: Z 0 H C	
            0x81 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADD A,D | Bytes: 1  Cycle:4 Flags: Z 0 H C	
            0x82 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADD A,E | Bytes: 1  Cycle:4 Flags: Z 0 H C	
            0x83 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADD A,H | Bytes: 1  Cycle:4 Flags: Z 0 H C	
            0x84 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADD A,L | Bytes: 1  Cycle:4 Flags: Z 0 H C	
            0x85 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADD A,(HL) | Bytes: 1  Cycle:8 Flags: Z 0 H C	
            0x86 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADD A,A | Bytes: 1  Cycle:4 Flags: Z 0 H C	
            0x87 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADC A,B | Bytes: 1  Cycle:4 Flags: Z 0 H C	
            0x88 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADC A,C | Bytes: 1  Cycle:4 Flags: Z 0 H C	
            0x89 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADC A,D | Bytes: 1  Cycle:4 Flags: Z 0 H C	
            0x8a => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADC A,E | Bytes: 1  Cycle:4 Flags: Z 0 H C	
            0x8b => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADC A,H | Bytes: 1  Cycle:4 Flags: Z 0 H C	
            0x8c => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADC A,L | Bytes: 1  Cycle:4 Flags: Z 0 H C	
            0x8d => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADC A,(HL) | Bytes: 1  Cycle:8 Flags: Z 0 H C	
            0x8e => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADC A,A | Bytes: 1  Cycle:4 Flags: Z 0 H C
            0x8f => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // SUB B | Bytes: 1  Cycle:4 Flags: Z 1 H C	
            0x90 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // SUB C | Bytes: 1  Cycle:4 Flags: Z 1 H C	
            0x91 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // SUB D | Bytes: 1  Cycle:4 Flags: Z 1 H C	
            0x92 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // SUB E | Bytes: 1  Cycle:4 Flags: Z 1 H C	
            0x93 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // SUB H | Bytes: 1  Cycle:4 Flags: Z 1 H C	
            0x94 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // SUB L | Bytes: 1  Cycle:4 Flags: Z 1 H C	
            0x95 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // SUB (HL) | Bytes: 1  Cycle:8 Flags: Z 1 H C	
            0x96 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // SUB A | Bytes: 1  Cycle:4 Flags: Z 1 H C	
            0x97 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // SBC A,B | Bytes: 1  Cycle:4 Flags: Z 1 H C	
            0x98 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // SBC A,C | Bytes: 1  Cycle:4 Flags: Z 1 H C	
            0x99 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // SBC A,D | Bytes: 1  Cycle:4 Flags: Z 1 H C	
            0x9a => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // SBC A,E | Bytes: 1  Cycle:4 Flags: Z 1 H C	
            0x9b => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // SBC A,H | Bytes: 1  Cycle:4 Flags: Z 1 H C	
            0x9c => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // SBC A,L | Bytes: 1  Cycle:4 Flags: Z 1 H C	
            0x9d => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // SBC A,(HL) | Bytes: 1  Cycle:8 Flags: Z 1 H C	
            0x9e => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // SBC A,A | Bytes: 1  Cycle:4 Flags: Z 1 H C
            0x9f => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // AND B | Bytes: 1  Cycle:4 Flags: Z 0 1 0	
            0xa0 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // AND C | Bytes: 1  Cycle:4 Flags: Z 0 1 0	
            0xa1 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // AND D | Bytes: 1  Cycle:4 Flags: Z 0 1 0	
            0xa2 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // AND E | Bytes: 1  Cycle:4 Flags: Z 0 1 0	
            0xa3 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // AND H | Bytes: 1  Cycle:4 Flags: Z 0 1 0	
            0xa4 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // AND L | Bytes: 1  Cycle:4 Flags: Z 0 1 0	
            0xa5 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // AND (HL) | Bytes: 1  Cycle:8 Flags: Z 0 1 0	
            0xa6 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // AND A | Bytes: 1  Cycle:4 Flags: Z 0 1 0	
            0xa7 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // XOR B | Bytes: 1  Cycle:4 Flags: Z 0 0 0	
            0xa8 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // XOR C | Bytes: 1  Cycle:4 Flags: Z 0 0 0	
            0xa9 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // XOR D | Bytes: 1  Cycle:4 Flags: Z 0 0 0	
            0xaa => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // XOR E | Bytes: 1  Cycle:4 Flags: Z 0 0 0	
            0xab => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // XOR H | Bytes: 1  Cycle:4 Flags: Z 0 0 0	
            0xac => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // XOR L | Bytes: 1  Cycle:4 Flags: Z 0 0 0	
            0xad => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // XOR (HL) | Bytes: 1  Cycle:8 Flags: Z 0 0 0	
            0xae => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // XOR A | Bytes: 1  Cycle:4 Flags: Z 0 0 0
            0xaf => { let a = self.registers.a; self.xor(a); 1},
            // OR B | Bytes: 1  Cycle:4 Flags: Z 0 0 0	
            0xb0 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // OR C | Bytes: 1  Cycle:4 Flags: Z 0 0 0	
            0xb1 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // OR D | Bytes: 1  Cycle:4 Flags: Z 0 0 0	
            0xb2 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // OR E | Bytes: 1  Cycle:4 Flags: Z 0 0 0	
            0xb3 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // OR H | Bytes: 1  Cycle:4 Flags: Z 0 0 0	
            0xb4 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // OR L | Bytes: 1  Cycle:4 Flags: Z 0 0 0	
            0xb5 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // OR (HL) | Bytes: 1  Cycle:8 Flags: Z 0 0 0	
            0xb6 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // OR A | Bytes: 1  Cycle:4 Flags: Z 0 0 0	
            0xb7 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // CP B | Bytes: 1  Cycle:4 Flags: Z 1 H C	
            0xb8 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // CP C | Bytes: 1  Cycle:4 Flags: Z 1 H C	
            0xb9 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // CP D | Bytes: 1  Cycle:4 Flags: Z 1 H C	
            0xba => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // CP E | Bytes: 1  Cycle:4 Flags: Z 1 H C	
            0xbb => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // CP H | Bytes: 1  Cycle:4 Flags: Z 1 H C	
            0xbc => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // CP L | Bytes: 1  Cycle:4 Flags: Z 1 H C	
            0xbd => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // CP (HL) | Bytes: 1  Cycle:8 Flags: Z 1 H C	
            0xbe => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // CP A | Bytes: 1  Cycle:4 Flags: Z 1 H C
            0xbf => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // RET NZ | Bytes: 1  Cycle:20/8 Flags: - - - -
            0xc0 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // POP BC | Bytes: 1  Cycle:1 Flags: - - - -	
            0xc1 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // JP NZ,a16 | Bytes: 3  Cycle:16/12 Flags: - - - -	
            0xc2 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // JP a16 | Bytes: 3  Cycle:16 Flags: - - - -	
            0xc3 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // CALL NZ,a16 | Bytes: 3  Cycle:24/12 Flags: - - - -	
            0xc4 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // PUSH BC | Bytes: 1  Cycle:16 Flags: - - - -	
            0xc5 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADD A,d8 | Bytes: 2  Cycle:8 Flags: Z 0 H C	
            0xc6 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // RST 00H | Bytes: 1  Cycle:16 Flags: - - - -	
            0xc7 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // RET Z | Bytes: 1  Cycle:20/8 Flags: - - - -	
            0xc8 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // RET | Bytes: 1  Cycle:16 Flags: - - - -	
            0xc9 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // JP Z,a16 | Bytes: 3  Cycle:16/12 Flags: - - - -	
            0xca => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // PREFIX CB | Bytes: 1  Cycle:4 Flags: - - - -	
            // HACK! This is a whole other instruction set.
            // We're just doing BIT H, 7
            0xcb => {
                let res = self.registers.h & (1 << (7 as u32)) == 0;
                self.registers.f.set(res, false, true, false);
                2
            },
            // CALL Z,a16 | Bytes: 3  Cycle:24/12 Flags: - - - -	
            0xcc => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // CALL a16 | Bytes: 3  Cycle:24 Flags: - - - -	
            // Next instruction PC on stack. Then jump
            0xcd => self.call(),
            // ADC A,d8 Bytes: 2  Cycle:8 Flags: Z 0 H C	
            0xce => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // RST 08H | Bytes: 1  Cycle:16 Flags: - - - -
            0xcf => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // RET NC | Bytes: 1  Cycle:20/8 Flags: - - - -	
            0xd0 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // POP DE | Bytes: 1  Cycle:12 Flags: - - - -	
            0xd1 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // JP NC,a16 | Bytes: 3  Cycle:16/12 Flags: - - - -	 	
            0xd2 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // Undocumented
            0xd3 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // CALL NC,a16 | Bytes: 3  Cycle:24/12 Flags: - - - -	
            0xd4 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // PUSH DE | Bytes: 1  Cycle:16 Flags: - - - -	
            0xd5 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // SUB d8 | Bytes: 2  Cycle:8 Flags: Z 1 H C	
            0xd6 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // RST 10H | Bytes: 1  Cycle:16 Flags: - - - -	
            0xd7 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // RET C | Bytes: 1  Cycle:20/8 Flags: - - - -	
            0xd8 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // RETI | Bytes: 1  Cycle:16 Flags: - - - -	
            0xd9 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // JP C,a16 | Bytes: 3  Cycle:16/12 Flags: - - - -	 	
            0xda => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // Undocumented
            0xdb => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // CALL C,a16 | Bytes: 3  Cycle:24/12 Flags: - - - -	 	
            0xdc => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // Undocumented
            0xdd => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // SBC A,d8 | Bytes: 2  Cycle:8 Flags: Z 1 H C	
            0xde => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // RST 18H | Bytes: 1  Cycle:16 Flags: - - - -
            0xdf => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LDH (a8),A | Bytes: 2  Cycle:12 Flags: - - - -	
            0xe0 => self.ld_addr_a8_a(),
            // POP HL | Bytes: 1  Cycle:12 Flags: - - - -	
            0xe1 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD (C),A | Bytes: 2  Cycle:8 Flags: - - - -	 	 	
            0xe2 => self.ld_addr_c_a(),
            // Undocumented
            0xe3 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // Undocumented
            0xe4 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // PUSH HL | Bytes: 1  Cycle:16 Flags: - - - -	
            0xe5 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // AND d8 | Bytes: 2  Cycle:8 Flags: Z 0 1 0	
            0xe6 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // RST 20H | Bytes: 1  Cycle:16 Flags: - - - -	
            0xe7 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // ADD SP,r8 | Bytes: 2  Cycle:16 Flags: 0 0 H C	
            0xe8 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // JP (HL) | Bytes: 1  Cycle:4 Flags: - - - -	
            0xe9 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD (a16),A | Bytes: 3  Cycle:16 Flags: - - - -	 	 	 	
            0xea => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // Undocumented
            0xeb => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // Undocumented
            0xec => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // Undocumented
            0xed => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // XOR d8 | Bytes: 2  Cycle:8 Flags: Z 0 0 0	
            0xee => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // RST 28H | Bytes: 1  Cycle:16 Flags: - - - -
            0xef => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LDH A,(a8) | Bytes: 2  Cycle:12 Flags: - - - -	
            0xf0 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // POP AF | Bytes: 1  Cycle:12 Flags: Z N H C	
            0xf1 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD A,(C) | Bytes: 2  Cycle:8 Flags: - - - -	
            0xf2 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // DI | Bytes: 1  Cycle:4 Flags: - - - -	 	
            0xf3 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // Undocumented
            0xf4 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // PUSH AF | Bytes: 1  Cycle:16 Flags: - - - -	
            0xf5 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // OR d8 | Bytes: 2  Cycle:8 Flags: Z 0 0 0	
            0xf6 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // RST 30H | Bytes: 1  Cycle:16 Flags: - - - -	
            0xf7 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD HL,SP+r8 | Bytes: 2  Cycle:12 Flags: 0 0 H C	
            0xf8 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD SP,HL | Bytes: 1  Cycle:8 Flags: - - - -	
            0xf9 => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // LD A,(a16) | Bytes: 3  Cycle:16 Flags: - - - -	
            0xfa => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // EI | Bytes: 1  Cycle:4 Flags: - - - -	 	 	
            0xfb => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // Undocumented
            0xfc => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // Undocumented
            0xfd => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // CP d8 | Bytes: 2  Cycle:8 Flags: Z 1 H C	
            0xfe => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            // RST 38H | Bytes: 1  Cycle:16 Flags: - - - -
            0xff => { panic!("Opcode: 0x{:02X} Instruction not implemented yet", opcode) },
            x => { panic!("Unrecognized OpCode! {:?}", x) },
        }
    }

    fn xor(&mut self, input: u8) {
        let a = self.registers.a ^ input;
        self.registers.a = a;
        self.registers.f.set(a==0, false, false, false);
    }

    fn jr_nz(&mut self) -> u16 {
        if (self.registers.f.zero == false) {
            let pc = self.registers.pc as i32; 
            // TODO: This won't work with larger jumps, need to find the proper way to cast it.
            let rel = (self.mmu.rb(self.registers.pc + 1) as i8) as i32; 
            let target = pc.wrapping_add(rel) + 2;
            self.registers.pc = target as u16;
            0
        } else {
            2
        }
    }

    fn ld_hld (&mut self) -> u16 {
        // TODO: generic way to do this.
        let mut hl = self.registers.get_hl();
        self.mmu.wb(hl, self.registers.a);
        hl -= 1;
        self.registers.set_hl(hl);
        1
    }

    fn ld_hl (&mut self) -> u16 {
         let hl = self.mmu.rw(self.registers.pc + 1);
         self.registers.set_hl(hl);
         3
    }

    fn ld_addr_c_a (&mut self) -> u16 {
        let addr_c = 0xFF00 + (self.registers.c as u16);
        let val = self.registers.a;
        self.mmu.wb(addr_c, val);
        2
    }

    fn ld_addr_a8_a (&mut self) -> u16 {
        let addr = 0xFF00 + ((self.mmu.rb(self.registers.pc + 1)) as u16);
        let val = self.registers.a;
        self.mmu.wb(addr, val);
        2
    }

    fn ld_addr_hl_a (&mut self) -> u16 {
        let addr_hl = self.registers.get_hl();
        let val = self.registers.a;
        self.mmu.wb(addr_hl, val);
        1
    }

    fn ld_e_d8 (&mut self) -> u16 {
        let val = self.mmu.rb(self.registers.pc + 1);
        self.registers.e = val;
        2
    }

    fn ld_de_d16 (&mut self) -> u16 {
        let val = self.mmu.rw(self.registers.pc + 1);
        self.registers.set_de(val);
        3
    }

    fn ld_a_addr_de (&mut self) -> u16 {
        let addr = self.registers.get_de();
        println!("addr: {:x}", addr);
        let val = self.mmu.rb(addr);
        self.registers.a = val;
        1
    }

    fn call (&mut self) -> u16 {
        let next_pc = self.registers.pc + 3;
        self.push_word(next_pc);
        let jump_to = self.mmu.rw(self.registers.pc + 1);
        self.registers.pc = jump_to;
        0
    }

    pub fn push_word(&mut self, val: u16) {
        self.registers.sp -= 2;
        self.mmu.ww(self.registers.sp, val);
    }
}
