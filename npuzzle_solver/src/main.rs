use std::env;
use anyhow::{Result, bail};
use npuzzle_solver::solver::NpuzzleSolver;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        bail!("\nWrong number of arguments provided\n\ncargo run -- <filename>\n");
    }
    let solver: NpuzzleSolver = NpuzzleSolver::from_file(&args[0])?;

    println!("size: {}\npuzzle:{}\ntarget:{}", solver.size, solver.start, solver.target);
    Ok(())
}