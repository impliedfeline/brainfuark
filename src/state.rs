use std::io::{Read, Write};

use crate::program::{Instruction, Program};

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
