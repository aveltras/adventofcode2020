use std::{borrow::Borrow, fs, str::FromStr};

struct Password {
    min_occurences: usize,
    max_occurences: usize,
    letter: char,
    text: String,
}

impl FromStr for Password {
    type Err = &'static str;

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

pub fn solve() -> Result<usize, &'static str> {
    let part1 = solve_part1()?;
    Ok(part1)
}

pub fn solve_part1() -> Result<usize, &'static str> {
    let contents = fs::read_to_string("src/day02/input.txt").expect("boom");
    let lines = contents.lines();
    let mut total: usize = 0;

    for line in lines {
        let password: Password = line.parse()?;
        let occurences = password
            .text
            .chars()
            .filter(|x| x == password.letter.borrow())
            .count();

        if occurences >= password.min_occurences && occurences <= password.max_occurences {
            total += 1;
        }
    }

    Ok(total)
}
