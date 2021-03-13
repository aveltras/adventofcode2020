use std::fs;
use std::io::Result;

pub fn solve() -> Result<(usize, usize)> {
    let part1 = solve_part1()?;
    let part2 = solve_part2()?;
    Ok((part1, part2))
}

pub fn solve_part1() -> Result<usize> {
    let contents = fs::read_to_string("src/day01/input.txt").expect("boom");
    let lines_first = contents.lines();
    let lines_second = contents.clone();

    for first in lines_first {
        for second in lines_second.lines() {
            let first_operand: usize = first.parse().unwrap();
            let second_operand: usize = second.parse().unwrap();
            if (first_operand + second_operand) == 2020 {
                return Ok(first_operand * second_operand);
            }
        }
    }

    panic!("Could not solve day 01 - part 1.");
}

pub fn solve_part2() -> Result<usize> {
    let contents = fs::read_to_string("src/day01/input.txt").expect("boom");
    let lines_first = contents.lines();
    let lines_second = contents.clone();
    let lines_third = contents.clone();

    for first in lines_first {
        for second in lines_second.lines() {
            for third in lines_third.lines() {
                let first_operand: usize = first.parse().unwrap();
                let second_operand: usize = second.parse().unwrap();
                let third_operand: usize = third.parse().unwrap();
                if (first_operand + second_operand + third_operand) == 2020 {
                    return Ok(first_operand * second_operand * third_operand);
                }
            }
        }
    }

    panic!("Could not solve day 01 - part 2.")
}
