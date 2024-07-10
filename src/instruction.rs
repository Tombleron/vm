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
    Hlt,  //
    Load, // LOAD [reg] [val] - Load a value into a register
    //                     `val` is a i32 value, `LOAD $0 10` converts to `01 00 00 0A`
    Add, // ADD [reg1] [reg2] [reg3] - Add two registers and store in a third
    Sub, // SUB [reg1] [reg2] [reg3] - Subtract two registers and store in a third
    Mul, // MUL [reg1] [reg2] [reg3] - Multiply two registers and store in a third
    Div, // DIV [reg1] [reg2] [reg3] - Divide two registers and store in a third
    //                              `rem` register holds reminder
    Jmp,  // JMP [reg] - Jump to an address stored in a register
    Jmpf, // JMPF [reg] - Jump forward from the current address
    Jmpb, // JMPB [reg] - Jump backwards from the current address

    Eq, // EQ [reg1] [reg2] - Set a register to 1 if two other registers are equal, 0 otherwise
    //                      `cmp` register holds the result
    Neq, // NEQ [reg1] [reg2] - Set a register to 1 if two other registers are not equal, 0 otherwise
    //                      `cmp` register holds the result
    Gt, // GT [reg1] [reg2] - Set a register to 1 if one register is greater than another, 0 otherwise
    //                      `cmp` register holds the result
    Lt, // LT [reg1] [reg2] - Set a register to 1 if one register is less than another, 0 otherwise
    //                      `cmp` register holds the result
    Gtq, // GTE [reg1] [reg2] - Set a register to 1 if one register is greater than or equal to another, 0 otherwise
    //                      `cmp` register holds the result
    Ltq, // LTE [reg1] [reg2] - Set a register to 1 if one register is less than or equal to another, 0 otherwise
    //                      `cmp` register holds the result
    Jeq, // JEQ [reg] - Jump to an address stored in a register if the `cmp` register is 1

    Alloc, // ALLOC [reg] - Allocate a number of bytes on the heap
    Inc,   // INC [reg] - Increment the value in a register
    Dec,   // DEC [reg] - Decrement the value in a register

    Igl, // IGL - Illegal instruction
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::Hlt,
            1 => Opcode::Load,
            2 => Opcode::Add,
            3 => Opcode::Sub,
            4 => Opcode::Mul,
            5 => Opcode::Div,
            6 => Opcode::Jmp,
            7 => Opcode::Jmpf,
            8 => Opcode::Jmpb,
            9 => Opcode::Eq,
            10 => Opcode::Neq,
            11 => Opcode::Gt,
            12 => Opcode::Lt,
            13 => Opcode::Gtq,
            14 => Opcode::Ltq,
            15 => Opcode::Jeq,

            17 => Opcode::Alloc,
            18 => Opcode::Inc,
            19 => Opcode::Dec,

            _ => Opcode::Igl,
        }
    }
}

impl From<Opcode> for u8 {
    fn from(v: Opcode) -> Self {
        match v {
            Opcode::Hlt => 0,
            Opcode::Load => 1,
            Opcode::Add => 2,
            Opcode::Sub => 3,
            Opcode::Mul => 4,
            Opcode::Div => 5,
            Opcode::Jmp => 6,
            Opcode::Jmpf => 7,
            Opcode::Jmpb => 8,
            Opcode::Eq => 9,
            Opcode::Neq => 10,
            Opcode::Gt => 11,
            Opcode::Lt => 12,
            Opcode::Gtq => 13,
            Opcode::Ltq => 14,
            Opcode::Jeq => 15,
            Opcode::Alloc => 17,
            Opcode::Inc => 18,
            Opcode::Dec => 19,
            Opcode::Igl => unreachable!(),
        }
    }
}

impl Opcode {
    pub fn size(&self) -> usize {
        match self {
            Opcode::Hlt => 1,
            Opcode::Load => 4,
            Opcode::Add => 4,
            Opcode::Sub => 4,
            Opcode::Mul => 4,
            Opcode::Div => 4,
            Opcode::Jmp => 2,
            Opcode::Jmpf => 2,
            Opcode::Jmpb => 2,
            Opcode::Eq => 3,
            Opcode::Neq => 3,
            Opcode::Gt => 3,
            Opcode::Lt => 3,
            Opcode::Gtq => 3,
            Opcode::Ltq => 3,
            Opcode::Jeq => 2,
            Opcode::Alloc => 2,
            Opcode::Inc => 2,
            Opcode::Dec => 2,
            Opcode::Igl => unreachable!(),
        }
    }

    pub fn operands(&self) -> Operands {
        match self {
            Opcode::Hlt => Operands { operands: vec![] },
            Opcode::Load => Operands {
                operands: vec![OperandType::Register, OperandType::Number],
            },
            Opcode::Add => Operands {
                operands: vec![
                    OperandType::Register,
                    OperandType::Register,
                    OperandType::Register,
                ],
            },
            Opcode::Sub => Operands {
                operands: vec![
                    OperandType::Register,
                    OperandType::Register,
                    OperandType::Register,
                ],
            },
            Opcode::Mul => Operands {
                operands: vec![
                    OperandType::Register,
                    OperandType::Register,
                    OperandType::Register,
                ],
            },
            Opcode::Div => Operands {
                operands: vec![
                    OperandType::Register,
                    OperandType::Register,
                    OperandType::Register,
                ],
            },
            Opcode::Jmp => Operands {
                operands: vec![OperandType::Register],
            },
            Opcode::Jmpf => Operands {
                operands: vec![OperandType::Register],
            },
            Opcode::Jmpb => Operands {
                operands: vec![OperandType::Register],
            },
            Opcode::Eq => Operands {
                operands: vec![OperandType::Register, OperandType::Register],
            },
            Opcode::Neq => Operands {
                operands: vec![OperandType::Register, OperandType::Register],
            },
            Opcode::Gt => Operands {
                operands: vec![OperandType::Register, OperandType::Register],
            },
            Opcode::Lt => Operands {
                operands: vec![OperandType::Register, OperandType::Register],
            },
            Opcode::Gtq => Operands {
                operands: vec![OperandType::Register, OperandType::Register],
            },
            Opcode::Ltq => Operands {
                operands: vec![OperandType::Register, OperandType::Register],
            },
            Opcode::Jeq => Operands {
                operands: vec![OperandType::Register],
            },
            Opcode::Alloc => Operands {
                operands: vec![OperandType::Register],
            },
            Opcode::Inc => Operands {
                operands: vec![OperandType::Register],
            },
            Opcode::Dec => Operands {
                operands: vec![OperandType::Register],
            },
            Opcode::Igl => unreachable!(),
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
        let instruction = Instruction::new(Opcode::Hlt);
        assert_eq!(instruction.opcode, Opcode::Hlt);
    }
}
