use std::collections::HashMap;
use std::io::{Read, Write};
use std::str::FromStr;

use log::debug;
use thiserror::Error;

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Write,
    Read,
    JumpLeft,
    JumpRight,
}

impl Instruction {
    fn is_token(c: &char) -> bool {
        let token_chars = ['<', '>', '+', '-', '.', ',', '[', ']'];
        token_chars.contains(&c)
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("encountered unexpected character during parse")]
    UnexpectedCharacter,
}

impl TryFrom<char> for Instruction {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Instruction::MoveLeft),
            '>' => Ok(Instruction::MoveRight),
            '+' => Ok(Instruction::Increment),
            '-' => Ok(Instruction::Decrement),
            '.' => Ok(Instruction::Write),
            ',' => Ok(Instruction::Read),
            '[' => Ok(Instruction::JumpLeft),
            ']' => Ok(Instruction::JumpRight),
            _ => Err(ParseError::UnexpectedCharacter),
        }
    }
}

#[derive(Debug)]
pub struct Program(pub Vec<Instruction>);

impl FromStr for Program {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .filter(Instruction::is_token)
            .map(Instruction::try_from)
            .collect::<Result<Vec<Instruction>, Self::Err>>()
            .map(Program)
    }
}

impl Program {
    pub fn run<T: Read, U: Write, const LEN: usize>(
        &self,
        data: &mut [u8; LEN],
        mut data_ptr: usize,
        mut input: T,
        output: &mut U,
    ) -> usize {
        let program = &self.0;
        let jump_addr: HashMap<usize, usize> = {
            let mut tmp = HashMap::new();
            let mut jump_stack: Vec<usize> = Vec::new();
            for (i, instr) in program.iter().enumerate() {
                match instr {
                    Instruction::JumpLeft => jump_stack.push(i),
                    Instruction::JumpRight => {
                        let matching = jump_stack.pop().unwrap();
                        tmp.insert(matching, i);
                        tmp.insert(i, matching);
                    }
                    _ => continue,
                }
            }
            tmp
        };

        debug!("Jump addresses: {jump_addr:#?}");

        let mut instr_ptr: usize = 0;
        loop {
            let instr = {
                if program.len() <= instr_ptr {
                    break;
                } else {
                    &program[instr_ptr]
                }
            };
            match instr {
                Instruction::MoveLeft => data_ptr -= 1,
                Instruction::MoveRight => data_ptr += 1,
                Instruction::Increment => data[data_ptr] += 1,
                Instruction::Decrement => data[data_ptr] -= 1,
                Instruction::Write => {
                    output.write(&[data[data_ptr]]).unwrap();
                }
                Instruction::Read => {
                    let mut buffer = [0];
                    input.read(&mut buffer).unwrap();
                    data[data_ptr] = buffer[0];
                }
                Instruction::JumpLeft => {
                    if data[data_ptr] == 0 {
                        instr_ptr = jump_addr[&instr_ptr];
                        continue;
                    }
                }
                Instruction::JumpRight => {
                    if data[data_ptr] != 0 {
                        instr_ptr = jump_addr[&instr_ptr];
                        continue;
                    }
                }
            }
            instr_ptr += 1;
        }
        data_ptr
    }
}
