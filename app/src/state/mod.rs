use anyhow::{Result, Ok};
use std::cmp::{Ord, Ordering};

#[derive(Debug)]
pub struct State {
    pub size: usize,
    pub number_amount: usize,
    pub puzzle: Vec<u16>,
    pub distance: u32
}

impl State {
    pub fn new(size: usize, puzzle: &Vec<u16>) -> Result<State> {
        Ok(Self {
            size: size,
            number_amount: size * size,
            puzzle: puzzle.to_vec(),
            distance: 0
        })
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance < other.distance {
            Ordering::Less
        } else if self.distance > other.distance {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        if self.distance != other.distance {
            false
        } else {
            true
        }
    }
}

impl Eq for State { }