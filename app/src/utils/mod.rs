use npuzzle_error::NPuzzleError;
// use std::error::Error;

#[path = "../npuzzle_error/mod.rs"]
mod npuzzle_error;

pub fn check_puzzle_conformity(puzzle: &Vec<Vec<u16>>, size: usize) -> Result<(), NPuzzleError> {
    let mut big_vec: Vec<u16> = Vec::new();
    let mut i: usize = 0;
    let mut j: u16 = 0;
    
    if size != puzzle.len() {
        return Err(NPuzzleError::WrongPuzzleHeight);
    }
    for v in puzzle.clone() {
        if size != v.len() {
            println!("size {}", size);
            println!("v.len() {}", v.len());
            return Err(NPuzzleError::WrongPuzzleWidth);
        }
        big_vec.extend(v);
    }

    if size * size != big_vec.len() {
        return Err(NPuzzleError::WrongPuzzleSize);
    }

    while i < big_vec.len() {
        if big_vec.contains(&j) == false {
            return Err(NPuzzleError::InvalidNumberInPuzzle);
        }
        i += 1;
        j += 1;
    }
    Ok(())
}