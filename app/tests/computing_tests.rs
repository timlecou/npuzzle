use anyhow::{Result};
use npuzzle::NPuzzle;

#[path = "../src/npuzzle.rs"]
mod npuzzle;

#[test]
fn has_reached_goal_on_size_3_unfinished_puzzle() -> Result<()> {
    let puz: NPuzzle = NPuzzle::new(&String::from("tests/puzzles/unfinished-3-0.txt"))?;
    assert_eq!(puz.has_reached_goal(), false);
    Ok(())
}

#[test]
fn has_reached_goal_on_size_5_unfinished_puzzle() -> Result<()> {
    let puz: NPuzzle = NPuzzle::new(&String::from("tests/puzzles/unfinished-5-0.txt"))?;
    assert_eq!(puz.has_reached_goal(), false);
    Ok(())
}

#[test]
fn has_reached_goal_on_size_3_finished_puzzle() -> Result<()> {
    let puz: NPuzzle = NPuzzle::new(&String::from("tests/puzzles/finished-3-0.txt"))?;
    assert_eq!(puz.has_reached_goal(), true);
    Ok(())
}

#[test]
fn has_reached_goal_on_size_5_finished_puzzle() -> Result<()> {
    let puz: NPuzzle = NPuzzle::new(&String::from("tests/puzzles/finished-5-0.txt"))?;
    assert_eq!(puz.has_reached_goal(), true);
    Ok(())
}

#[test]
fn get_possible_moves_1() -> Result<()> {
    let puz: NPuzzle = NPuzzle::new(&String::from("tests/puzzles/unfinished-5-0.txt"))?;
    let (moves, size) = puz.get_possible_moves();
    assert_eq!(moves, vec![1, -1, 5, -5]);
    Ok(())
}

#[test]
fn get_possible_moves_2() -> Result<()> {
    let puz: NPuzzle = NPuzzle::new(&String::from("tests/puzzles/unfinished-3-0.txt"))?;
    let (moves, size) = puz.get_possible_moves();
    assert_eq!(moves, vec![-1, 3, -3]);
    Ok(())
}

#[test]
fn get_possible_moves_3() -> Result<()> {
    let puz: NPuzzle = NPuzzle::new(&String::from("tests/puzzles/unfinished-4-0.txt"))?;
    let (moves, size) = puz.get_possible_moves();
    assert_eq!(moves, vec![1, 4]);
    Ok(())
}

#[test]
fn get_possible_moves_4() -> Result<()> {
    let puz: NPuzzle = NPuzzle::new(&String::from("tests/puzzles/unfinished-4-1.txt"))?;
    let (moves, size) = puz.get_possible_moves();
    assert_eq!(moves, vec![-1, 4]);
    Ok(())
}

#[test]
fn get_possible_moves_5() -> Result<()> {
    let puz: NPuzzle = NPuzzle::new(&String::from("tests/puzzles/unfinished-4-2.txt"))?;
    let (moves, size) = puz.get_possible_moves();
    assert_eq!(moves, vec![-1, -4]);
    Ok(())
}

#[test]
fn get_possible_moves_6() -> Result<()> {
    let puz: NPuzzle = NPuzzle::new(&String::from("tests/puzzles/unfinished-4-3.txt"))?;
    let (moves, size) = puz.get_possible_moves();
    assert_eq!(moves, vec![1, -4]);
    Ok(())
}

#[test]
fn compute_next_states_1() -> Result<()> {
    let puz: NPuzzle = NPuzzle::new(&String::from("tests/puzzles/unfinished-4-0.txt"))?;
    let next_states: Vec<Vec<u16>> = puz.compute_next_states();

    assert_eq!(next_states.len(), 2);
    assert_eq!(vec![2, 0, 4, 3, 1, 8, 6, 5, 7, 9, 10, 11, 13, 14, 12, 15], next_states[0]);
    assert_eq!(vec![1, 2, 4, 3, 0, 8, 6, 5, 7, 9, 10, 11, 13, 14, 12, 15], next_states[1]);
    Ok(())
}

#[test]
fn compute_next_states_2() -> Result<()> {
    let puz: NPuzzle = NPuzzle::new(&String::from("tests/puzzles/unfinished-3-1.txt"))?;
    let next_states: Vec<Vec<u16>> = puz.compute_next_states();

    assert_eq!(next_states.len(), 4);
    assert_eq!(vec![1, 2, 3, 5, 6, 0, 8, 7, 4], next_states[0]);
    assert_eq!(vec![1, 2, 3, 0, 5, 6, 8, 7, 4], next_states[1]);
    assert_eq!(vec![1, 2, 3, 5, 7, 6, 8, 0, 4], next_states[2]);
    assert_eq!(vec![1, 0, 3, 5, 2, 6, 8, 7, 4], next_states[3]);
    Ok(())
}

#[test]
fn compute_next_states_3() -> Result<()> {
    let puz: NPuzzle = NPuzzle::new(&String::from("tests/puzzles/unfinished-3-2.txt"))?;
    let next_states: Vec<Vec<u16>> = puz.compute_next_states();

    assert_eq!(next_states.len(), 3);
    assert_eq!(vec![1, 2, 3, 5, 0, 6, 8, 7, 4], next_states[0]);
    assert_eq!(vec![1, 2, 3, 8, 5, 6, 0, 7, 4], next_states[1]);
    assert_eq!(vec![0, 2, 3, 1, 5, 6, 8, 7, 4], next_states[2]);
    Ok(())
}

#[test]
fn manhattan_distance_1() -> Result<()> {
    let puz: NPuzzle = NPuzzle::new(&String::from("tests/puzzles/unfinished-2-0.txt"))?;
    let next_states: Vec<Vec<u16>> = puz.compute_next_states();
    let mut distances: Vec<u32> = Vec::new();

    for state in next_states {
        distances.push(puz.manhattan_distance(&state));
    }
    assert_eq!(3, distances[0]);
    assert_eq!(1, distances[1]);
    Ok(())
}

#[test]
fn manhattan_distance_2() -> Result<()> {
    let puz: NPuzzle = NPuzzle::new(&String::from("tests/puzzles/unfinished-3-3.txt"))?;
    let next_states: Vec<Vec<u16>> = puz.compute_next_states();
    let mut distances: Vec<u32> = Vec::new();

    for state in next_states {
        NPuzzle::print_puzzle(&state, 3);
        print!("\n");
        distances.push(puz.manhattan_distance(&state));
    }
    assert_eq!(15, distances[0]);
    assert_eq!(13, distances[1]);
    assert_eq!(13, distances[2]);
    assert_eq!(13, distances[3]);
    Ok(())
}