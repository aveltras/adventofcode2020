use std::io;
use std::{borrow::Borrow, fs, str::FromStr};

struct Password {
    min_occurences: usize,
    max_occurences: usize,
    letter: char,
    text: String,
}

impl FromStr for Password {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let range = parts[0];
        let letter = parts[1].chars().next().unwrap();
        let text = parts[2].to_owned();
        let range_parts: Vec<&str> = range.split('-').collect();
        let min_occurences = range_parts[0].parse().unwrap();
        let max_occurences = range_parts[1].parse().unwrap();
        Ok(Password {
            min_occurences,
            max_occurences,
            letter,
            text,
        })
    }
}

pub fn solve() -> (usize, usize) {
    let part1 = solve_part1();
    let part2 = solve_part2();
    (part1, part2)
}

pub fn solve_part1() -> usize {
    let contents = fs::read_to_string("src/day02/input.txt").unwrap();
    let lines = contents.lines();
    let mut total: usize = 0;

    for line in lines {
        let password: Password = line.parse().unwrap();
        let occurences = password
            .text
            .chars()
            .filter(|x| x == password.letter.borrow())
            .count();

        if occurences >= password.min_occurences && occurences <= password.max_occurences {
            total += 1;
        }
    }

    total
}

pub fn solve_part2() -> usize {
    let contents = fs::read_to_string("src/day02/input.txt").unwrap();
    let lines = contents.lines();
    let mut total: usize = 0;

    for line in lines {
        let password: Password = line.parse().unwrap();
        if let Some(letter_at_pos1) = password.text.chars().nth(password.min_occurences - 1) {
            if let Some(letter_at_pos2) = password.text.chars().nth(password.max_occurences - 1) {
                let mut valid = 0;

                if letter_at_pos1 == password.letter {
                    valid += 1;
                }

                if letter_at_pos2 == password.letter {
                    valid += 1;
                }

                if valid == 1 {
                    total += 1
                }
            }
        }
    }

    total
}
