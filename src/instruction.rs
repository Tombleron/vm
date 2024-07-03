#[derive(Debug, PartialEq, Clone)]
pub enum OperandType {
    Register,
    Number,
}

pub struct Operands {
    pub operands: Vec<OperandType>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    HLT,  //
    LOAD, // LOAD [reg] [val] - Load a value into a register
    //                     `val` is a i32 value, `LOAD $0 10` converts to `01 00 00 0A`
    ADD, // ADD [reg1] [reg2] [reg3] - Add two registers and store in a third
    SUB, // SUB [reg1] [reg2] [reg3] - Subtract two registers and store in a third
    MUL, // MUL [reg1] [reg2] [reg3] - Multiply two registers and store in a third
    DIV, // DIV [reg1] [reg2] [reg3] - Divide two registers and store in a third
    //                              `rem` register holds reminder
    JMP,  // JMP [reg] - Jump to an address stored in a register
    JMPF, // JMPF [reg] - Jump forward from the current address
    JMPB, // JMPB [reg] - Jump backwards from the current address

    EQ, // EQ [reg1] [reg2] - Set a register to 1 if two other registers are equal, 0 otherwise
    //                      `cmp` register holds the result
    NEQ, // NEQ [reg1] [reg2] - Set a register to 1 if two other registers are not equal, 0 otherwise
    //                      `cmp` register holds the result
    GT, // GT [reg1] [reg2] - Set a register to 1 if one register is greater than another, 0 otherwise
    //                      `cmp` register holds the result
    LT, // LT [reg1] [reg2] - Set a register to 1 if one register is less than another, 0 otherwise
    //                      `cmp` register holds the result
    GTQ, // GTE [reg1] [reg2] - Set a register to 1 if one register is greater than or equal to another, 0 otherwise
    //                      `cmp` register holds the result
    LTQ, // LTE [reg1] [reg2] - Set a register to 1 if one register is less than or equal to another, 0 otherwise
    //                      `cmp` register holds the result
    JEQ,  // JEQ [reg] - Jump to an address stored in a register if the `cmp` register is 1
    JNEQ, // JNEQ [reg] - Jump to an address stored in a register if the `cmp` register is 0

    IGL, // IGL - Illegal instruction
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::HLT,
            1 => Opcode::LOAD,
            2 => Opcode::ADD,
            3 => Opcode::SUB,
            4 => Opcode::MUL,
            5 => Opcode::DIV,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GT,
            12 => Opcode::LT,
            13 => Opcode::GTQ,
            14 => Opcode::LTQ,
            15 => Opcode::JEQ,
            16 => Opcode::JNEQ,

            _ => Opcode::IGL,
        }
    }
}

impl From<Opcode> for u8 {
    fn from(v: Opcode) -> Self {
        match v {
            Opcode::HLT => 0,
            Opcode::LOAD => 1,
            Opcode::ADD => 2,
            Opcode::SUB => 3,
            Opcode::MUL => 4,
            Opcode::DIV => 5,
            Opcode::JMP => 6,
            Opcode::JMPF => 7,
            Opcode::JMPB => 8,
            Opcode::EQ => 9,
            Opcode::NEQ => 10,
            Opcode::GT => 11,
            Opcode::LT => 12,
            Opcode::GTQ => 13,
            Opcode::LTQ => 14,
            Opcode::JEQ => 15,
            Opcode::JNEQ => 16,
            Opcode::IGL => unreachable!(),
        }
    }
}

impl Opcode {
    pub fn size(&self) -> usize {
        match self {
            Opcode::HLT => 1,
            Opcode::LOAD => 4,
            Opcode::ADD => 4,
            Opcode::SUB => 4,
            Opcode::MUL => 4,
            Opcode::DIV => 4,
            Opcode::JMP => 2,
            Opcode::JMPF => 2,
            Opcode::JMPB => 2,
            Opcode::EQ => 3,
            Opcode::NEQ => 3,
            Opcode::GT => 3,
            Opcode::LT => 3,
            Opcode::GTQ => 3,
            Opcode::LTQ => 3,
            Opcode::JEQ => 2,
            Opcode::JNEQ => 2,
            Opcode::IGL => unreachable!(),
        }
    }

    pub fn operands(&self) -> Operands {
        match self {
            Opcode::HLT => Operands { operands: vec![] },
            Opcode::LOAD => Operands {
                operands: vec![OperandType::Register, OperandType::Number],
            },
            Opcode::ADD => Operands {
                operands: vec![
                    OperandType::Register,
                    OperandType::Register,
                    OperandType::Register,
                ],
            },
            Opcode::SUB => Operands {
                operands: vec![
                    OperandType::Register,
                    OperandType::Register,
                    OperandType::Register,
                ],
            },
            Opcode::MUL => Operands {
                operands: vec![
                    OperandType::Register,
                    OperandType::Register,
                    OperandType::Register,
                ],
            },
            Opcode::DIV => Operands {
                operands: vec![
                    OperandType::Register,
                    OperandType::Register,
                    OperandType::Register,
                ],
            },
            Opcode::JMP => Operands {
                operands: vec![OperandType::Register],
            },
            Opcode::JMPF => Operands {
                operands: vec![OperandType::Register],
            },
            Opcode::JMPB => Operands {
                operands: vec![OperandType::Register],
            },
            Opcode::EQ => Operands {
                operands: vec![OperandType::Register, OperandType::Register],
            },
            Opcode::NEQ => Operands {
                operands: vec![OperandType::Register, OperandType::Register],
            },
            Opcode::GT => Operands {
                operands: vec![OperandType::Register, OperandType::Register],
            },
            Opcode::LT => Operands {
                operands: vec![OperandType::Register, OperandType::Register],
            },
            Opcode::GTQ => Operands {
                operands: vec![OperandType::Register, OperandType::Register],
            },
            Opcode::LTQ => Operands {
                operands: vec![OperandType::Register, OperandType::Register],
            },
            Opcode::JEQ => Operands {
                operands: vec![OperandType::Register],
            },
            Opcode::JNEQ => Operands {
                operands: vec![OperandType::Register],
            },
            Opcode::IGL => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
