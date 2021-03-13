mod day01;
mod day02;
mod day03;

fn main() {
    let (result_part1, result_part2) = day01::solve().expect("day 01 error");
    println!("Day 1 - part 1 answer is {}", result_part1);
    println!("Day 1 - part 2 answer is {}", result_part2);

    let (result_part1, result_part2) = day02::solve().expect("day02 error");
    println!("Day 2 - part 1 answer is {}", result_part1);
    println!("Day 2 - part 2 answer is {}", result_part2);

    let (result_part1, result_part2) = day03::solve().expect("day03 error");
    println!("Day 3 - part 1 answer is {}", result_part1);
    println!("Day 3 - part 2 answer is {}", result_part2);
}
