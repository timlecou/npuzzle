use npuzzle::NPuzzle;
use anyhow::{Result};

#[path = "../src/npuzzle.rs"]
mod npuzzle;

#[test]
fn three_raws_file() -> Result<()> {
    let puz: NPuzzle = NPuzzle::new(&String::from("tests/puzzles/npuzzle-3-1.txt"))?;
    assert_eq!(puz._get_size(), 3);
    assert_eq!(puz._get_puzzle().len(), 9);
    assert_eq!(vec![3, 2, 6, 1, 4, 0, 8, 7, 5], puz._get_puzzle());
    Ok(())
}


#[test]
fn three_raws_file_with_comments() -> Result<()> {
    let puz: NPuzzle = NPuzzle::new(&String::from("tests/puzzles/npuzzle-3-2.txt"))?;
    assert_eq!(puz._get_size(), 3);
    assert_eq!(puz._get_puzzle().len(), 9);
    assert_eq!(vec![3, 2, 6, 1, 4, 0, 8, 7, 5], puz._get_puzzle());
    Ok(())
}