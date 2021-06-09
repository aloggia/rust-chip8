use crate::bus::Bus;
use std::fmt;
use std::fmt::Formatter;
use std::borrow::Borrow;


pub const PROGRAM_START: u16 = 0x200;

//#[derive(Debug)]
pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
    prev_pc: u16,
    ret_stack: Vec<u16>
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
            prev_pc: 0,
            ret_stack: Vec::<u16>::new()
        }
    }
    // Function that reads a single instruction then increments the program counter by 2
    // Because each instruction takes 2 bytes, we want to increment the program counter by 2 after each instr
    pub fn run_instruction(&mut self, bus: &mut Bus) {

        // Read the first byte in the instruction
        let hi = bus.ram_read_byte(self.pc) as u16;
        // Read the second byte in the instruction
        let lo = bus.ram_read_byte(self.pc + 1) as u16;
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
            0x0 => {
                match nn {
                    0xE0 => {
                        // Clear screen
                        bus.clear_screen();
                        self.pc += 2;
                    }
                    0xEE => {
                        // Return from suproutine
                        let addr = self.ret_stack.pop().unwrap();
                        self.pc = addr;
                    }
                    _ => panic!("Unrecognized '0x00*' instruction: {:#X}, {:#X}", self.pc, instr)
                }
            },
            0x1 => {
                // jmp to nnn
                self.pc = nnn;
            },
            0x2 => {
                // Call subroutine at nnn
                self.ret_stack.push(self.pc + 2);
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
            0x8 => {
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);
                match n {
                    0 => {
                        // set VX = VY
                        self.write_reg_vx(x, vy);
                    },
                    1 => {
                        // set VX to VX or VY (bitwise or)
                        self.write_reg_vx(x, vx | vy);
                    },
                    2 => {
                        // set VX to VX and VY (bitwise and)
                        self.write_reg_vx(x, vx & vy);
                    },
                    3 => {
                        // set VX to VX xor VY (bitwise xor)
                        self.write_reg_vx(x, vx ^ vy);
                    },
                    4 => {
                        // add VY to VX, VF is set to 1 if there is a carry, set VF to 0 otherwise
                        let sum: u16 = vx as u16 + vy as u16;
                        self.write_reg_vx(x, sum as u8);
                        if sum > 0xFF {
                            self.write_reg_vx(0xF, 1);
                        } else {
                            self.write_reg_vx(0xF, 0);
                        }
                    },
                    5 => {
                        // subtract VY from VX, VF is set to 0 when theres a borrow, and 1 otherwise
                        let diff: i8 = vx as i8 - vy as i8;
                        self.write_reg_vx(x, diff as u8);
                        if diff < 0 {
                            self.write_reg_vx(0xF, 1);
                        } else {
                            self.write_reg_vx(0xF, 0);
                        }
                    },
                    6 => {
                        // bit shift VY right one and copy that result into VX
                        // VF is set to the least significant bit in VY BEFORE the shift
                        self.write_reg_vx(0xF, vy & 0x1);
                        self.write_reg_vx(y, vy >> 1);
                        self.write_reg_vx(x, vy >> 1);

                    },
                    7 => {

                    },

                    _ => panic!("Unrecognized '0x8XY*' instruction: {:#X}, {:#X}", self.pc, instr)
                }
                self.pc += 2;
            },
            0xA => {
                // set i to nnn
                self.i = nnn;
                self.pc += 2;
            },
            0xD => {
                // draw sprite at location x, y
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);
                self.debug_draw_sprite(bus, vx, vy,n);
                self.pc += 2;
            },
            0xE => {
                //TODO: clean up this code: they do almost the same thing
                match nn {
                    0x9E => {
                        // skip the next instr if the key stored in is pressed
                        let key = self.read_reg_vx(x);
                        if bus.key_pressed(key) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    },
                    0xA1 => {
                        // skip the next instr if the key stored in isnt pressed
                        let key = self.read_reg_vx(x);
                        if !bus.key_pressed(key) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }

                    }
                    _ => panic!("Unrecognized '0xEX**' instruction: {:#X}, {:#X}", self.pc, instr)
                }
            },
            0xF => {
                match nn {
                    0x07 => {
                        // Set VX to the value of the delay timer
                        self.write_reg_vx(x, bus.get_delay_timer());
                        self.pc += 2;
                    },
                    0x0A => {

                    },
                    0x15 => {
                        // set the delay timer to VX
                        bus.set_delay_timer(self.read_reg_vx(x));
                        self.pc += 2;
                    },
                    0x65 => {
                        // Fill V0 to VX with values from memory starting at location I
                        for index in 0..x {
                            let value = bus.ram_read_byte(self.i + index as u16);
                            self.write_reg_vx(index, value);
                        }
                        self.pc += 2;
                    },
                    0x1E => {
                        // i += Vx
                        let vx = self.read_reg_vx(x);
                        self.i += vx as u16;
                        self.pc += 2;
                    }

                    _ => panic!("Unrecognized '0xFX**' instruction: {:#X}, {:#X}", self.pc, instr)
                }

            },

            _ => panic!("Unrecognized instruction: {:#X}, {:#X}", self.pc, instr)
        }

    }
    pub fn debug_draw_sprite(&mut self, bus: &mut Bus, x: u8, y: u8, height: u8) {
        println!("Drawing sprite at ({}, {})", x, y);
        let mut should_set_vf = false;
        for y in 0..height {
            let b = bus.ram_read_byte(self.i + y as u16);
            if bus.debug_draw_byte(b, x, y) {
                should_set_vf = true;
            }
            bus.debug_draw_byte(b, x, y);
        }
        if should_set_vf {
            self.write_reg_vx(0xF, 1)
        } else {
            self.write_reg_vx(0xF, 0);
        }
        bus.present_screen();
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
        write!(f, "\n{:#X}\n", self.pc);
        write!(f, "vx: ");
        for item in self.vx.iter() {
            write!(f, "{:#X}", *item);
        }
        write!(f, "\n");
        write!(f, "i: {:#X}\n", self.i)
    }
}