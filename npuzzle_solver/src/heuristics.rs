
use std::fmt::Display;
use crate::solver::{ Board, NpuzzleSolver };



pub enum Heuristics {
    ManhattanDistance,
    MisplacedTiles,
    EuclidianDistance
}

impl Display for Heuristics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Heuristics::ManhattanDistance => "manhattan distance",
            Heuristics::MisplacedTiles => "misplaced tiles",
            Heuristics::EuclidianDistance => "euclidian distance squared"
        })
    }
}

pub trait Heuristic {
    fn run_heuristic(&self, puzzle: &Board, target: &Board, size: usize) -> usize;
}

impl Heuristic for Heuristics {
    fn run_heuristic(&self, puzzle: &Board, target: &Board, size: usize) -> usize {
        match *self {
            Heuristics::ManhattanDistance => ManhattanDistanceHeuristic.run_heuristic(puzzle, target, size),
            Heuristics::MisplacedTiles => MisplacedTilesHeuristic.run_heuristic(puzzle, target, size),
            Heuristics::EuclidianDistance => EuclidianDistanceHeuristic.run_heuristic(puzzle, target, size)
        }
    }
}


struct ManhattanDistanceHeuristic;
impl Heuristic for ManhattanDistanceHeuristic {
    fn run_heuristic(&self, puzzle: &Board, target: &Board, size: usize) -> usize {
        let mut distance: usize = 0;
        for (idx, nb) in puzzle.inner().iter().enumerate() {
            let target_idx = target.inner().iter().position(|&n| n == *nb).unwrap();
            let (puzzle_x, puzzle_y) = (idx % size, idx / size);
            let (target_x, target_y) = (target_idx % size, target_idx / size);
            distance += ((target_y as isize - puzzle_y as isize).abs() + (target_x as isize - puzzle_x as isize).abs()) as usize;
        }
        distance
    }
}

struct MisplacedTilesHeuristic;
impl Heuristic for MisplacedTilesHeuristic {
    fn run_heuristic(&self, puzzle: &Board, target: &Board, _size: usize) -> usize {
        puzzle
        .inner()
        .iter()
        .zip(target.inner())
        .filter(|(puzzle, target)| puzzle != target)
        .count()
    }
}

struct EuclidianDistanceHeuristic;
impl Heuristic for EuclidianDistanceHeuristic {
    fn run_heuristic(&self, puzzle: &Board, target: &Board, size: usize) -> usize {
        let mut distance: usize = 0;
        for (idx, nb) in puzzle.inner().iter().enumerate() {
            let target_idx = target.inner().iter().position(|&n| n == *nb).unwrap();
            let (puzzle_x, puzzle_y) = (idx % size, idx / size);
            let (target_x, target_y) = (target_idx % size, target_idx / size);
            distance += (((target_y as isize - puzzle_y as isize).pow(2) + (target_x as isize - puzzle_x as isize).pow(2)) as f32).sqrt() as usize;
        }
        distance
    }
}