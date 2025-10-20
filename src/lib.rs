use std::io::{Read, Write};
use std::str::FromStr;

use thiserror::Error;

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Write,
    Read,
    JumpLeft(usize),
    JumpRight(usize),
}

impl Instruction {
    fn is_token(c: char) -> bool {
        let token_chars = ['<', '>', '+', '-', '.', ',', '[', ']'];
        token_chars.contains(&c)
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("encountered unexpected character during parse")]
    UnexpectedCharacter,
}

#[derive(Debug)]
pub struct Program(pub Vec<Instruction>);

impl FromStr for Program {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut prog = Vec::new();
        let mut jump_stack: Vec<usize> = Vec::new();
        for (i, chr) in s.chars().filter(|c| Instruction::is_token(*c)).enumerate() {
            let token = match chr {
                '<' => Instruction::MoveLeft,
                '>' => Instruction::MoveRight,
                '+' => Instruction::Increment,
                '-' => Instruction::Decrement,
                '.' => Instruction::Write,
                ',' => Instruction::Read,
                '[' => {
                    // TODO: Error handling for unbalanced parens
                    jump_stack.push(i);
                    Instruction::JumpLeft(usize::MAX)
                }
                ']' => {
                    // TODO: Error handling for unbalanced parens
                    let jump_addr = jump_stack.pop().unwrap();
                    if let Instruction::JumpLeft(_) = prog[jump_addr] {
                        prog[jump_addr] = Instruction::JumpLeft(i)
                    }
                    Instruction::JumpRight(jump_addr)
                }
                _ => return Err(Self::Err::UnexpectedCharacter),
            };
            prog.push(token);
        }
        Ok(Self(prog))
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
                    output.write_all(&[data[data_ptr]]).unwrap();
                }
                Instruction::Read => {
                    let mut buffer = [0];
                    input.read_exact(&mut buffer).unwrap();
                    data[data_ptr] = buffer[0];
                }
                Instruction::JumpLeft(jump_addr) => {
                    if data[data_ptr] == 0 {
                        instr_ptr = *jump_addr;
                        continue;
                    }
                }
                Instruction::JumpRight(jump_addr) => {
                    if data[data_ptr] != 0 {
                        instr_ptr = *jump_addr;
                        continue;
                    }
                }
            }
            instr_ptr += 1;
        }
        data_ptr
    }
}
