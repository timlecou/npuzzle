use anyhow::{ Result, bail };

pub fn check_puzzle_conformity(puzzle: &Vec<u16>, size: usize) -> Result<()> {
    let mut i: u16 = 0;
    
    if size * size != puzzle.len() {
        bail!("\n\nSize provided doesn't match the puzzle amount of numbers\n\nsize provided: {}\namount of number: {}\n\n", size, puzzle.len());
    }

    while (i as usize) < puzzle.len() {
        if puzzle.contains(&i) == false {
            bail!("Missing number in puzzle\n\nMissing: {}\n\n", i);
        }
        i += 1;
    }
    Ok(())
}

pub fn print_vec_slice(sl: &[u16]) {
    for nb in sl {
        print!("{} ", nb)
    }
    print!("\n")
}