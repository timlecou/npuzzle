use anyhow::{Result};

use npuzzle_solver::solver::NpuzzleSolver;


#[test]
fn three_raws_file() -> Result<()> {
    let solver: NpuzzleSolver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-3-1.txt"))?;
    assert_eq!(solver.size, 3);
    assert_eq!(vec![3, 2, 6, 1, 4, 0, 8, 7, 5], solver.start.0);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 0], solver.target.0);
    Ok(())
}


#[test]
fn three_raws_file_with_comments() -> Result<()> {
    let solver: NpuzzleSolver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-3-2.txt"))?;
    assert_eq!(solver.size, 3);
    assert_eq!(vec![3, 2, 6, 1, 4, 0, 8, 7, 5], solver.start.0);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 0], solver.target.0);
    Ok(())
}

#[test]
fn four_raws_file() -> Result<()> {
    let solver: NpuzzleSolver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-4-1.txt"))?;
    assert_eq!(solver.size, 4);
    assert_eq!(vec![3, 2, 4, 0, 1, 8, 6, 5, 7, 9, 10, 11, 13, 14, 12, 15], solver.start.0);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 , 13, 14, 15, 0], solver.target.0);
    Ok(())
}

#[test]
fn test_invalid_path() -> Result<()> {
    match NpuzzleSolver::from_file(&String::from("tests/puzzles/np4-1.txt")) {
        Ok(_) => panic!("Invalid_path should have returned an error"),
        Err(_) => return Ok(())
    };
}

#[test]
fn test_empty_file() -> Result<()> {
    match NpuzzleSolver::from_file(&String::from("tests/puzzles/empty.txt")) {
        Ok(_) => panic!("Empty_file should have returned an error"),
        Err(_) => return Ok(())
    };
}

#[test]
fn test_size_missing_file() -> Result<()> {
    match NpuzzleSolver::from_file(&String::from("tests/puzzles/size_missing.txt")) {
        Ok(_) => panic!("Size_missing should have returned an error"),
        Err(_) => return Ok(())
    };
}

#[test]
fn test_puzzle_bigger_than_size() -> Result<()> {
    match NpuzzleSolver::from_file(&String::from("tests/puzzles/puzzle_bigger_than_size.txt")) {
        Ok(_) => panic!("Puzzle_bigger_than_size should have returned an error"),
        Err(_) => return Ok(())
    };
}

#[test]
fn test_puzzle_smaller_than_size() -> Result<()> {
    match NpuzzleSolver::from_file(&String::from("tests/puzzles/puzzle_smaller_than_size.txt")) {
        Ok(_) => panic!("Puzzle_bigger_than_size should have returned an error"),
        Err(_) => return Ok(())
    };
}