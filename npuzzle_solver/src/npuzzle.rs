
use std::fs::File;
use std::{time, thread};
use std::io::{prelude::*, BufReader};
use utils::{
    check_puzzle_conformity,
    print_vec_slice
};
use state::State;
use anyhow::{Result, bail};

#[path = "utils/mod.rs"]
mod utils;
#[path = "state/mod.rs"]
mod state;

#[derive(Debug)]
pub struct NPuzzle {
    size: usize,
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
        while (idx as usize) < self.size * self.size {
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

    pub fn has_reached_goal(&self, state: &State) -> bool {
        let matching = state.puzzle.iter().zip(&self.goal).filter(|&(a, b)| a == b).count();
        if matching == self.size * self.size {
            true
        }
        else {
            false
        }
    }

    pub fn has_reached_goal2(&self, puzzle: &Vec<u16>) -> bool {
        let matching = puzzle.iter().zip(&self.goal).filter(|&(a, b)| a == b).count();
        if matching == self.size * self.size {
            true
        }
        else {
            false
        }
    }

    pub fn get_possible_moves(&self, state: &Vec<u16>) -> (Vec<i16>, usize) {
        let mut moves: Vec<i16> = Vec::new();
        let possibilities: [i16; 4] = [1, -1, self.size as i16, self.size as i16 * -1];
        let idx: usize = state.iter().position(|&nb| nb == 0).unwrap();

        let size_steping_range: Vec<usize> = ((self.size - 1)..self.size * self.size).step_by(self.size).collect();
        let size_steping_range_from_0: Vec<usize> = (0..self.size * self.size).step_by(self.size).collect();

        for p in possibilities {
            if idx as i16 + p >= 0 && idx as i16 + p < (self.size * self.size) as i16 {
                if p == 1 && size_steping_range.contains(&idx) {
                    continue ;
                }
                if p == -1 && size_steping_range_from_0.contains(&idx) {
                    continue ;
                }
                moves.push(p);
            }
        }
        (moves, idx)
    }

    fn get_successors<'a>(&self, node: &'a State) -> Result<Vec<State<'a>>> {
        let mut next_states: Vec<State> = Vec::new();
        let (possible_moves, idx) = self.get_possible_moves(&node.puzzle);
        
        let mut tmp: u16;

        for mv in possible_moves {
            let mut state: State = State::new(&node.puzzle)?;

            tmp = state.puzzle[(idx as i16 + mv) as usize];
            state.puzzle[(idx as i16 + mv) as usize] = state.puzzle[idx];
            state.puzzle[idx] = tmp;
            next_states.push(state);
        }
        Ok(next_states)
    }

    pub fn manhattan_distance(&self, puzzle: &Vec<u16>) -> Result<u32> {
        let mut current_distance: u32;
        let mut final_distance: u32 = 0;
        let mut goal_idx: i16;
        let mut puzzle_idx: i16;
        let mut goal_x: i16;
        let mut goal_y: i16;
        let mut puzzle_x: i16;
        let mut puzzle_y: i16;

        for nb in puzzle.clone() {
            if nb != 0 {
                goal_idx = self.goal.iter().position(|&n| n == nb).unwrap() as i16;
                puzzle_idx = puzzle.iter().position(|&n| n == nb).unwrap() as i16;
                if goal_idx == puzzle_idx {
                    continue ;
                }
                (goal_x, goal_y) = (goal_idx % self.size as i16, goal_idx / self.size as i16);
                (puzzle_x, puzzle_y) = (puzzle_idx % self.size as i16, puzzle_idx / self.size as i16);
                current_distance = ((goal_x - puzzle_x).abs() + (goal_y - puzzle_y).abs()) as u32;
                final_distance += current_distance;
            }
        }
        Ok(final_distance)
    }

    // fn huristic_next_state(&self, state: State, open: &mut LinkedList<Vec<u16>>) -> Result<State> {
    //     /* get the possible next states */
    //     let mut next_states: Vec<State> = self.compute_next_states(&state, open)?;

    //     if next_states.len() == 0 {
    //         bail!("No next state");
    //     }
        
    //     for next_state_idx in 0..next_states.len() {
    //         next_states[next_state_idx].distance = self.manhattan_distance(&next_states[next_state_idx].puzzle)?;
    //     }
    //     next_states.sort();
    //     next_states.drain(next_states.iter().filter(|&n| n.distance == next_states[0].distance).count()..next_states.len());
    //     let shortest_distance = next_states[0].distance;
    //     next_states.retain(|n| n.distance == shortest_distance);
        
    //     let state =  match next_states.choose(&mut rand::thread_rng()) {
    //         Some(v) => v,
    //         None => bail!("Random choose failed")
    //     };
        
    //     // let idx: usize = self.puzzle.iter().position(|&nb| nb == 0).unwrap();
    //     // let tmp: u16 = self.puzzle[(idx as i16 + state.mov) as usize];
    //     // self.puzzle[(idx as i16 + state.mov) as usize] = self.puzzle[idx];
    //     // self.puzzle[idx] = tmp;
    //     Ok(state.clone())
    // }

    // pub fn solve(&self) -> Result<()> {
    //     let mut open: LinkedList<Vec<u16>> = LinkedList::new();
    //     let mut close: LinkedList<Vec<u16>> = LinkedList::new();
    //     let mut next_state: State = State::new(self.size, &self.puzzle, 0)?;
    //     let mut g: u32 = 0;

    //     open.push_back(next_state.puzzle.to_vec());
    //     // while !self.has_reached_goal(&next_state) {
    //     while open.len() > 0 {
    //         // self.print_u16_vec(&next_state.puzzle);
    //         // println!("--->");
    //         open.push_back(next_state.puzzle.to_vec());
    //         next_state = self.huristic_next_state(next_state, &mut open)?;
            
    //         g += 1;
    //         // thread::sleep(time::Duration::from_millis(2000));
    //     }
    //     self.print_u16_vec(&next_state.puzzle);
    //     println!("\nNumber of states: {}", open.len());
    //     Ok(())
    // }

    // fn pop_and_return_smallest_f<'a>(&self, opened: &mut Vec<State<'a>>, closed: &Vec<State>, depth: u16) -> Result<State<'a>> {
    //     let mut shortest_distance_idx: usize = 0;
    //     let mut shortest_distance: u32 = 1000000;
    //     // let mut state: State = State::new(&vec![])?;

    //     for (idx, st) in opened.iter_mut().enumerate() {
    //         if st.distance == 0 {
    //             st.distance = depth as u32  + self.manhattan_distance(&st.puzzle)?;
    //         }
    //         if !closed.contains(&st) {
    //             if st.distance < shortest_distance {
    //                 shortest_distance = st.distance;
    //                 shortest_distance_idx = idx;
    //             }
    //         }
    //     }
    //     for (idx, st) in &mut opened.iter().enumerate() {
    //         if idx == shortest_distance_idx {
    //             // state = st.clone(); 
    //             opened.remove(idx);
    //             return Ok(st.clone());
    //             // break ;
    //         }
    //     }
    //     Err(anyhow!("Couldn't find smallest f in opened list"))
    // }

    fn get_smallest_f_idx(&self, opened: &mut Vec<State>, depth: u16) -> Result<usize> {
        let mut shortest_distance_idx: usize = 0;
        let mut shortest_distance: u32 = 1000000;

        for (idx, st) in opened.iter_mut().enumerate() {
            if st.distance == 0 {
                st.distance = depth as u32  + self.manhattan_distance(&st.puzzle)?;
            }
            // if !closed.contains(&st) {
            if st.distance < shortest_distance {
                shortest_distance = st.distance;
                shortest_distance_idx = idx;
            }
            // }
        }
        Ok(shortest_distance_idx)
    }

    pub fn solve2<'a>(&self) -> Result<()> {
        let mut opened: Vec<State> = Vec::new();
        let mut closed: Vec<State> = Vec::new();
        // let mut both: Vec<Vec<u16>> = Vec::new();
        let mut successors: Vec<State>;
        let mut depth: u16 = 0;
        let mut idx;
        let binding = State::new(&self.puzzle)?;
        let mut last_closed_state: &State;

        opened.push(binding);
        while !opened.is_empty() {
            idx = self.get_smallest_f_idx(&mut opened, depth)?;
            closed.push(opened.swap_remove(idx));
            if self.has_reached_goal(closed.last().expect("Error when reaching closed vec last element")) {
                break ;
            }
            // last_closed_state = &closed.last().unwrap();
            let c = closed.clone();
            last_closed_state = &c.last().cloned().unwrap();
            successors = self.get_successors(&last_closed_state)?;
            for successor in successors {
                if self.has_reached_goal(&successor) {
                    closed.push(successor);
                    break ;
                }
                else if !opened.contains(&successor) && !closed.contains(&successor) {
                    opened.push(successor);
                }
            }
            depth += 1;
            thread::sleep(time::Duration::from_millis(2000));
        }
        println!("pas trouvé");
        dbg!(closed.last().expect(""));
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