use crate::{addressing_modes::AddressingMode, instructions::{find_instruction_by_name, Instruction}};

pub struct CPU {
    accumulator_register: u8,
    x_register: u8,
    y_register: u8,
    flags_register: u8,
    //BIT 7: NEGATIVE
    //BIT 6: OVERFLOW
    //BIT 5: b flag???
    //BIT 4: b flag???
    //BIT 3: DECIMAL
    //BIT 2: INTERRUPT DISABLE
    //BIT 1: ZERO
    //BIT 0: CARRY
    program_counter: u16,
    memory: [u8; 0xFFFF]
 }
  
 impl CPU {
    pub fn new() -> Self {
        CPU {
            accumulator_register: 0,
            x_register: 0,
            y_register: 0,
            flags_register: 0,
            program_counter: 0,
            memory: [0u8; 0xFFFF]
        }
    } 

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }
 
    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }
 

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn reset(&mut self) {
        self.accumulator_register = 0;
        self.x_register = 0;
        self.flags_register = 0;
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    fn update_flags_lda(&mut self) {
        if self.accumulator_register == 0 {
            self.flags_register = self.flags_register | 0b00000010;
        } else {
            self.flags_register = self.flags_register & 0b11111101;
        }
        if self.accumulator_register & 0b10000000 == 1 {
            self.flags_register = self.flags_register | 0b10000000;
        } else {
            self.flags_register = self.flags_register & 0b01111111;
        }
    }

    fn match_addressing_mode(&mut self, instr: Instruction) -> u16 {
        match instr.getAddressingMode() {
            AddressingMode::IMMEDIATE => {
                self.program_counter
            }
            AddressingMode::ZERO_PAGE => {
                self.mem_read(self.program_counter) as u16
            }
            AddressingMode::ZERO_PAGE_X => {
                let addr = self.mem_read(self.program_counter) as u16;
                addr.wrapping_add(self.x_register as u16)
            }
            AddressingMode::ABSOLUTE => {
                self.mem_read_u16(self.program_counter)
            }
            AddressingMode::ABSOLUTE_X => {
                let addr = self.mem_read_u16(self.program_counter);
                addr.wrapping_add(self.x_register as u16)
            }
            AddressingMode::ABSOLUTE_Y => {
                let addr = self.mem_read_u16(self.program_counter);
                addr.wrapping_add(self.y_register as u16)
            }
            AddressingMode::INDIRECT_X => {
                let base = self.mem_read(self.program_counter);
                let ptr: u8 = (base as u8).wrapping_add(self.x_register);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::INDIRECT_Y => {
                let base = self.mem_read(self.program_counter);
                let ptr: u8 = (base as u8).wrapping_add(self.y_register);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
        } 
    }

    fn lda(&mut self, instr: Instruction) {
        let val = self.mem_read(self.match_addressing_mode(instr));
        self.accumulator_register = val;
        self.update_flags_lda();
    }

    fn sta(&mut self, instr: Instruction) {
        self.mem_write(self.match_addressing_mode(instr), self.accumulator_register);
    }

    fn adc(&mut self, instr: Instruction) {
        let addr = self.match_addressing_mode(instr);
        let val = self.mem_read(addr);
        let sum: u16 = val as u16 + self.accumulator_register as u16 + (self.flags_register & 0b00000001) as u16;
        if sum > 0xFF {
            self.flags_register |= 0b00000001;
        } else {
            self.flags_register &= 0b11111110;
        }
        if (val ^ sum as u8) & (sum as u8 ^ self.accumulator_register) & 0x80 != 0 {
            self.flags_register |= 0b01000000;
        } else {
            self.flags_register &= 0b10111111;
        }
        self.accumulator_register = sum as u8;
    }

    fn add(&mut self, instr: Instruction) {
        let addr = self.match_addressing_mode(instr);
        let val = self.mem_read(addr);
        self.accumulator_register &= val;
        if self.accumulator_register == 0 {
            self.flags_register |= 0b00000010;
        } else {
            self.flags_register &= 0b11111101;
        }
    }
  
    pub fn run(&mut self) {
        loop {
            let mut instruction = find_instruction_by_name(self.mem_read(self.program_counter));
            self.program_counter += 1;
            match instruction.opcode {
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(instruction);
                }
                0x00 => {
                    return;
                }
                0xF8 => {
                    self.flags_register = self.flags_register | 0b00001000;
                }
                0x78 => {
                    self.flags_register = self.flags_register | 0b00000100;
                }
                0xAA => {
                    self.x_register = self.accumulator_register;
                    if self.x_register == 0 {
                        self.flags_register = self.flags_register | 0b00000010;
                    } else {
                        self.flags_register = self.flags_register & 0b11111101;
                    }
                    if self.x_register & 0b10000000 == 1 {
                        self.flags_register = self.flags_register | 0b10000000;
                    } else {
                        self.flags_register = self.flags_register & 0b01111111;
                    }
                }
                0xE8 => {
                    self.x_register += 1;
                    if self.x_register == 0 {
                        self.flags_register = self.flags_register | 0b00000010;
                    } else {
                        self.flags_register = self.flags_register & 0b11111101;
                    }
                    if self.x_register & 0b10000000 == 1 {
                        self.flags_register = self.flags_register | 0b10000000;
                    } else {
                        self.flags_register = self.flags_register & 0b01111111;
                    }
                }
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.sta(instruction);
                }
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                    self.adc(instruction);
                }
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    self.add(instruction);
                }
                _ => {
                    print!("else!");
                }
            }
            self.program_counter += instruction.getIncrement();
        }
    }
 }

 fn main() {let mut cpu = CPU::new();
    // cpu.run(vec![0xa9, 0x05, 0x00]);


 }
