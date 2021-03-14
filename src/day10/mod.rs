use std::{collections::HashMap, fs};

pub fn solve() -> (usize, usize) {
    let input = fs::read_to_string("src/day10/input.txt").unwrap();
    let lines = input.lines();

    let mut adapters: Vec<usize> = lines.map(|x| x.parse::<usize>().unwrap()).collect();
    adapters.sort();
    adapters.push(adapters[adapters.len() - 1] + 3);
    println!("{:?}", adapters);

    let mut jumps = HashMap::<usize, usize>::new();
    let mut previous = 0;

    for joltage in adapters {
        let diff = joltage - previous;
        *jumps.entry(diff).or_insert(0) += 1;
        previous = joltage;
    }

    println!("{:?}", jumps);

    (jumps.get(&1).unwrap() * jumps.get(&3).unwrap(), 1)
}
