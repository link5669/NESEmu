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
  
    pub fn run(&mut self) {
        loop {
            let instruction = self.mem_read(self.program_counter);
            self.program_counter += 1;
            match instruction {
                0xA9 => {
                    let param = self.mem_read(self.program_counter);
                    self.accumulator_register = param;
                    self.update_flags_lda();
                    self.program_counter +=1;
                }
                //zero page
                0xA5 => {
                    let addr = self.mem_read(self.program_counter) as u16;
                    let val = self.mem_read(addr);
                    self.accumulator_register = val;
                    self.update_flags_lda();
                    self.program_counter +=1;
                }
                0xB5 => {
                    let mut addr = self.mem_read(self.program_counter) as u16;
                    addr = addr.wrapping_add(self.x_register as u16);
                    let val = self.mem_read(addr);
                    self.accumulator_register = val;
                    self.update_flags_lda();
                    self.program_counter +=1;
                }
                0xAD => {
                    let param = self.mem_read_u16(self.program_counter);
                    let val = self.mem_read(param);
                    self.accumulator_register = val;
                    self.update_flags_lda();
                    self.program_counter +=2;
                }
                0xBD => {
                    let mut addr = self.mem_read_u16(self.program_counter);
                    addr = addr.wrapping_add(self.x_register as u16);
                    let val = self.mem_read(addr);
                    self.accumulator_register = val;
                    self.update_flags_lda();
                    self.program_counter +=2;
                }
                0xB9 => {
                    let mut addr = self.mem_read_u16(self.program_counter);
                    addr = addr.wrapping_add(self.y_register as u16);
                    let val = self.mem_read(addr);
                    self.accumulator_register = val;
                    self.update_flags_lda();
                    self.program_counter +=2;
                }
                0xA1 => {
                    let base = self.mem_read(self.program_counter);
                    let ptr: u8 = (base as u8).wrapping_add(self.x_register);
                    let lo = self.mem_read(ptr as u16);
                    let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                    let addr = (hi as u16) << 8 | (lo as u16);
                    let val = self.mem_read(addr);
                    self.accumulator_register = val;
                    self.update_flags_lda();
                    self.program_counter +=2;
                }
                0xB1 => {
                    let base = self.mem_read(self.program_counter);
                    let ptr: u8 = (base as u8).wrapping_add(self.y_register);
                    let lo = self.mem_read(ptr as u16);
                    let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                    let addr = (hi as u16) << 8 | (lo as u16);
                    let val = self.mem_read(addr);
                    self.accumulator_register = val;
                    self.update_flags_lda();
                    self.program_counter +=2;
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
                0x85 => {
                    let addr = self.mem_read(self.program_counter) as u16;
                    self.mem_write(addr, self.accumulator_register);
                    self.program_counter +=1;
                }
                0x95 => {
                    let mut addr = self.mem_read(self.program_counter) as u16;
                    addr = addr.wrapping_add(self.x_register as u16);
                    self.mem_write(addr, self.accumulator_register);
                    self.program_counter +=1;
                }
                0x8D => {
                    let param = self.mem_read_u16(self.program_counter);
                    self.mem_write(param, self.accumulator_register);
                    self.program_counter +=2;
                }
                0x9D => {
                    let mut param = self.mem_read_u16(self.program_counter);
                    param = param.wrapping_add(self.x_register as u16);
                    self.mem_write(param, self.accumulator_register);
                    self.program_counter +=2;
                }
                0x99 => {
                    let mut param = self.mem_read_u16(self.program_counter);
                    param = param.wrapping_add(self.y_register as u16);
                    self.mem_write(param, self.accumulator_register);
                    self.program_counter +=2;
                }
                0x81 => {
                    let base = self.mem_read(self.program_counter);
                    let ptr: u8 = (base as u8).wrapping_add(self.x_register);
                    let lo = self.mem_read(ptr as u16);
                    let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                    let addr = (hi as u16) << 8 | (lo as u16);
                    self.mem_write(addr, self.accumulator_register);
                    self.program_counter +=2;
                }
                0x91 => {
                    let base = self.mem_read(self.program_counter);
                    let ptr: u8 = (base as u8).wrapping_add(self.y_register);
                    let lo = self.mem_read(ptr as u16);
                    let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                    let addr = (hi as u16) << 8 | (lo as u16);
                    self.mem_write(addr, self.accumulator_register);
                    self.program_counter +=2;
                }
                _ => {
                    print!("else!");
                }
            }
        }
    }
 }

 fn main() {let mut cpu = CPU::new();
    // cpu.run(vec![0xa9, 0x05, 0x00]);
 }
