use crate::ram::Ram;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0
        }
    }
    // Function that reads a single instruction then increments the program counter by 2
    // Because each instruction takes 2 bytes, we want to increment the program counter by 2 after each instr
    pub fn run_instruction(&mut self, ram: &mut Ram) {

        // Read the first byte in the instruction
        let lo = ram.read_byte(self.pc) as u16;
        // Read the second byte in the instruction
        let hi = ram.read_byte(self.pc + 1) as u16;
        // Create one instr of both bytes
        let instr: u16 = (lo << 8) | hi;
        println!("Instruction read: {:#X} lo: {:#X} hi: {:#X}", instr, lo, hi);
        if hi == 0 && lo == 0 {
            panic!();
        }
        self.pc += 2;
    }
}