use std::env;
use anyhow::{Result, bail};
use npuzzle::NPuzzle;
mod npuzzle;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let puzzle: NPuzzle =  match NPuzzle::new(&args) {
        Some(v) => v,
        None => { bail!("fail to build puzzle") }
    };

    // println!("{}", puzzle.size);
    dbg!(puzzle);
    Ok(())
}