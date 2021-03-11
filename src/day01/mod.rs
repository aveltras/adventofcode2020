use std::fs;

pub fn solve() -> Result<usize, &'static str> {
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

    Err("Could not solve day 01.")
}
