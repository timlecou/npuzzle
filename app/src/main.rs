use std::env;
use anyhow::{Result, bail};
use npuzzle::NPuzzle;
mod npuzzle;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let puzzle: NPuzzle =  match NPuzzle::new(&args) {
        Ok(v) => v,
        Err(error) => { bail!("Fail to build puzzle: {}", error) }
    };

    // println!("{}", puzzle.size);
    dbg!(puzzle);
    Ok(())
}