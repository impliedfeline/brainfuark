use brainfuark::*;
use std::iter::repeat_n;

mod common;

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
    let program = Program(repeat_n(Instruction::Increment, 256).collect());
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
    let mut data = common::init_data();
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
    let mut data = common::init_data();
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
    let mut data = common::init_data();
    let input = [b'a', b'b', b'c', b'1', b'2', b'3'];
    let mut output = Vec::new();
    program.run(&mut data, 0, input.as_slice(), &mut output);
    assert_eq!(output, input);
}
