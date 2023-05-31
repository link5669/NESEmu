use crate::addressing_modes::AddressingMode;
use lazy_static::lazy_static;
pub struct Instruction {
    name: &'static str,
    opcode: u8,
    addressing_mode: AddressingMode,
    increment: u8
}

impl Instruction {
    fn new(name: &'static str, opcode: u8, addressing_mode: AddressingMode, increment: u8) -> Self{
        Instruction {
            name,
            opcode,
            addressing_mode,
            increment
        }
    }
    pub fn getAddressingMode(&self) -> AddressingMode {
        self.addressing_mode
    }
    pub fn getIncrement(&self) -> u8 {
        self.increment
    }
}

lazy_static! {
    static ref INSTRUCTIONS: Vec<Instruction> = vec![
        Instruction::new("LDA", 0xA9, AddressingMode::IMMEDIATE, 1),
        Instruction::new("LDA", 0xA5, AddressingMode::ZERO_PAGE, 2),
        Instruction::new("LDA", 0xB5, AddressingMode::ZERO_PAGE_X, 2),
        Instruction::new("LDA", 0xAD, AddressingMode::ABSOLUTE, 2),
        Instruction::new("LDA", 0xBD, AddressingMode::ABSOLUTE_X, 3),
        Instruction::new("LDA", 0xB9, AddressingMode::ABSOLUTE_Y, 3),
        Instruction::new("LDA", 0xA1, AddressingMode::INDIRECT_X, 2),
        Instruction::new("LDA", 0xB1, AddressingMode::INDIRECT_Y, 2),
        Instruction::new("BRK", 0x00, AddressingMode::IMPLIED, 1),
        Instruction::new("SEI", 0x78, AddressingMode::IMPLIED, 1),
        Instruction::new("SED", 0xF8, AddressingMode::IMPLIED, 1),
        Instruction::new("TAX", 0xAA, AddressingMode::IMPLIED, 1),
        Instruction::new("INX", 0xE8, AddressingMode::IMPLIED, 1),
        Instruction::new("STA", 0x85, AddressingMode::ZERO_PAGE, 2),
        Instruction::new("STA", 0x95, AddressingMode::ZERO_PAGE_X, 2),
        Instruction::new("STA", 0x8D, AddressingMode::ABSOLUTE, 3),
        Instruction::new("STA", 0x9D, AddressingMode::ABSOLUTE_X, 3),
        Instruction::new("STA", 0x99, AddressingMode::ABSOLUTE_Y, 3),
        Instruction::new("STA", 0x81, AddressingMode::INDIRECT_X, 2),
        Instruction::new("STA", 0x91, AddressingMode::INDIRECT_Y, 2),
        Instruction::new("ADC", 0x69, AddressingMode::IMMEDIATE, 2),
        Instruction::new("ADC", 0x65, AddressingMode::ZERO_PAGE, 2),
        Instruction::new("ADC", 0x75, AddressingMode::ZERO_PAGE_X, 2),
        Instruction::new("ADC", 0x6D, AddressingMode::ABSOLUTE, 3),
        Instruction::new("ADC", 0x7D, AddressingMode::ABSOLUTE_X, 3),
        Instruction::new("ADC", 0x79, AddressingMode::ABSOLUTE_Y, 3),
        Instruction::new("ADC", 0x61, AddressingMode::INDIRECT_X, 2),
        Instruction::new("ADC", 0x71, AddressingMode::INDIRECT_Y, 2),
        Instruction::new("AND", 0x29, AddressingMode::IMMEDIATE, 2),
        Instruction::new("AND", 0x25, AddressingMode::ZERO_PAGE, 2),
        Instruction::new("AND", 0x35, AddressingMode::ZERO_PAGE_X, 2),
        Instruction::new("AND", 0x2D, AddressingMode::ABSOLUTE, 3),
        Instruction::new("AND", 0x3D, AddressingMode::ABSOLUTE_X, 3),
        Instruction::new("AND", 0x39, AddressingMode::ABSOLUTE_Y, 3),
        Instruction::new("AND", 0x21, AddressingMode::INDIRECT_X, 2),
        Instruction::new("AND", 0x31, AddressingMode::INDIRECT_Y, 2),
    ];
}

pub fn find_instruction_by_opcode(opcode: u8) -> Option<&Instruction> {
    INSTRUCTIONS.iter().find(|instruction| instruction.opcode == opcode)
}