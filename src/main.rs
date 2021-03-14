use std::io;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    let results = vec![
        // day01::solve(),
        // day02::solve(),
        // day03::solve(),
        // day04::solve(),
        day05::solve(),
    ];

    for (idx, (part1, part2)) in results.iter().enumerate() {
        println!("Day {} - part 1 answer is {}", idx + 1, part1);
        println!("Day {} - part 2 answer is {}", idx + 1, part2);
    }
}
