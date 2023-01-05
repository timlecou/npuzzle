use anyhow::{Result};

use npuzzle_solver::solver::NpuzzleSolver;
use npuzzle_solver::heuristics::{ Heuristics, Heuristic };

/**
 * 
 * Manhattan distance
 * 
 */

#[test]
fn test_manhattan_distance_heuristic_three_sized_puzzle() -> Result<()> {
    let solver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-3-1.txt"))?;
    let distance = Heuristics::ManhattanDistance.run_heuristic(&solver.start, &solver.target, solver.size);
    assert_eq!(distance, 10);
    Ok(())
}

#[test]
fn test_manhattan_distance_heuristic_three_sized_puzzle_2() -> Result<()> {
    let solver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-3-3.txt"))?;
    let distance = Heuristics::ManhattanDistance.run_heuristic(&solver.start, &solver.target, solver.size);
    assert_eq!(distance, 18);
    Ok(())
}

#[test]
fn test_manhattan_distance_heuristic_five_sized_puzzle_1() -> Result<()> {
    let solver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-5-0.txt"))?;
    let distance = Heuristics::ManhattanDistance.run_heuristic(&solver.start, &solver.target, solver.size);
    assert_eq!(distance, 76);
    Ok(())
}

#[test]
fn test_manhattan_distance_heuristic_two_sized_puzzle_1() -> Result<()> {
    let solver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-2-0.txt"))?;
    let distance = Heuristics::ManhattanDistance.run_heuristic(&solver.start, &solver.target, solver.size);
    assert_eq!(distance, 6);
    Ok(())
}


/**
 * 
 * Misplaced tiles
 * 
 */

 #[test]
fn test_misplaced_tiles_heuristic_three_sized_puzzle_1() -> Result<()> {
    let solver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-3-1.txt"))?;
    let distance = Heuristics::MisplacedTiles.run_heuristic(&solver.start, &solver.target, solver.size);
    assert_eq!(distance, 8);
    Ok(())
}

#[test]
fn test_misplaced_tiles_heuristic_three_sized_puzzle_2() -> Result<()> {
    let solver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-3-3.txt"))?;
    let distance = Heuristics::MisplacedTiles.run_heuristic(&solver.start, &solver.target, solver.size);
    assert_eq!(distance, 9);
    Ok(())
}

#[test]
fn test_misplaced_tiles_heuristic_five_sized_puzzle_1() -> Result<()> {
    let solver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-5-0.txt"))?;
    let distance = Heuristics::MisplacedTiles.run_heuristic(&solver.start, &solver.target, solver.size);
    assert_eq!(distance, 24);
    Ok(())
}

#[test]
fn test_misplaced_tiles_heuristic_two_sized_puzzle_1() -> Result<()> {
    let solver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-2-0.txt"))?;
    let distance = Heuristics::MisplacedTiles.run_heuristic(&solver.start, &solver.target, solver.size);
    assert_eq!(distance, 4);
    Ok(())
}