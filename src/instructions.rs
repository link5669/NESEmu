use crate::addressing_modes::AddressingMode;
use lazy_static::lazy_static;
pub struct Instruction {
    name: String,
    opcode: u8,
    addressing_mode: AddressingMode
}

impl Instruction {
    fn new(name: String, opcode: u8, addressing_mode: AddressingMode) -> Self{
        Instruction {
            name,
            opcode,
            addressing_mode
        }
    }
}
lazy_static! {
    pub static ref INSTRUCTIONS: Vec<Instruction> = vec![
        Instruction::new("LDA".to_string(), 0xA9, AddressingMode::IMMEDIATE)
        // match instruction {
        //     0xA9 => {
        //         let param = self.mem_read(self.program_counter);
        //         self.accumulator_register = param;
        //         self.update_flags_lda();
        //         self.program_counter +=1;
        //     }
        //     0xA5 => {
        //         let addr = self.mem_read(self.program_counter) as u16;
        //         let val = self.mem_read(addr);
        //         self.accumulator_register = val;
        //         self.update_flags_lda();
        //         self.program_counter +=1;
        //     }
        //     0xB5 => {
        //         let mut addr = self.mem_read(self.program_counter) as u16;
        //         addr = addr.wrapping_add(self.x_register as u16);
        //         let val = self.mem_read(addr);
        //         self.accumulator_register = val;
        //         self.update_flags_lda();
        //         self.program_counter +=1;
        //     }
        //     0xAD => {
        //         let param = self.mem_read_u16(self.program_counter);
        //         let val = self.mem_read(param);
        //         self.accumulator_register = val;
        //         self.update_flags_lda();
        //         self.program_counter +=2;
        //     }
        //     0xBD => {
        //         let mut addr = self.mem_read_u16(self.program_counter);
        //         addr = addr.wrapping_add(self.x_register as u16);
        //         let val = self.mem_read(addr);
        //         self.accumulator_register = val;
        //         self.update_flags_lda();
        //         self.program_counter +=2;
        //     }
        //     0xB9 => {
        //         let mut addr = self.mem_read_u16(self.program_counter);
        //         addr = addr.wrapping_add(self.y_register as u16);
        //         let val = self.mem_read(addr);
        //         self.accumulator_register = val;
        //         self.update_flags_lda();
        //         self.program_counter +=2;
        //     }
        //     0xA1 => {
        //         let base = self.mem_read(self.program_counter);
        //         let ptr: u8 = (base as u8).wrapping_add(self.x_register);
        //         let lo = self.mem_read(ptr as u16);
        //         let hi = self.mem_read(ptr.wrapping_add(1) as u16);
        //         let addr = (hi as u16) << 8 | (lo as u16);
        //         let val = self.mem_read(addr);
        //         self.accumulator_register = val;
        //         self.update_flags_lda();
        //         self.program_counter +=2;
        //     }
        //     0xB1 => {
        //         let base = self.mem_read(self.program_counter);
        //         let ptr: u8 = (base as u8).wrapping_add(self.y_register);
        //         let lo = self.mem_read(ptr as u16);
        //         let hi = self.mem_read(ptr.wrapping_add(1) as u16);
        //         let addr = (hi as u16) << 8 | (lo as u16);
        //         let val = self.mem_read(addr);
        //         self.accumulator_register = val;
        //         self.update_flags_lda();
        //         self.program_counter +=2;
        //     }
        //     0x00 => {
        //         return;
        //     }
        //     0xF8 => {
        //         self.flags_register = self.flags_register | 0b00001000;
        //     }
        //     0x78 => {
        //         self.flags_register = self.flags_register | 0b00000100;
        //     }
        //     0xAA => {
        //         self.x_register = self.accumulator_register;
        //         if self.x_register == 0 {
        //             self.flags_register = self.flags_register | 0b00000010;
        //         } else {
        //             self.flags_register = self.flags_register & 0b11111101;
        //         }
        //         if self.x_register & 0b10000000 == 1 {
        //             self.flags_register = self.flags_register | 0b10000000;
        //         } else {
        //             self.flags_register = self.flags_register & 0b01111111;
        //         }
        //     }
        //     0xE8 => {
        //         self.x_register += 1;
        //         if self.x_register == 0 {
        //             self.flags_register = self.flags_register | 0b00000010;
        //         } else {
        //             self.flags_register = self.flags_register & 0b11111101;
        //         }
        //         if self.x_register & 0b10000000 == 1 {
        //             self.flags_register = self.flags_register | 0b10000000;
        //         } else {
        //             self.flags_register = self.flags_register & 0b01111111;
        //         }
        //     }
        //     0x85 => {
        //         let addr = self.mem_read(self.program_counter) as u16;
        //         self.mem_write(addr, self.accumulator_register);
        //         self.program_counter +=1;
        //     }
        //     0x95 => {
        //         let mut addr = self.mem_read(self.program_counter) as u16;
        //         addr = addr.wrapping_add(self.x_register as u16);
        //         self.mem_write(addr, self.accumulator_register);
        //         self.program_counter +=1;
        //     }
        //     0x8D => {
        //         let param = self.mem_read_u16(self.program_counter);
        //         self.mem_write(param, self.accumulator_register);
        //         self.program_counter +=2;
        //     }
        //     0x9D => {
        //         let mut param = self.mem_read_u16(self.program_counter);
        //         param = param.wrapping_add(self.x_register as u16);
        //         self.mem_write(param, self.accumulator_register);
        //         self.program_counter +=2;
        //     }
        //     0x99 => {
        //         let mut param = self.mem_read_u16(self.program_counter);
        //         param = param.wrapping_add(self.y_register as u16);
        //         self.mem_write(param, self.accumulator_register);
        //         self.program_counter +=2;
        //     }
        //     0x81 => {
        //         let base = self.mem_read(self.program_counter);
        //         let ptr: u8 = (base as u8).wrapping_add(self.x_register);
        //         let lo = self.mem_read(ptr as u16);
        //         let hi = self.mem_read(ptr.wrapping_add(1) as u16);
        //         let addr = (hi as u16) << 8 | (lo as u16);
        //         self.mem_write(addr, self.accumulator_register);
        //         self.program_counter +=2;
        //     }
        //     0x91 => {
        //         let base = self.mem_read(self.program_counter);
        //         let ptr: u8 = (base as u8).wrapping_add(self.y_register);
        //         let lo = self.mem_read(ptr as u16);
        //         let hi = self.mem_read(ptr.wrapping_add(1) as u16);
        //         let addr = (hi as u16) << 8 | (lo as u16);
        //         self.mem_write(addr, self.accumulator_register);
        //         self.program_counter +=2;
        //     }
        //     _ => {
        //         print!("else!");
        //     }
        // }
    ];
}