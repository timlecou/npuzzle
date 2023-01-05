use anyhow::{Result};

use npuzzle_solver::solver::NpuzzleSolver;

#[test]
fn test_count_inversions_three_sized_puzzle_1() -> Result<()> {
    let solver = NpuzzleSolver::from_file(&String::from("tests/puzzles/3_inversions-3-0.txt"))?;
    assert_eq!(NpuzzleSolver::count_inversions(&solver.start), 3);
    Ok(())
}

#[test]
fn test_count_inversions_three_sized_puzzle_2() -> Result<()> {
    let solver = NpuzzleSolver::from_file(&String::from("tests/puzzles/4_inversions-3-0.txt"))?;
    assert_eq!(NpuzzleSolver::count_inversions(&solver.start), 4);
    Ok(())
}

#[test]
fn test_count_inversions_four_sized_puzzle_1() -> Result<()> {
    let solver = NpuzzleSolver::from_file(&String::from("tests/puzzles/6_inversions-4-0.txt"))?;
    assert_eq!(NpuzzleSolver::count_inversions(&solver.start), 6);
    Ok(())
}

#[test]
fn test_is_solvable_with_three_sized_unsolvable() -> Result<()> {
    let solver = NpuzzleSolver::from_file(&String::from("tests/puzzles/unsolvable-3-0.txt"))?;
    assert_eq!(NpuzzleSolver::is_solvable(&solver.start, 3), false);
    Ok(())
}

#[test]
fn test_is_solvable_with_three_sized_solvable() -> Result<()> {
    let solver = NpuzzleSolver::from_file(&String::from("tests/puzzles/solvable-3-0.txt"))?;
    assert_eq!(NpuzzleSolver::is_solvable(&solver.start, 3), true);
    Ok(())
}

#[test]
fn test_is_solvable_with_four_sized_unsolvable() -> Result<()> {
    let solver = NpuzzleSolver::from_file(&String::from("tests/puzzles/unsolvable-4-0.txt"))?;
    assert_eq!(NpuzzleSolver::is_solvable(&solver.start, 4), false);
    Ok(())
}

#[test]
fn test_is_solvable_with_four_sized_solvable() -> Result<()> {
    let solver = NpuzzleSolver::from_file(&String::from("tests/puzzles/solvable-4-0.txt"))?;
    assert_eq!(NpuzzleSolver::is_solvable(&solver.start, 4), true);
    Ok(())
}