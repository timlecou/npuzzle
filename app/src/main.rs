use std::env;
use anyhow::{Result, bail};
use npuzzle::NPuzzle;
mod npuzzle;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("\nWrong number of arguments provided\n\ncargo run -- <filename>\n");
    }
    let mut puzzle: NPuzzle = NPuzzle::new(&args[1])?;

    println!("Current state:");
    puzzle.print_current_state();
    println!("\nGoal:");
    puzzle.print_goal();
    println!("\n");
    puzzle.solve();
    Ok(())
}