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
pub struct Program(Vec<Instruction>);

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

#[cfg(test)]
mod tests {
    use std::iter::repeat;

    use super::*;

    // 1. single instruction tests (when those can have reasonable expectations)
    // 2. round trip tests
    // 3. hello world
    // 4. moving outside of tape shold panic (two tests)
    // 5. incrementing and decrementing outside of bounds should panic

    fn init_data() -> [u8; 30_000] {
        [0u8; 30_000]
    }

    #[test]
    fn increment_works() {
        let program = Program(vec![Instruction::Increment]);
        let mut data = [254u8];
        program.run(&mut data, 0, [].as_slice(), &mut vec![]);
        assert_eq!(data, [255]);
    }

    #[test]
    #[should_panic]
    fn increment_over_u8_panics() {
        let program = Program(repeat(Instruction::Increment).take(256).collect());
        let mut data = [0u8];
        program.run(&mut data, 0, [].as_slice(), &mut vec![]);
    }

    #[test]
    fn decrement_works() {
        let program = Program(vec![Instruction::Decrement]);
        let mut data = [1u8];
        program.run(&mut data, 0, [].as_slice(), &mut vec![]);
        assert_eq!(data, [0u8]);
    }

    #[test]
    #[should_panic]
    fn decrement_under_u8_panics() {
        let program = Program(vec![Instruction::Decrement]);
        let mut data = [0u8];
        program.run(&mut data, 0, [].as_slice(), &mut vec![]);
    }

    #[test]
    fn increment_decrement_works() {
        let program = Program(vec![Instruction::Increment, Instruction::Decrement]);
        let mut data = [0u8];
        program.run(&mut data, 0, [].as_slice(), &mut vec![]);
        assert_eq!(data, [0u8]);
    }

    #[test]
    fn move_right_works() {
        let program = Program(vec![Instruction::MoveRight]);
        let data_ptr = program.run(&mut [0u8], 0, [].as_slice(), &mut vec![]);
        assert_eq!(data_ptr, 1);
    }

    #[test]
    #[should_panic]
    fn move_right_over_usize_panics() {
        let program = Program(vec![Instruction::MoveRight]);
        program.run(&mut [0u8], usize::MAX, [].as_slice(), &mut vec![]);
    }

    #[test]
    fn move_left_works() {
        let program = Program(vec![Instruction::MoveLeft]);
        let data_ptr = program.run(&mut [0u8], 1, [].as_slice(), &mut vec![]);
        assert_eq!(data_ptr, 0);
    }

    #[test]
    #[should_panic]
    fn move_left_under_usize_panics() {
        let program = Program(vec![Instruction::MoveLeft]);
        program.run(&mut [0u8], 0, [].as_slice(), &mut vec![]);
    }

    #[test]
    #[should_panic]
    fn increment_out_of_bounds_panics() {
        let program = Program(vec![Instruction::Increment]);
        program.run(&mut [0u8], 1, [].as_slice(), &mut vec![]);
    }

    #[test]
    fn write_works() {
        let program = Program(vec![Instruction::Write]);
        let mut data = init_data();
        let mut output = Vec::new();
        program.run(&mut data, 0, [].as_slice(), &mut output);
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn read_works() {
        let program = Program(vec![Instruction::Read]);
        let mut data = [0u8];
        program.run(&mut data, 0, [1u8].as_slice(), &mut vec![]);
        assert_eq!(data, [1u8]);
    }

    #[test]
    fn read_write_works() {
        let program = Program(vec![Instruction::Read, Instruction::Write]);
        let mut data = init_data();
        let input = [b'a'];
        let mut output = Vec::new();
        program.run(&mut data, 0, input.as_slice(), &mut output);
        assert_eq!(output, input);
    }

    #[test]
    fn read_write_longer_works() {
        let program = Program(
            [Instruction::Read, Instruction::Write]
                .into_iter()
                .cycle()
                .take(2 * 6)
                .collect(),
        );
        let mut data = init_data();
        let input = [b'a', b'b', b'c', b'1', b'2', b'3'];
        let mut output = Vec::new();
        program.run(&mut data, 0, input.as_slice(), &mut output);
        assert_eq!(output, input);
    }

    #[test]
    fn hello_world_works() {
        let program: Program = r"
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."
        .parse().unwrap();
        let mut data = init_data();
        let mut output = Vec::new();
        program.run(&mut data, 0, [].as_slice(), &mut output);
        assert_eq!(output, "Hello World!\n".as_bytes());
    }
}
