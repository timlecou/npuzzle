// use anyhow::{ Result };
use npuzzle_error::NPuzzleError;
use std::error::Error;

#[path = "../npuzzle_error/mod.rs"]
mod npuzzle_error;

pub fn check_puzzle_conformity(puzzle: &Vec<Vec<u8>>, size: usize) -> Result<(), NPuzzleError> {
    let mut big_vec: Vec<u8> = Vec::new();
    let mut i: usize = 0;
    let mut j: u8 = 0;
    
    if size != puzzle.len() {
        return Err(NPuzzleError::BadSizeInput);
    }
    for v in puzzle.clone() {
        if size != v.len() {
            return Err(NPuzzleError::BadSizeInput);
        }
        big_vec.extend(v);
    }

    if size != big_vec.len() {
        return Err(NPuzzleError::BadSizeInput);
    }

    while i < big_vec.len() {
        let mut tmp: u8 = 0;
        if big_vec.contains(&j) == false {
            return Err(NPuzzleError::BadSizeInput);
        }
        i += 1;
        j += 1;
    }
    Ok(())
}