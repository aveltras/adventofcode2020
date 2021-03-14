use std::{collections::HashSet, fs};

pub fn solve() -> (usize, usize) {
    let input = fs::read_to_string("src/day06/input.txt").unwrap();
    let lines = input.split("\n\n");

    let mut total = 0;

    for line in lines {
        let mut answers = HashSet::<char>::new();
        let charset: HashSet<char> = line.chars().filter(|c| c.is_alphabetic()).collect();
        total += charset.len();
    }

    (total, 1)
}
