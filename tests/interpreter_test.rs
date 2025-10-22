#![allow(clippy::pedantic)]
use brainfuark::*;
use std::io::empty;

mod common;

#[test]
fn increment_works() {
    let mut state = ProgramState {
        data: [254u8],
        ..ProgramState::default()
    };
    state.step(Instruction::Increment, &mut empty(), &mut vec![]);
    assert_eq!(state.data, [255]);
}

#[test]
#[should_panic]
fn increment_over_u8_panics() {
    let mut state = ProgramState {
        data: [255u8],
        ..ProgramState::default()
    };
    state.step(Instruction::Increment, &mut empty(), &mut vec![]);
}

#[test]
fn decrement_works() {
    let mut state = ProgramState {
        data: [1u8],
        ..ProgramState::default()
    };
    state.step(Instruction::Decrement, &mut empty(), &mut vec![]);
    assert_eq!(state.data, [0u8]);
}

#[test]
#[should_panic]
fn decrement_under_u8_panics() {
    let mut state = ProgramState {
        data: [0u8],
        ..ProgramState::default()
    };
    state.step(Instruction::Decrement, &mut empty(), &mut vec![]);
}

#[test]
fn increment_decrement_works() {
    let program = Program(vec![Instruction::Increment, Instruction::Decrement]);
    let mut state = ProgramState {
        data: [0u8],
        ..ProgramState::default()
    };
    state.run(&program, &mut empty(), &mut vec![]);
    assert_eq!(state.data, [0u8]);
}

#[test]
fn move_right_works() {
    let mut state: ProgramState<0> = ProgramState::default();
    state.step(Instruction::MoveRight, &mut empty(), &mut vec![]);
    assert_eq!(state.data_ptr, 1);
}

#[test]
#[should_panic]
fn move_right_over_usize_panics() {
    let mut state: ProgramState<0> = ProgramState {
        data_ptr: usize::MAX,
        ..ProgramState::default()
    };
    state.step(Instruction::MoveRight, &mut empty(), &mut vec![]);
}

#[test]
fn move_left_works() {
    let mut state: ProgramState<0> = ProgramState {
        data_ptr: 1,
        ..ProgramState::default()
    };
    state.step(Instruction::MoveLeft, &mut empty(), &mut vec![]);
    assert_eq!(state.data_ptr, 0);
}

#[test]
#[should_panic]
fn move_left_under_usize_panics() {
    let mut state: ProgramState<0> = ProgramState::default();
    state.step(Instruction::MoveLeft, &mut empty(), &mut vec![]);
}

#[test]
#[should_panic]
fn increment_out_of_bounds_panics() {
    let mut state: ProgramState<0> = ProgramState {
        data_ptr: 1,
        ..ProgramState::default()
    };
    state.step(Instruction::Increment, &mut empty(), &mut vec![]);
}

#[test]
fn write_works() {
    let mut state: ProgramState<1> = ProgramState::default();
    let mut output = Vec::new();
    state.step(Instruction::Write, &mut empty(), &mut output);
    assert_eq!(output, vec![0]);
}

#[test]
fn read_works() {
    let mut state: ProgramState<1> = ProgramState::default();
    let input = [1u8];
    state.step(Instruction::Read, &mut input.as_ref(), &mut vec![]);
    assert_eq!(state.data, [1u8]);
}

#[test]
fn read_write_works() {
    let program = Program(vec![Instruction::Read, Instruction::Write]);
    let mut state: ProgramState<1> = ProgramState::default();
    let input = [b'a'];
    let mut output = Vec::new();
    state.run(&program, &mut input.as_ref(), &mut output);
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
    let mut state: ProgramState<6> = ProgramState::default();
    let input = [b'a', b'b', b'c', b'1', b'2', b'3'];
    let mut output = Vec::new();
    state.run(&program, &mut input.as_ref(), &mut output);
    assert_eq!(output, input);
}
