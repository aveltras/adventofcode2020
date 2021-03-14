use std::fs;

pub fn solve() -> (usize, usize) {
    let input = fs::read_to_string("src/day09/input.txt").unwrap();
    let lines = input.lines();

    let window = 25;
    let numbers: Vec<usize> = lines.map(|x| x.parse::<usize>().unwrap()).collect();

    let mut invalid_number: usize = numbers[window];

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
            invalid_number = numbers[i];
            break;
        }
    }

    let length = numbers.len();
    let mut contiguous: Option<Vec<usize>> = None;

    'outer: for i in 0..length {
        let mut sum = 0;
        for j in i..length {
            sum += numbers[j];
            if sum > invalid_number {
                break;
            } else if sum == invalid_number {
                contiguous = Some((i..=j).map(|x| numbers[x]).collect());
                break 'outer;
            } else {
                continue;
            }
        }
    }

    let mut contiguous = contiguous.unwrap();
    contiguous.sort();

    let largest = &contiguous[contiguous.len() - 1];
    let smallest = &contiguous[0];

    (invalid_number, smallest + largest)
}
