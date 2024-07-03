//! Module for parsing assembly code into tokens using nom
//!
//! # Example
//! LOAD $0 1E
//! LOAD $1 14
//! ADD $0 $1 $3
use crate::instruction::{Opcode, OperandType};
use nom;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_until};
use nom::character::complete::{
    alphanumeric1, crlf, digit1, line_ending, multispace0, multispace1, newline, space1,
};
use nom::combinator::{map, map_res, value};
use nom::multi::{many1, separated_list0};
use nom::sequence::{preceded, terminated};
use nom::Parser;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg: u8 },
    Number { num: i32 },
}

pub fn parse_opcode(input: &str) -> nom::IResult<&str, Opcode> {
    alt((
        value(Opcode::HLT, tag("HLT")),
        value(Opcode::LOAD, tag("LOAD")),
        value(Opcode::ADD, tag("ADD")),
        value(Opcode::SUB, tag("SUB")),
        value(Opcode::MUL, tag("MUL")),
        value(Opcode::DIV, tag("DIV")),
        value(Opcode::JMP, tag("JMP")),
        value(Opcode::JMPF, tag("JMPF")),
        value(Opcode::JMPB, tag("JMPB")),
        value(Opcode::EQ, tag("EQ")),
        value(Opcode::NEQ, tag("NEQ")),
        value(Opcode::GT, tag("GT")),
        value(Opcode::LT, tag("LT")),
        value(Opcode::GTQ, tag("GTQ")),
        value(Opcode::LTQ, tag("LTQ")),
        value(Opcode::JEQ, tag("JEQ")),
        value(Opcode::JNEQ, tag("JNEQ")),
    ))(input)
}

pub fn parse_register(input: &str) -> nom::IResult<&str, Token> {
    use nom::character::complete::u8;
    let (rem, reg) = preceded(tag("$"), u8)(input)?;

    Ok((rem, Token::Register { reg }))
}

pub fn parse_number(input: &str) -> nom::IResult<&str, Token> {
    let (rem, num) = map_res(alphanumeric1, |digit_str: &str| digit_str.parse::<i32>())(input)?;

    Ok((rem, Token::Number { num }))
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
    operands: Vec<Token>,
}

impl Instruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![self.opcode.into()];

        for operand in &self.operands {
            match operand {
                Token::Register { reg } => bytes.push(*reg),
                Token::Number { num } => {
                    let converted = *num as u16;
                    let byte1 = converted;
                    let byte2 = converted >> 8;
                    bytes.push(byte2 as u8);
                    bytes.push(byte1 as u8);
                }
                _ => unimplemented!(),
            }
        }

        bytes
    }
}

pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            instructions: vec![],
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    // TODO: normal error
    pub fn from_str(input: &str) -> Result<Program, String> {
        let (_, instructions) = parse_assembly(input).map_err(|e| e.to_string())?;
        let mut program = Program::new();
        for instruction in instructions {
            program.add_instruction(instruction);
        }
        Ok(program)
    }

    pub fn from_file(file_path: impl AsRef<std::path::Path>) -> Result<Program, String> {
        let input = std::fs::read_to_string(file_path).map_err(|e| e.to_string())?;
        Program::from_str(&input)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        for instruction in &self.instructions {
            bytes.extend(instruction.to_bytes());
        }
        bytes
    }
}

pub fn parse_instruction(input: &str) -> nom::IResult<&str, Instruction> {
    let (rem, opcode) = parse_opcode(input)?;
    let (mut rem, _) = space1(rem)?;

    let opcode_operands = opcode.operands().operands;
    let mut operands = vec![];

    for operand in opcode_operands {
        // Gobble spaces
        let (rem_local, _) = multispace0(rem)?;
        let (rem_local, operand) = match operand {
            OperandType::Register => parse_register(rem_local),
            OperandType::Number => parse_number(rem_local),
        }?;
        operands.push(operand);

        rem = rem_local;
    }

    Ok((rem, Instruction { opcode, operands }))
}

// Get one line without new line characters
pub fn parse_lines(input: &str) -> nom::IResult<&str, Vec<&str>> {
    many1(map(terminated(take_until("\n"), newline), |line: &str| {
        line
    }))(input)
}

pub fn parse_assembly(input: &str) -> nom::IResult<&str, Vec<Instruction>> {
    let (rem, lines) = parse_lines(input)?;

    let mut instructions = vec![];

    for (i, line) in lines.iter().enumerate() {
        let (_, instruction) = parse_instruction(line).map_err(|e| {
            println!("Error on line {}: {}", i, e);
            e
        })?;

        instructions.push(instruction);
    }

    Ok((rem, instructions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_opcode() {
        let result = parse_opcode("LOAD $0 1");
        assert_eq!(result, Ok((" $0 1", Opcode::LOAD)));
    }

    #[test]
    fn test_parse_register() {
        let result = parse_register("$0 1");
        assert_eq!(result, Ok((" 1", Token::Register { reg: 0 })));
    }

    #[test]
    fn test_parse_number() {
        let result = parse_number("01");
        assert_eq!(result, Ok(("", Token::Number { num: 1 })));
    }

    #[test]
    fn test_parse_instruction() {
        let result = parse_instruction("LOAD $0 1");

        assert_eq!(
            result,
            Ok((
                "",
                Instruction {
                    opcode: Opcode::LOAD,
                    operands: vec![Token::Register { reg: 0 }, Token::Number { num: 1 }]
                }
            ))
        );
    }

    #[test]
    fn test_parse_lines() {
        let result = parse_lines("LOAD $0 1\nLOAD $1 2\n");

        assert_eq!(result, Ok(("", vec!["LOAD $0 1", "LOAD $1 2"])));
    }

    #[test]
    fn test_parse_assembly() {
        let result = parse_assembly("LOAD $0 1\nLOAD $1 2\n");

        assert_eq!(
            result,
            Ok((
                "",
                vec![
                    Instruction {
                        opcode: Opcode::LOAD,
                        operands: vec![Token::Register { reg: 0 }, Token::Number { num: 1 }]
                    },
                    Instruction {
                        opcode: Opcode::LOAD,
                        operands: vec![Token::Register { reg: 1 }, Token::Number { num: 2 }]
                    }
                ]
            ))
        );
    }
}
