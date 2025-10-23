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
