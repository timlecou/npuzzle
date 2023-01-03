use std::env;
use anyhow::{Result, bail};
use npuzzle::NPuzzle;
mod npuzzle;
use state::State;
#[path = "state/mod.rs"]
mod state;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("\nWrong number of arguments provided\n\ncargo run -- <filename>\n");
    }
    let puzzle: NPuzzle = NPuzzle::new(&args[1])?;

    println!("Current state:");
    puzzle.print_current_state();
    println!("\nGoal:");
    puzzle.print_goal();
    println!("\n");
    dbg!(puzzle.solve2()?.last().expect(""));
    //Hamming Distance/Misplaced Tiles
    Ok(())
}

// fn main() -> Result<()> {
//     let mut s1: State = State::new(&vec![1, 2, 3])?;
//     let mut s2: State = State::new(&vec![1, 2, 3])?;
//     let mut s3: State = State::new(&vec![3, 1, 2])?;
//     let mut v1: Vec<State> = Vec::new();

//     s1.distance = 5;
//     s2.distance = 3;
//     s3.distance = 5;

//     v1.push(s1.to_owned());
//     v1.push(s2.to_owned());
//     v1.push(s3.to_owned());

//     println!("{}", v1.contains(&s1));
//     println!("{}", v1.contains(&s2));
//     println!("{}", v1.contains(&s3));
//     Ok(())
// }