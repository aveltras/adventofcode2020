use std::{collections::HashSet, fs};

pub fn solve() -> (usize, usize) {
    let input = fs::read_to_string("src/day06/input.txt").unwrap();
    let lines = input.split("\n\n");

    let mut total_anyone = 0;
    let mut total_everyone = 0;

    for line in lines {
        let charset: HashSet<char> = line.chars().filter(|c| c.is_alphabetic()).collect();
        total_anyone += charset.len();

        let mut common: HashSet<char> = ('a'..='z').collect();

        for answer in line.lines() {
            common = common
                .intersection(&answer.chars().collect())
                .copied()
                .collect();
        }

        total_everyone += common.len();
    }

    (total_anyone, total_everyone)
}
