use std::env;
use anyhow::{Result, bail};
use npuzzle_solver::solver::{ NpuzzleSolver };

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        bail!("\nWrong number of arguments provided\n\ncargo run -- <filename>\n");
    }
    let solver: NpuzzleSolver = NpuzzleSolver::from_file(&args[0])?;
    // todo!("Random puzzle generation");
    // if !NpuzzleSolver::is_solvable(&solver.start, solver.size) {
    //     bail!("This puzzle is unsolvable:\n{}", solver.start);
    // }

    let results = solver.solve_astar(npuzzle_solver::heuristics::Heuristics::ManhattanDistance);
    println!("{}", results);
    // println!("size: {}\npuzzle:{}\ntarget:{}", solver.size, solver.start, solver.target);
    Ok(())
}