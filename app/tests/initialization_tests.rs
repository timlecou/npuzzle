use std::collections::LinkedList;

use npuzzle::NPuzzle;

#[path = "../src/npuzzle.rs"]
mod npuzzle;

#[test]
fn three_raws_file() {
    let mut args: Vec<String> = Vec::new();
    args.push(String::from("/Users/timlecou/Documents/projects_42/github/npuzzle/app/src/main.rs"));
    args.push(String::from("/Users/timlecou/Documents/projects_42/github/npuzzle/app/npuzzle-3-1.txt"));
    let puz: NPuzzle = match NPuzzle::new(&args) {
        Some(p) => p,
        None => panic!("No return")
    };
    let mut big_vec: Vec<u8> = Vec::new();
    let mut i: usize = 0;

    assert_eq!(puz.get_size(), 3);
    assert_eq!(puz.get_puzzle().len(), 3);
    for v in &puz.get_puzzle() {
        assert_eq!(v.len(), 3);
        big_vec.extend(v);
    }
    assert_eq!(vec![3, 2, 6], puz.get_puzzle()[0]);
    assert_eq!(vec![1, 4, 0], puz.get_puzzle()[1]);
    assert_eq!(vec![8, 7, 5], puz.get_puzzle()[2]);

    assert_eq!(big_vec.len(), 9);

    while i < big_vec.len() {
        let mut tmp: u8 = 0;
        match u8::try_from(i) {
            Ok(nb) => tmp = nb,
            Err(e) => panic!("{e}")
        };
        assert!(big_vec.contains(&tmp));
        i += 1;
    }
}

#[test]
fn six_raws_file() {
    let mut args: Vec<String> = Vec::new();
    args.push(String::from("/Users/timlecou/Documents/projects_42/github/npuzzle/app/src/main.rs"));
    args.push(String::from("/Users/timlecou/Documents/projects_42/github/npuzzle/app/npuzzle-6-1.txt"));
    let puz: NPuzzle = match NPuzzle::new(&args) {
        Some(p) => p,
        None => panic!("No return")
    };

    assert_eq!(puz.get_size(), 6);
    assert_eq!(puz.get_puzzle().len(), 6);
    for v in puz.get_puzzle() {
        assert_eq!(v.len(), 6);
    }
    assert_eq!(vec![1, 2, 3, 4, 5, 6], puz.get_puzzle()[0]);
    assert_eq!(vec![2, 3, 4, 5, 6, 7], puz.get_puzzle()[1]);
    assert_eq!(vec![3, 4, 5, 6, 7, 8], puz.get_puzzle()[2]);
    assert_eq!(vec![4, 5, 6, 7, 8, 9], puz.get_puzzle()[3]);
    assert_eq!(vec![5, 6, 7, 8, 9, 10], puz.get_puzzle()[4]);
    assert_eq!(vec![6, 7, 8, 9, 10, 11], puz.get_puzzle()[5]);
}

#[test]
fn three_raws_file_with_comments() {
    let mut args: Vec<String> = Vec::new();
    args.push(String::from("/Users/timlecou/Documents/projects_42/github/npuzzle/app/src/main.rs"));
    args.push(String::from("/Users/timlecou/Documents/projects_42/github/npuzzle/app/npuzzle-3-2.txt"));
    let puz: NPuzzle = match NPuzzle::new(&args) {
        Some(p) => p,
        None => panic!("No return")
    };

    assert_eq!(puz.get_size(), 3);
    assert_eq!(puz.get_puzzle().len(), 3);
    for v in puz.get_puzzle() {
        assert_eq!(v.len(), 3);
    }
    assert_eq!(vec![3, 2, 6], puz.get_puzzle()[0]);
    assert_eq!(vec![1, 4, 0], puz.get_puzzle()[1]);
    assert_eq!(vec![8, 7, 5], puz.get_puzzle()[2]);
}