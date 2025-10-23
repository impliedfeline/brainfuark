use std::error::Error;

use anyhow::anyhow;
use brainfuark::{optimized::OProgram, program::Program, state::ProgramState};
use log::{debug, error, info};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        error!("No command-line arguments provided");
        Err(anyhow!("No command-line arguments provided, aborting"))?;
    }

    let path = args.first().unwrap();
    info!("Reading file: {path}");

    let contents = std::fs::read_to_string(path)?;
    debug!("Program file contents: {contents}");

    let program: Program = contents.parse()?;
    debug!("Parsed program: {program:#?}");

    let mut state: ProgramState<30_000> = ProgramState::default();
    state.run(&program, &mut std::io::stdin(), &mut std::io::stdout());
    debug!("hello?");

    let oprogram: OProgram = OProgram::parse(&mut program.0.into_iter().peekable()).improve();
    debug!("Optimized program: {oprogram:#?}");

    let mut state: ProgramState<30_000> = ProgramState::default();
    oprogram.run(&mut state, &mut std::io::stdin(), &mut std::io::stdout());

    Ok(())
}
