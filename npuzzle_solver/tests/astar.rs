use anyhow::{Result};

use npuzzle_solver::solver::{NpuzzleSolver, Board};

/**
 * 
 * get_possible_moves
 * 
 */

#[test]
fn test_get_possible_moves_1() -> Result<()> {
    let solver: NpuzzleSolver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-3-1.txt"))?;
    assert_eq!(NpuzzleSolver::get_possible_moves(
        &solver.size,
        solver.start
        .inner()
        .iter()
        .position(|nb| nb == &0)
        .unwrap()),
        [Some(4), Some(2), None, Some(8)]);
    Ok(())
}

#[test]
fn test_get_possible_moves_2() -> Result<()> {
    let solver: NpuzzleSolver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-4-1.txt"))?;
    assert_eq!(NpuzzleSolver::get_possible_moves(
        &solver.size,
        solver.start
        .inner()
        .iter()
        .position(|nb| nb == &0)
        .unwrap()),
        [Some(2), None, None, Some(7)]);
    Ok(())
}

#[test]
fn test_get_possible_moves_3() -> Result<()> {
    let solver: NpuzzleSolver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-3-3.txt"))?;
    assert_eq!(NpuzzleSolver::get_possible_moves(
        &solver.size,
        solver.start
        .inner()
        .iter()
        .position(|nb| nb == &0)
        .unwrap()),
        [Some(3), Some(1), Some(5), Some(7)]);
    Ok(())
}

#[test]
fn test_get_possible_moves_4() -> Result<()> {
    let solver: NpuzzleSolver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-5-0.txt"))?;
    assert_eq!(NpuzzleSolver::get_possible_moves(
        &solver.size,
        solver.start
        .inner()
        .iter()
        .position(|nb| nb == &0)
        .unwrap()),
        [Some(15), Some(11), Some(17), Some(21)]);
    Ok(())
}

/**
 * 
 * get_successors
 * 
 */

#[test]
fn test_get_successors_1() -> Result<()> {
    let solver: NpuzzleSolver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-3-1.txt"))?;
    assert_eq!(NpuzzleSolver::get_successors(&solver.start, solver.size), vec![
        Board(vec![3, 2, 6, 1, 0, 4, 8, 7, 5]),
        Board(vec![3, 2, 0, 1, 4, 6, 8, 7, 5]),
        Board(vec![3, 2, 6, 1, 4, 5, 8, 7, 0])]);
    Ok(())
}

#[test]
fn test_get_successors_2() -> Result<()> {
    let solver: NpuzzleSolver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-4-1.txt"))?;
    assert_eq!(NpuzzleSolver::get_successors(&solver.start, solver.size), vec![
        Board(vec![3, 2, 0, 4, 1, 8, 6, 5, 7, 9, 10, 11, 13, 14, 12, 15]),
        Board(vec![3, 2, 4, 5, 1, 8, 6, 0, 7, 9, 10, 11, 13, 14, 12, 15])]);
    Ok(())
}

#[test]
fn test_get_successors_3() -> Result<()> {
    let solver: NpuzzleSolver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-5-0.txt"))?;
    assert_eq!(NpuzzleSolver::get_successors(&solver.start, solver.size), vec![
        Board(vec![8, 9, 16, 11, 21, 10, 6, 18, 2, 5, 13, 4, 1, 3, 15, 0, 7, 23, 22, 24, 19, 14, 17, 12, 20]),
        Board(vec![8, 9, 16, 11, 21, 10, 6, 18, 2, 5, 13, 0, 1, 3, 15, 7, 4, 23, 22, 24, 19, 14, 17, 12, 20]),
        Board(vec![8, 9, 16, 11, 21, 10, 6, 18, 2, 5, 13, 4, 1, 3, 15, 7, 23, 0, 22, 24, 19, 14, 17, 12, 20]),
        Board(vec![8, 9, 16, 11, 21, 10, 6, 18, 2, 5, 13, 4, 1, 3, 15, 7, 14, 23, 22, 24, 19, 0, 17, 12, 20])]);
    Ok(())
}