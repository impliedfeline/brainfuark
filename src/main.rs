use std::{collections::HashMap, env, fs, io};

#[derive(Debug)]
enum Instruction {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Print,
    Read,
    JumpLeft,
    JumpRight,
}

impl Instruction {
    fn is_token_char(c: &char) -> bool {
        let token_chars = ['<', '>', '+', '-', '.', ',', '[', ']'];
        token_chars.contains(&c)
    }
}

#[derive(Debug)]
enum ParseError {
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
            '.' => Ok(Instruction::Print),
            ',' => Ok(Instruction::Read),
            '[' => Ok(Instruction::JumpLeft),
            ']' => Ok(Instruction::JumpRight),
            _ => Err(ParseError::UnexpectedCharacter),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).unwrap();

    println!("Program file contents: {contents}");

    let program: Result<Vec<Instruction>, ParseError> = contents
        .chars()
        .filter(Instruction::is_token_char)
        .map(Instruction::try_from)
        .collect();
    let program: Vec<Instruction> = program.unwrap();

    println!("Parsed program: {program:#?}");

    let mut data: Vec<u8> = vec![0; 30_000];
    let mut data_ptr: usize = 0;
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
                _ => continue
            }
        }
        tmp
    };

    println!("Jump addresses: {jump_addr:#?}");

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
            Instruction::Print => print!("{}", data[data_ptr] as char),
            Instruction::Read => {
                let mut line = String::new();
                io::stdin().read_line(&mut line).unwrap();
                data[data_ptr] = line.bytes().next().unwrap();
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
}
