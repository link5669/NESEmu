pub struct CPU {
    accumulator_register: u8,
    x_register: u8,
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
  
    pub fn run(&mut self) {
        loop {
            let opscode = self.mem_read(self.program_counter);
            self.program_counter += 1;
            match opscode {
                0xA9 => {
                    let param = self.mem_read(self.program_counter);
                    self.program_counter +=1;
                    self.accumulator_register = param;
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
