use anyhow::{ Result, bail };
use std::fmt::Display;
use std::fs::File;
use std::io::{prelude::*, BufReader};


#[derive(Debug)]
pub struct Board(pub Vec<u16>);

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

#[derive(Debug)]
pub struct NpuzzleSolver {
    pub start: Board,
    pub target: Board,
    pub size: usize
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
                            if numbers.len() == 1 && size == 0 {
                                size = numbers[0] as usize;
                            }
                            else if numbers.len() > 1 {
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

        let (size, puzzle) = parse_file(filename)?;
        let mut target: Board = Board::new(0);

        target.0 = puzzle.0.clone();
        target.0.sort();
        target.0.remove(0);
        target.0.push(0);
        Ok(Self {
            start: puzzle,
            target: target,
            size: size
        })
    }
}