use std::fs::File;
use std::io::{prelude::*, BufReader};
use utils::check_puzzle_conformity;
use anyhow::{ Result };
use npuzzle_error::NPuzzleError;
use std::error::Error;

#[path = "npuzzle_error/mod.rs"]
mod npuzzle_error;

#[path = "utils/mod.rs"]
mod utils;

#[derive(Debug)]
pub struct NPuzzle {
    size: usize,
    puzzle: Vec<Vec<u8>>
}

impl NPuzzle {
    pub fn new(args: &[String]) -> Result<NPuzzle, impl Error> {
        if args.len() != 2 {
            panic!("You shall provide only 1 argument");
        }
        let file = File::open(&args[1]).expect("Fail to open file");
        let reader = BufReader::new(file);
        let mut puzzle: Vec<Vec<u8>> = Vec::new();
        let mut size: usize = 0;

        for line in reader.lines() {
            match line {
                Ok(v) => {
                    if v.len() > 0 {

                        let numbers: Vec<u8> = v
                            .split_whitespace()
                            .filter_map(|w| w.parse().ok())
                            .collect();
                        if numbers.len() == 1 {
                            size = match usize::try_from(numbers[0]) {
                                Ok(nb) => nb,
                                Err(e) => return Err(e)
                            };
                        }
                        else if numbers.len() > 1 {
                            puzzle.push(numbers);
                        }
                    }
                },
                Err(e) => return Err(e)
            }
        }
        match check_puzzle_conformity(&puzzle, size) {
            Err(e) => { return Err(e) },
            _ => {},
        }
        return Some(Self {
            size: size,
            puzzle: puzzle
        })
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_puzzle(&self) -> Vec<Vec<u8>> {
        self.puzzle.clone()
    }
}