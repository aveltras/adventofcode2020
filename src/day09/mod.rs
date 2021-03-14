use std::fs;

pub fn solve() -> (usize, usize) {
    let input = fs::read_to_string("src/day09/input.txt").unwrap();
    let lines = input.lines();

    let window = 25;
    let numbers: Vec<usize> = lines.map(|x| x.parse::<usize>().unwrap()).collect();

    let mut first_number: usize = numbers[window];

    for i in window..numbers.len() {
        let mut found = false;

        'outer: for j in (i - window)..i {
            for l in (j + 1)..i {
                if numbers[j] + numbers[l] == numbers[i] {
                    found = true;
                    break 'outer;
                }
            }
        }

        if !found {
            first_number = numbers[i];
            break;
        }
    }

    (first_number, 1)
}
