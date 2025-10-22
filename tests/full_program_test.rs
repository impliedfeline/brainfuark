#![allow(clippy::pedantic)]
use brainfuark::*;

mod common;

#[test]
fn hello_world_works() {
    let program: Program = r"
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."
    .parse().unwrap();
    let mut state = ProgramState {
        data: [0u8; 30_000],
        ..ProgramState::default()
    };
    let mut output = Vec::new();
    state.run(&program, &mut [].as_ref(), &mut output);
    assert_eq!(output, "Hello World!\n".as_bytes());
}
