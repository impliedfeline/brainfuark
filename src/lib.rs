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

#[derive(Debug)]
pub struct ProgramState<const LEN: usize> {
    pub data: [u8; LEN],
    pub data_ptr: usize,
    pub instr_ptr: usize,
}

impl<const LEN: usize> Default for ProgramState<LEN> {
    fn default() -> Self {
        Self {
            data: [0u8; LEN],
            data_ptr: Default::default(),
            instr_ptr: Default::default(),
        }
    }
}

impl<const LEN: usize> ProgramState<LEN> {
    pub fn run<T: Read, U: Write>(&mut self, prog: &Program, input: &mut T, output: &mut U) {
        while let Some(instr) = prog.0.get(self.instr_ptr) {
            self.step(*instr, input, output);
        }
    }

    /// Runs a single instruction against a given `ProgramState`.
    ///
    /// # Panics
    ///
    /// Currently, the program panics in various different illegal states for a program state.
    /// In a later commit this function will be made panic-free, but that is still to do.
    pub fn step<T: Read, U: Write>(&mut self, instr: Instruction, input: &mut T, output: &mut U) {
        match instr {
            Instruction::MoveLeft => self.data_ptr -= 1,
            Instruction::MoveRight => self.data_ptr += 1,
            Instruction::Increment => self.data[self.data_ptr] += 1,
            Instruction::Decrement => self.data[self.data_ptr] -= 1,
            Instruction::Write => {
                output.write_all(&[self.data[self.data_ptr]]).unwrap();
            }
            Instruction::Read => {
                let mut buffer = [0];
                input.read_exact(&mut buffer).unwrap();
                self.data[self.data_ptr] = buffer[0];
            }
            Instruction::JumpLeft(jump_addr) => {
                if self.data[self.data_ptr] == 0 {
                    self.instr_ptr = jump_addr;
                    return;
                }
            }
            Instruction::JumpRight(jump_addr) => {
                if self.data[self.data_ptr] != 0 {
                    self.instr_ptr = jump_addr;
                    return;
                }
            }
        }
        self.instr_ptr += 1;
    }
}
