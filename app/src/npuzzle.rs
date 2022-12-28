use std::fs::File;
use std::io::{prelude::*, BufReader};
use utils::check_puzzle_conformity;
// use anyhow::{ Result };
use std::error::Error;
use npuzzle_error::NPuzzleError;

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
    pub fn new(args: &[String]) -> Result<NPuzzle, Box<dyn std::error::Error>> {
        if args.len() != 2 {
            panic!("You shall provide only 1 argument");
        }
        let file = File::open(&args[1])?;
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
                            size = numbers[0] as usize;
                        }
                        else if numbers.len() > 1 {
                            puzzle.push(numbers);
                        }
                    }
                },
                Err(e) => return Err(Box::new(e))
            }
        }
        check_puzzle_conformity(&puzzle, size)?;
        Ok(Self {
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