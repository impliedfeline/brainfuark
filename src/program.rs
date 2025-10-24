use std::{
    io::{Read, Write},
    str::FromStr,
};

use thiserror::Error;

use crate::state::ProgramState;

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
    pub fn is_token(c: char) -> bool {
        let token_chars = ['<', '>', '+', '-', '.', ',', '[', ']'];
        token_chars.contains(&c)
    }

    /// Runs a single instruction against a given `ProgramState`.
    ///
    /// # Panics
    ///
    /// Currently, the program panics in various different illegal states for a program state.
    /// In a later commit this function will be made panic-free, but that is still to do.
    pub fn step<I: Read, O: Write, const LEN: usize>(
        self,
        state: &mut ProgramState<LEN>,
        input: &mut I,
        output: &mut O,
    ) -> Option<usize> {
        match self {
            Instruction::MoveLeft => state.data_ptr -= 1,
            Instruction::MoveRight => state.data_ptr += 1,
            Instruction::Increment => state.data[state.data_ptr] += 1,
            Instruction::Decrement => state.data[state.data_ptr] -= 1,
            Instruction::Write => {
                output.write_all(&[state.data[state.data_ptr]]).unwrap();
            }
            Instruction::Read => {
                let mut buffer = [0];
                input.read_exact(&mut buffer).unwrap();
                state.data[state.data_ptr] = buffer[0];
            }
            Instruction::JumpLeft(jump_addr) => {
                if state.data[state.data_ptr] == 0 {
                    return Some(jump_addr);
                }
            }
            Instruction::JumpRight(jump_addr) => {
                if state.data[state.data_ptr] != 0 {
                    return Some(jump_addr);
                }
            }
        }
        None
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("encountered unexpected character during parse")]
    UnexpectedCharacter,
}

#[derive(Debug)]
pub struct Program(pub Vec<Instruction>);

impl Program {
    pub fn run<I: Read, O: Write, const LEN: usize>(
        &self,
        mut instr_ptr: usize,
        state: &mut ProgramState<LEN>,
        input: &mut I,
        output: &mut O,
    ) {
        while let Some(instr) = self.0.get(instr_ptr) {
            if let Some(jump_addr) = instr.step(state, input, output) {
                instr_ptr = jump_addr;
            }
        }
    }
}

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
                        prog[jump_addr] = Instruction::JumpLeft(i);
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
