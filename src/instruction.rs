#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,    //
    LOAD,   // LOAD [reg] [val] - Load a value into a register
    ADD,    // ADD [reg1] [reg2] [reg3] - Add two registers and store in a third
    SUB,    // SUB [reg1] [reg2] [reg3] - Subtract two registers and store in a third
    MUL,    // MUL [reg1] [reg2] [reg3] - Multiply two registers and store in a third
    DIV,    // DIV [reg1] [reg2] [reg3] - Divide two registers and store in a third
            //                              `rem` register holds reminder
    JMP,    // JMP [reg] - Jump to an address stored in a register
    JMPF,   // JMPF [reg] - Jump forward from the current address
    JMPB,   // JMPB [reg] - Jump backwards from the current address

    EQ,     // EQ [reg1] [reg2] - Set a register to 1 if two other registers are equal, 0 otherwise
            //                      `cmp` register holds the result
    NEQ,    // NEQ [reg1] [reg2] - Set a register to 1 if two other registers are not equal, 0 otherwise
            //                      `cmp` register holds the result
    GT,     // GT [reg1] [reg2] - Set a register to 1 if one register is greater than another, 0 otherwise
            //                      `cmp` register holds the result
    LT,     // LT [reg1] [reg2] - Set a register to 1 if one register is less than another, 0 otherwise
            //                      `cmp` register holds the result
    GTQ,    // GTE [reg1] [reg2] - Set a register to 1 if one register is greater than or equal to another, 0 otherwise
            //                      `cmp` register holds the result
    LTQ,    // LTE [reg1] [reg2] - Set a register to 1 if one register is less than or equal to another, 0 otherwise
            //                      `cmp` register holds the result
    JEQ,    // JEQ [reg] - Jump to an address stored in a register if the `cmp` register is 1
    JNEQ,   // JNEQ [reg] - Jump to an address stored in a register if the `cmp` register is 0

    IGL,    // IGL - Illegal instruction
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
