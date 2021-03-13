use std::io;

use io::Error;

mod day01;
mod day02;
mod day03;

fn main() -> Result<(), Error> {
    let (result_part1, result_part2) = day01::solve()?;
    println!("Day 1 - part 1 answer is {}", result_part1);
    println!("Day 1 - part 2 answer is {}", result_part2);

    let (result_part1, result_part2) = day02::solve()?;
    println!("Day 2 - part 1 answer is {}", result_part1);
    println!("Day 2 - part 2 answer is {}", result_part2);

    let (result_part1, result_part2) = day03::solve()?;
    println!("Day 3 - part 1 answer is {}", result_part1);
    println!("Day 3 - part 2 answer is {}", result_part2);

    Ok(())
}
