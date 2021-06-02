use crate::ram::Ram;
use std::fmt;
use std::fmt::Formatter;

pub const PROGRAM_START: u16 = 0x200;

//#[derive(Debug)]
pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
    prev_pc: u16
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
            prev_pc: 0,
        }
    }
    // Function that reads a single instruction then increments the program counter by 2
    // Because each instruction takes 2 bytes, we want to increment the program counter by 2 after each instr
    pub fn run_instruction(&mut self, ram: &mut Ram) {

        // Read the first byte in the instruction
        let hi = ram.read_byte(self.pc) as u16;
        // Read the second byte in the instruction
        let lo = ram.read_byte(self.pc + 1) as u16;
        // Create one instr of both bytes
        let instr: u16 = (hi << 8) | lo;


        println!("Instruction read: {:#X} lo: {:#X} hi: {:#X}", instr, lo, hi);

        let nnn = instr & 0x0FFF;
        let nn = (instr & 0x0FF) as u8;
        let n = (instr & 0x00F) as u8;
        let x = ((instr & 0x0F00) >> 8) as u8;
        let y = ((instr & 0x00F0) >> 4) as u8;
        println!("nnn: {:?}, nn: {:?}, n: {:?}, x: {:?}, y:{:?}", nnn, nn, n, x, y);

        // Error checking to make sure the pc is incremented after each instruction
        if self.prev_pc == self.pc {
            panic!("Increment pc!");
        }
        self.prev_pc = self.pc;

        match (instr & 0xF000) >> 12 {
            0x1 => {
                // jmp to nnn
                self.pc = nnn;
            },
            0x3 => {
                // if(Vx == nn)
                let vx = self.read_reg_vx(x);
                if vx == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x6 => {
                // set vx to nn
                self.write_reg_vx(x, nn);
                self.pc += 2;
            },
            0x7 => {
                // add nn to vx (no change to carry flag)
                let vx = self.read_reg_vx(x);
                self.write_reg_vx(x, vx.wrapping_add(nn));
                self.pc += 2;
            },
            0xA => {
                // set i to nnn
                self.i = nnn;
                self.pc += 2;
            },
            0xD => {
                // draw sprite at location x, y
                self.debug_draw_sprite(ram, x, y,n);
                self.pc += 2;
            },
            0xF => {
                // i += Vx
                let vx = self.read_reg_vx(x);
                self.i += vx as u16;
                self.pc += 2;
            }

            _ => panic!("Unrecognized instruction: {:#X}, {:#X}", self.pc, instr)
        }

    }
    pub fn debug_draw_sprite(&self, ram: &mut Ram, x: u8, y: u8,  height: u8) {
        println!("Drawing sprite at ({}, {})", x, y);
        for y in 0..height {
            let mut b = ram.read_byte(self.i + y as u16);
            for _ in 0..8 {
                match (b & 0b1000_0000) >> 7 {
                    0 => print!("_"),
                    1 => print!("#"),
                    _ => unreachable!()
                }
                b = b << 1;
            }
            print!("\n");
        }

        print!("\n");
    }
    pub fn write_reg_vx(&mut self, index: u8, value: u8) {
        self.vx[index as usize] = value;
    }
    pub fn read_reg_vx(&mut self, index: u8) -> u8 {
        self.vx[index as usize]
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {

        write!(f, "{:#X}\n", self.pc);
        write!(f, "vx: ");
        for item in self.vx.iter() {
            write!(f, "{:#X}", *item);
        }
        write!(f, "\n");
        write!(f, "i: {:#X}\n", self.i)
    }
}