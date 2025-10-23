use std::collections::HashMap;
use std::io::{Read, Write};
use std::iter::Peekable;

use crate::program::Instruction;
use crate::state::ProgramState;

#[derive(Debug)]
pub struct OProgram(pub Vec<Block>);

impl OProgram {
    pub fn parse_loop<T: Iterator<Item = Instruction>>(instrs: &mut Peekable<T>) -> Option<Self> {
        if let Some(Instruction::JumpLeft(_)) = instrs.peek() {
            instrs.next();
            let p = Self::parse(instrs);
            if let Some(Instruction::JumpRight(_)) = instrs.peek() {
                instrs.next();
                Some(p)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn parse<T: Iterator<Item = Instruction>>(instrs: &mut Peekable<T>) -> Self {
        let mut blocks = Vec::new();
        while let Some(block) = Block::parse(instrs) {
            blocks.push(block);
        }
        Self(blocks)
    }

    /// Runs an optimized program
    ///
    /// # Panics
    ///
    /// Currently, the program panics in various different illegal states.
    /// In a later commit this function will be made panic-free, but that is still to do.
    pub fn run<I: Read, O: Write, const LEN: usize>(
        &self,
        state: &mut ProgramState<LEN>,
        input: &mut I,
        output: &mut O,
    ) {
        for block in &self.0 {
            match block {
                Block::DataOps(op_args) => {
                    for (offset, torsion) in &op_args.ops {
                        let location = &mut state.data[state.data_ptr.strict_add_signed(*offset)];
                        if torsion.is_negative() {
                            *location -= u8::try_from(torsion.unsigned_abs()).unwrap();
                        } else {
                            *location += u8::try_from(torsion.unsigned_abs()).unwrap();
                        }
                    }
                    state.data_ptr = state.data_ptr.strict_add_signed(op_args.distance);
                }
                Block::Loop(oprogram) => {
                    if state.data[state.data_ptr] == 0 {
                        continue;
                    }
                    loop {
                        oprogram.run(state, input, output);
                        if state.data[state.data_ptr] == 0 {
                            break;
                        }
                    }
                }
                Block::Io(io_instruction) => match io_instruction {
                    IoInstruction::Write => {
                        output.write_all(&[state.data[state.data_ptr]]).unwrap();
                    }
                    IoInstruction::Read => {
                        let mut buffer = [0];
                        input.read_exact(&mut buffer).unwrap();
                        state.data[state.data_ptr] = buffer[0];
                    }
                },
            }
        }
    }

    #[must_use]
    pub fn improve(self) -> Self {
        let mut improved = Vec::new();
        let mut prev_was_loop = true;
        for block in self.0 {
            match block {
                Block::DataOps(op_args) => {
                    if !op_args.ops.is_empty() || op_args.distance != 0 {
                        improved.push(Block::DataOps(op_args));
                        prev_was_loop = false;
                    }
                }
                Block::Loop(oprogram) => {
                    if !prev_was_loop {
                        improved.push(Block::Loop(oprogram.improve()));
                        prev_was_loop = true;
                    }
                }
                Block::Io(io_instruction) => {
                    improved.push(Block::Io(io_instruction));
                    prev_was_loop = false;
                }
            }
        }
        Self(improved)
    }
}

#[derive(Debug)]
pub struct OpArgs {
    pub ops: HashMap<isize, i16>,
    pub distance: isize,
}

impl OpArgs {
    pub fn parse<T: Iterator<Item = Instruction>>(instrs: &mut Peekable<T>) -> Option<Self> {
        let mut ops = HashMap::new();
        let mut distance = 0;
        let mut matched = false;
        while let Some(instr) = instrs.peek() {
            match instr {
                Instruction::MoveLeft => distance -= 1,
                Instruction::MoveRight => distance += 1,
                Instruction::Increment => {
                    ops.entry(distance).and_modify(|e| *e += 1).or_insert(1);
                }
                Instruction::Decrement => {
                    ops.entry(distance).and_modify(|e| *e -= 1).or_insert(-1);
                }
                _ => break,
            }
            matched = true;
            instrs.next();
        }
        if matched {
            ops.retain(|_, v| *v != 0);
            Some(Self { ops, distance })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum Block {
    DataOps(OpArgs),
    Loop(OProgram),
    Io(IoInstruction),
}

impl Block {
    pub fn parse<T: Iterator<Item = Instruction>>(instrs: &mut Peekable<T>) -> Option<Self> {
        OpArgs::parse(instrs)
            .map(Self::DataOps)
            .or_else(|| IoInstruction::parse(instrs).map(Self::Io))
            .or_else(|| OProgram::parse_loop(instrs).map(Self::Loop))
    }
}

#[derive(Debug)]
pub enum IoInstruction {
    Write,
    Read,
}

impl IoInstruction {
    pub fn parse<T: Iterator<Item = Instruction>>(instrs: &mut Peekable<T>) -> Option<Self> {
        if let Some(Instruction::Write | Instruction::Read) = instrs.peek() {
            instrs.next().map(|instr| match instr {
                Instruction::Write => Self::Write,
                Instruction::Read => Self::Read,
                _ => unreachable!(),
            })
        } else {
            None
        }
    }
}
