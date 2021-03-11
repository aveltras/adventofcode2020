mod day01;
mod day02;

fn main() {
    let (result_part1, result_part2) = day01::solve().expect("day 01 error");
    println!("Day 1 - part 1 answer is {}", result_part1);
    println!("Day 1 - part 2 answer is {}", result_part2);

    let result_part1 = day02::solve().expect("day02 error");
    println!("Day 2 - part 1 answer is {}", result_part1);
}
