use anyhow::{Result, Ok};
use std::cmp::{Ord, Ordering};

#[derive(Debug, Clone)]
pub struct State<'a> {
    pub puzzle: Vec<u16>,
    pub distance: u32,
    pub parent: Option<&'a State<'a>>
}

impl<'a> State<'a> {
    pub fn new(puzzle: &'a Vec<u16>) -> Result<State> {
        Ok(Self {
            puzzle: puzzle.to_vec(),
            distance: 0,
            parent: None
        })
    }
}

impl<'a> Ord for State<'a> {
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

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for State<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.distance != other.distance {
            false
        } else {
            true
        }
    }
}

impl<'a> Eq for State<'a> { }