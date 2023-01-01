
use std::fs::File;
use std::io::{prelude::*, BufReader};
use utils::{
    check_puzzle_conformity,
    print_vec_slice
};
use state::State;
use rand::thread_rng;
use anyhow::{Result, bail};

#[path = "utils/mod.rs"]
mod utils;
#[path = "state/mod.rs"]
mod state;

#[derive(Debug)]
pub struct NPuzzle {
    size: usize,
    number_amount: usize,
    puzzle: Vec<u16>,
    goal: Vec<u16>
}

impl NPuzzle {
    pub fn new(file_path: &String) -> Result<NPuzzle> {
        let (size, puzzle) = Self::parse_file(file_path)?;
        let mut goal: Vec<u16> = puzzle.clone();
        goal.sort();
        goal.remove(0);
        goal.push(0);

        check_puzzle_conformity(&puzzle, size)?;

        Ok(Self {
            size: size,
            number_amount: size * size,
            puzzle: puzzle,
            goal: goal
        })
    }

    fn parse_file(file_path: &String) -> Result<(usize, Vec<u16>)> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut puzzle: Vec<u16> = Vec::new();
        let mut size: usize = 0;

        for line in reader.lines() {
            match line {
                Ok(v) => {
                    if v.len() > 0 {
                        let mut numbers: Vec<u16> = v
                            .split_whitespace()
                            .filter_map(|w| w.parse().ok())
                            .collect();
                        if numbers.len() == 1 && size == 0 {
                            size = numbers[0] as usize;
                        }
                        else if numbers.len() > 1 {
                            puzzle.append(&mut numbers);
                        }
                    }
                },
                Err(e) => bail!("{}", e)
            }
        }
        Ok((size, puzzle))
    }

    fn print_u16_vec(&self, vec: &Vec<u16>) {
        let mut idx: u16 = 0;
        let mut size: usize = self.size;
        while (idx as usize) < self.number_amount {
            print_vec_slice(&vec[idx as usize..size]);
            idx = size as u16;
            size += self.size;
        }
    }

    pub fn print_puzzle(vec: &Vec<u16>, size: usize) {
        let mut idx: u16 = 0;
        let mut bsize: usize = size;
        while (idx as usize) < size * size {
            print_vec_slice(&vec[idx as usize..bsize]);
            idx = bsize as u16;
            bsize += size;
        }
    }

    pub fn has_reached_goal(&self) -> bool {
        let matching = self.puzzle.iter().zip(&self.goal).filter(|&(a, b)| a == b).count();
        if matching == self.number_amount {
            true
        }
        else {
            false
        }
    }

    pub fn get_possible_moves(&self) -> (&Vec<i16>, &usize) {
        let mut moves: Vec<i16> = Vec::new();
        let possibilities: [i16; 4] = [1, -1, self.size as i16, self.size as i16 * -1];
        let idx: usize = self.puzzle.iter().position(|&nb| nb == 0).unwrap();
        let size_steping_range: Vec<usize> = ((self.size - 1)..self.number_amount).step_by(self.size).collect();
        let size_steping_range_from_0: Vec<usize> = (0..self.number_amount).step_by(self.size).collect();

        for p in possibilities {
            if idx as i16 + p >= 0 && idx as i16 + p < self.number_amount as i16 {
                if p == 1 && size_steping_range.contains(&idx) {
                    continue ;
                }
                if p == -1 && size_steping_range_from_0.contains(&idx) {
                    continue ;
                }
                moves.push(p);
            }
        }
        (&moves, &idx)
    }

    pub fn compute_next_states(&self) -> Result<&Vec<State>> {
        let mut next_states: Vec<State> = Vec::new();
        let (possible_moves, idx) = self.get_possible_moves();
        
        let mut tmp: u16;

        for mv in possible_moves {
            let mut state: State = State::new(self.size, &self.puzzle, *mv)?;
            tmp = state.puzzle[(*idx as i16 + mv) as usize];
            state.puzzle[(*idx as i16 + mv) as usize] = state.puzzle[*idx];
            state.puzzle[*idx] = tmp;
            next_states.push(state);
        }
        Ok(&next_states)
    }

    pub fn manhattan_distance(&self, state: &mut State) {
        let mut current_distance: u32;
        let mut goal_idx: i16;
        let mut puzzle_idx: i16;
        let mut goal_x: i16;
        let mut goal_y: i16;
        let mut puzzle_x: i16;
        let mut puzzle_y: i16;

        for nb in state.puzzle.clone() {
            if nb != 0 {
                goal_idx = self.goal.iter().position(|&n| n == nb).unwrap() as i16;
                puzzle_idx = state.puzzle.iter().position(|&n| n == nb).unwrap() as i16;
                if goal_idx == puzzle_idx {
                    continue ;
                }
                (goal_x, goal_y) = (goal_idx % self.size as i16, goal_idx / self.size as i16);
                (puzzle_x, puzzle_y) = (puzzle_idx % self.size as i16, puzzle_idx / self.size as i16);
                current_distance = ((goal_x - puzzle_x).abs() + (goal_y - puzzle_y).abs()) as u32;
                state.distance += current_distance;
            }
        }
    }

    fn huristic_next_state(&mut self) -> Result<()> {
        /* get the possible next states */
        let mut next_states: &Vec<State> = self.compute_next_states()?;
        let mut rng = thread_rng();
        
        for mut state in next_states {
            self.manhattan_distance(&mut state);
        }
        next_states.sort();
        next_states.drain(next_states.iter().filter(|&n| n.distance == next_states[0].distance).count()..next_states.len());

        let idx: usize = self.puzzle.iter().position(|&nb| nb == 0).unwrap();
        let tmp: u16 = self.puzzle[(idx as i16 + next_states[0].mov) as usize];
        self.puzzle[(idx as i16 + next_states[0].mov) as usize] = self.puzzle[idx];
        self.puzzle[idx] = tmp;
        Ok(())
    }

    pub fn solve(&mut self) -> Result<()> {
        while !self.has_reached_goal() {
            self.huristic_next_state()?;
            self.print_current_state();
            println!("---------------------");
            // break ;
        }
        Ok(())
    }

    pub fn print_current_state(&self) {
        self.print_u16_vec(&self.puzzle);
    }

    pub fn print_goal(&self) {
        self.print_u16_vec(&self.goal);
    }

    pub fn _get_size(&self) -> usize {
        self.size
    }

    pub fn _get_puzzle(&self) -> Vec<u16> {
        self.puzzle.clone()
    }
}