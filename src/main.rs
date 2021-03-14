use std::io;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() {
    let results = vec![
        // day01::solve(),
        // day02::solve(),
        // day03::solve(),
        // day04::solve(),
        // day05::solve(),
        // day06::solve(),
        // day07::solve(),
        // day08::solve(),
        // day09::solve(),
        day10::solve(),
    ];

    for (idx, (part1, part2)) in results.iter().enumerate() {
        println!("Day {} - part 1 answer is {}", idx + 1, part1);
        println!("Day {} - part 2 answer is {}", idx + 1, part2);
    }
}
