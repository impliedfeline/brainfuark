use brainfuark::*;

mod common;

#[test]
fn hello_world_works() {
    let program: Program = r"
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."
    .parse().unwrap();
    let mut data = common::init_data();
    let mut output = Vec::new();
    program.run(&mut data, 0, [].as_slice(), &mut output);
    assert_eq!(output, "Hello World!\n".as_bytes());
}
