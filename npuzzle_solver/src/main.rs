use anyhow::{Result, bail};
use npuzzle_solver::solver::{ NpuzzleSolver };
use npuzzle_solver::heuristics::Heuristics;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name="npuzzle-solver")]
pub struct Args {

    #[structopt(short, long, default_value="manhattan")]
    pub heuristic: Heuristics,

    #[structopt(short, long)]
    pub filename: Option<String>,

    #[structopt(short, long)]
    pub random_size: Option<usize>
}

fn main() -> Result<()> {
    let args = Args::from_args();
    let solver = if let Some(random) = args.random_size {
        NpuzzleSolver::generate_random(random)?
    } else if let Some(filename) = args.filename {
        NpuzzleSolver::from_file(&filename)?
    } else {
        bail!("You must provide either a filename or a number to generate a random puzzle")
    };
    // if !NpuzzleSolver::is_solvable(&solver.start, solver.size) {
    //     bail!("This puzzle is unsolvable:\n{}", solver.start);
    // } else {
    //     println!("Solving . . .");
    // }

    println!("{}", solver.solve_astar(args.heuristic));
    Ok(())
}