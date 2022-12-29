use npuzzle::NPuzzle;

#[path = "../src/npuzzle.rs"]
mod npuzzle;

#[test]
fn three_raws_file() {
    let mut args: Vec<String> = Vec::new();
    args.push(String::from("/Users/timlecou/Documents/projects_42/github/npuzzle/app/src/main.rs"));
    args.push(String::from("/Users/timlecou/Documents/projects_42/github/npuzzle/app/puzzles/npuzzle-3-1.txt"));
    let puz: NPuzzle = match NPuzzle::new(&args) {
        Ok(p) => p,
        Err(e) => panic!("Error: {}", e)
    };
    let mut big_vec: Vec<u16> = Vec::new();
    let mut i: usize = 0;

    assert_eq!(puz._get_size(), 3);
    assert_eq!(puz._get_puzzle().len(), 3);
    for v in &puz._get_puzzle() {
        assert_eq!(v.len(), 3);
        big_vec.extend(v);
    }
    assert_eq!(vec![3, 2, 6], puz._get_puzzle()[0]);
    assert_eq!(vec![1, 4, 0], puz._get_puzzle()[1]);
    assert_eq!(vec![8, 7, 5], puz._get_puzzle()[2]);

    assert_eq!(big_vec.len(), 9);

    while i < big_vec.len() {
        let mut tmp: u16 = 0;
        tmp = i as u16;
        assert!(big_vec.contains(&tmp));
        i += 1;
    }
}


#[test]
fn three_raws_file_with_comments() {
    let mut args: Vec<String> = Vec::new();
    args.push(String::from("/Users/timlecou/Documents/projects_42/github/npuzzle/app/src/main.rs"));
    args.push(String::from("/Users/timlecou/Documents/projects_42/github/npuzzle/app/puzzles/npuzzle-3-2.txt"));
    let puz: NPuzzle = match NPuzzle::new(&args) {
        Ok(p) => p,
        Err(e) => panic!("Error: {}", e)
    };

    assert_eq!(puz._get_size(), 3);
    assert_eq!(puz._get_puzzle().len(), 3);
    for v in puz._get_puzzle() {
        assert_eq!(v.len(), 3);
    }
    assert_eq!(vec![3, 2, 6], puz._get_puzzle()[0]);
    assert_eq!(vec![1, 4, 0], puz._get_puzzle()[1]);
    assert_eq!(vec![8, 7, 5], puz._get_puzzle()[2]);
}