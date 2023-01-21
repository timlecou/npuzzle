use anyhow::{Result};

use npuzzle_solver::solver::NpuzzleSolver;


#[test]
fn three_raws_file() -> Result<()> {
    let solver: NpuzzleSolver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-3-1.txt"))?;
    assert_eq!(solver.size, 3);
    assert_eq!(vec![3, 2, 6, 1, 4, 0, 8, 7, 5], solver.start.0);
    assert_eq!(vec![1, 2, 3, 8, 0, 4, 7, 6, 5], solver.target.0);
    Ok(())
}


#[test]
fn three_raws_file_with_comments() -> Result<()> {
    let solver: NpuzzleSolver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-3-2.txt"))?;
    assert_eq!(solver.size, 3);
    assert_eq!(vec![3, 2, 6, 1, 4, 0, 8, 7, 5], solver.start.0);
    assert_eq!(vec![1, 2, 3, 8, 0, 4, 7, 6, 5], solver.target.0);
    Ok(())
}

#[test]
fn four_raws_file() -> Result<()> {
    let solver: NpuzzleSolver = NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-4-1.txt"))?;
    assert_eq!(solver.size, 4);
    assert_eq!(vec![3, 2, 4, 0, 1, 8, 6, 5, 7, 9, 10, 11, 13, 14, 12, 15], solver.start.0);
    assert_eq!(vec![1, 2, 3, 4, 12, 13, 14, 5, 11, 0, 15, 6, 10, 9, 8, 7], solver.target.0);
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
        Ok(_) => panic!("Puzzle_smaller_than_size should have returned an error"),
        Err(_) => return Ok(())
    };
}

#[test]
fn test_misformed_puzzle() -> Result<()> {
    match NpuzzleSolver::from_file(&String::from("tests/puzzles/misformed_puzzle.txt")) {
        Ok(_) => panic!("Misformed_puzzle should have returned an error"),
        Err(_) => return Ok(())
    };
}

#[test]
fn test_two_puzzles_in_same_file() -> Result<()> {
    match NpuzzleSolver::from_file(&String::from("tests/puzzles/two_puzzles.txt")) {
        Ok(_) => panic!("Two_puzzles_in_same_file should have returned an error"),
        Err(_) => return Ok(())
    };
}

#[test]
fn test_many_sizes_1() -> Result<()> {
    match NpuzzleSolver::from_file(&String::from("tests/puzzles/many_sizes_1.txt")) {
        Ok(_) => panic!("Too_many_sizes_1 should have returned an error"),
        Err(_) => return Ok(())
    };
}

#[test]
fn test_many_sizes_2() -> Result<()> {
    match NpuzzleSolver::from_file(&String::from("tests/puzzles/many_sizes_2.txt")) {
        Ok(_) => panic!("Too_many_size_2 should have returned an error"),
        Err(_) => return Ok(())
    };
}

#[test]
fn test_many_sizes_3() -> Result<()> {
    match NpuzzleSolver::from_file(&String::from("tests/puzzles/many_sizes_3.txt")) {
        Ok(_) => panic!("Too_many_sizes_3 should have returned an error"),
        Err(_) => return Ok(())
    };
}

#[test]
fn test_size_after_puzzle() -> Result<()> {
    match NpuzzleSolver::from_file(&String::from("tests/puzzles/size_after_puzzle.txt")) {
        Ok(_) => panic!("Size_after_puzzle should have returned an error"),
        Err(_) => return Ok(())
    };
}

#[test]
fn test_size_but_no_puzzle() -> Result<()> {
    match NpuzzleSolver::from_file(&String::from("tests/puzzles/size_but_no_puzzle.txt")) {
        Ok(_) => panic!("Size_but_no_puzzle should have returned an error"),
        Err(_) => return Ok(())
    };
}

#[test]
fn test_puzzle_but_no_size() -> Result<()> {
    match NpuzzleSolver::from_file(&String::from("tests/puzzles/puzzle_but_no_size.txt")) {
        Ok(_) => panic!("Puzzle_but_no_size should have returned an error"),
        Err(_) => return Ok(())
    };
}

#[test]
fn test_duplicate_number_in_puzzle() -> Result<()> {
    match NpuzzleSolver::from_file(&String::from("tests/puzzles/duplicate_number_in_puzzle.txt")) {
        Ok(_) => panic!("Duplicate_number should have returned an error"),
        Err(_) => return Ok(())
    };
}

#[test]
fn test_missing_number_in_puzzle() -> Result<()> {
    match NpuzzleSolver::from_file(&String::from("tests/puzzles/missing_number_in_puzzle.txt")) {
        Ok(_) => panic!("Missing_number should have returned an error"),
        Err(_) => return Ok(())
    };
}

#[test]
fn test_too_big_number_in_puzzle() -> Result<()> {
    match NpuzzleSolver::from_file(&String::from("tests/puzzles/too_big_number_in_puzzle.txt")) {
        Ok(_) => panic!("Too_big_number should have returned an error"),
        Err(_) => return Ok(())
    };
}

#[test]
fn test_negative_number_in_puzzle() -> Result<()> {
    match NpuzzleSolver::from_file(&String::from("tests/puzzles/negative_number_in_puzzle.txt")) {
        Ok(_) => panic!("Negative_number_than_size should have returned an error"),
        Err(_) => return Ok(())
    };
}

#[test]
fn test_negative_size() -> Result<()> {
    match NpuzzleSolver::from_file(&String::from("tests/puzzles/negative_size.txt")) {
        Ok(_) => panic!("Negative_size should have returned an error"),
        Err(_) => return Ok(())
    };
}

#[test]
fn test_one_sized_puzzle() -> Result<()> {
    match NpuzzleSolver::from_file(&String::from("tests/puzzles/npuzzle-1-0.txt")) {
        Ok(_) => panic!("One_sized_puzzle should have returned an error"),
        Err(_) => return Ok(())
    };
}