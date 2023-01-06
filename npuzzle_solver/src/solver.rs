use anyhow::{ Result, bail };
use indexmap::{
    map::Entry,
    IndexMap
};
use std::{
    collections::BinaryHeap,
    fmt::Display,
    fs::File,
    io::{
        prelude::*,
        BufReader
    },
    cmp::Ordering
};

use crate::heuristics::Heuristics;


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Board(pub Vec<u16>);

pub struct SmallestCostHolder {
    estimated_cost: usize,
    cost: usize,
    index: usize,
}

impl PartialEq for SmallestCostHolder {
    fn eq(&self, other: &Self) -> bool {
        self.estimated_cost.eq(&other.estimated_cost) && self.cost.eq(&other.cost)
    }
}

impl Eq for SmallestCostHolder {}

impl PartialOrd for SmallestCostHolder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SmallestCostHolder {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.estimated_cost.cmp(&self.estimated_cost) {
            Ordering::Equal => other.cost.cmp(&other.cost),
            s => s
        }
    }
}

pub struct NpuzzleResults {
    pub states: Option<Vec<Board>>,
    pub max_states_in_memory: usize,
    pub max_states_in_opened: usize
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = (self.0.len() as f64).sqrt() as usize;

        for (idx, nb) in self.0.iter().enumerate() {
            if idx % size == 0 {
                write!(f, "\n")?;
            } else {
                write!(f, " ")?;
            }
            write!(f, "{}", nb)?;
        }
        Ok(())
    }
}

impl Board {
    pub fn inner(&self) -> &Vec<u16> {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut Vec<u16> {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct NpuzzleSolver {
    pub start: Board,
    pub target: Board,
    pub size: usize
}

impl Display for NpuzzleResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.states {
            Some(states) => {
                for st in states {
                    write!(f, "{}", st)?;
                    write!(f, "    |")?;
                    write!(f, "    |")?;
                    write!(f, "  \\ | /")?;
                    write!(f, "   \\|/")?;
                }
                write!(f, "Complexity in time: {}", self.max_states_in_opened)?;
                write!(f, "Complexity in size: {}", self.max_states_in_memory)?;
                write!(f, "Number of moves: {}", states.len())?;
                Ok(())
            },
            None => write!(f, "No solution found !")
        }
    }
}

impl Board {
    fn new(size: usize) -> Board {
        Board(vec![0; size * size])
    }
}

impl NpuzzleSolver {
    pub fn from_file(filename: &String) -> Result<NpuzzleSolver> {

        fn parse_file(file_path: &String) -> Result<(usize, Board)> {
            let file = File::open(file_path)?;
            let reader = BufReader::new(file);
            let mut puzzle: Board = Board::new(0);
            let mut size: usize = 0;
    
            for line in reader.lines() {
                match line {
                    Ok(v) => {
                        if v.len() > 0 {
                            let mut numbers: Vec<u16> = v
                                .split_whitespace()
                                .filter_map(|w| w.parse().ok())
                                .collect();
                            if numbers.len() == 1 {
                                if size == 0 && puzzle.0.len() == 0 {
                                    size = numbers[0] as usize;
                                } else {
                                    bail!("Too many sizes provided in file");
                                }
                            } else if numbers.len() == size {
                                puzzle.0.append(&mut numbers);
                            }
                        }
                    },
                    Err(e) => bail!("{}", e)
                }
            }
            if size == 0 || puzzle.0.len() != size * size {
                bail!("Unstructured data in file");
            }
            Ok((size, puzzle))
        }

        pub fn check_puzzle_conformity(puzzle: &Board, size: usize) -> Result<()> {
            let mut i: u16 = 0;
            
            if size * size != puzzle.0.len() {
                bail!("\n\nSize provided doesn't match the puzzle amount of numbers\n\nsize provided: {}\namount of number: {}\n\n", size, puzzle.0.len());
            }
        
            while (i as usize) < puzzle.0.len() {
                if puzzle.0.contains(&i) == false {
                    bail!("Missing number in puzzle\n\nMissing: {}\n\n", i);
                }
                i += 1;
            }
            Ok(())
        }

        let (size, puzzle) = parse_file(filename)?;
        let mut target: Board = Board::new(0);

        target.0 = puzzle.0.clone();
        target.0.sort();
        target.0.remove(0);
        target.0.push(0);

        check_puzzle_conformity(&puzzle, size)?;
        Ok(Self {
            start: puzzle,
            target: target,
            size: size
        })
    }

    pub fn get_possible_moves(size: &usize, empty_idx: usize) -> [Option<usize>; 4] {
        [
            {
                if empty_idx % size > 0 {
                    Some(empty_idx - 1)
                } else {
                    None
                }
            },
            {
                if empty_idx / size > 0 {
                    Some(empty_idx - size)
                } else {
                    None
                }
            },
            {
                if (empty_idx + 1) % size > 0 {
                    Some(empty_idx + 1)
                } else {
                    None
                }
            },
            {
                if (empty_idx + 1) / size > 0 {
                    Some(empty_idx + size)
                } else {
                    None
                }
            }
        ]
    }

    pub fn get_successors(puzzle: &Board, size: usize) -> Vec<Board> {
        let empty_idx: usize = puzzle.inner().iter().position(|nb| nb == &0).unwrap();
        let moves = NpuzzleSolver::get_possible_moves(&size, empty_idx);

        moves
            .iter()
            .copied()
            .filter_map(|s| s)
            .map(|mv| {
                let mut board: Board = puzzle.clone();
                board.inner_mut().swap(empty_idx, mv);
                board
            })
            .collect()
    }

    pub fn solve_astar(&self, heuristic: Heuristics) -> NpuzzleResults {
        let mut to_see = BinaryHeap::new();
        let mut states_in_opened: usize = 0;
        let mut states_in_memory: usize = 0;
        to_see.push(SmallestCostHolder {
            estimated_cost: 0,
            cost: 0,
            index: 0,
        });
        let mut parents: IndexMap<Board, (usize, usize)> = IndexMap::default();
        parents.insert(self.start.clone(), (usize::max_value(), 0));
        while let Some(SmallestCostHolder { cost, index, .. }) = to_see.pop() {
            let successors = {
                let (node, &(_, c)) = parents.get_index(index).unwrap();
                if node == &self.target {
                    //reverse path
                    let reversed_path: Vec<Board> = Vec::new();
                    return NpuzzleResults {
                        states: Some(reversed_path),
                        max_states_in_opened: states_in_opened,
                        max_states_in_memory: states_in_memory
                    };
                }
                if cost > c {
                    continue ;
                }
                NpuzzleSolver::get_successors(node, self.size)
            };
        }
        NpuzzleResults {
            states: None,
            max_states_in_opened: states_in_opened,
            max_states_in_memory: states_in_memory
        }
    }

    pub fn is_solvable(puzzle: &Board, size: usize) -> bool {
        let inversions: usize = NpuzzleSolver::count_inversions(puzzle);
        let blank_square_row = puzzle
            .inner()
            .iter()
            .position(|nb| nb == &0)
            .unwrap() / size;

        if size % 2 == 0 {
            if (inversions + blank_square_row) % 2 == 1 {
                return true;
            }
        } else if inversions % 2 == 0 {
            return true;
        }
        false
    }

    pub fn count_inversions(puzzle: &Board) -> usize {
        let mut inversions: usize = 0;

        for (idx_1, nb_1) in puzzle.inner().iter().enumerate() {
            if nb_1 != &0 {
                for nb_2 in puzzle.inner().iter().skip(idx_1 + 1) {
                    if nb_2 != &0 && nb_2 < nb_1 {
                        inversions += 1;
                    }
                }
            }
        }
        inversions
    }
}