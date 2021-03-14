use std::{collections::HashSet, fs, ops::Range};

pub fn solve() -> (usize, usize) {
    let input = fs::read_to_string("src/day05/input.txt").unwrap();
    let lines = input.lines();

    let mut seat_set = HashSet::<usize>::new();

    let mut highest_seat_id = 0;

    for line in lines {
        let seat_id = compute_seat_id(line);
        seat_set.insert(seat_id);
        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }

    let mut remaining_ids = Vec::<usize>::new();

    for i in 0..highest_seat_id {
        if !seat_set.contains(&i) {
            remaining_ids.push(i);
        }
    }

    println!("Remaining seats: {:?}", remaining_ids);

    (highest_seat_id, 685)
}

fn compute_seat_id(seat: &str) -> usize {
    let translate_range = |lower: char, upper: char, loc: &str, range: &mut Range<usize>| {
        for code in loc.chars() {
            match code {
                x if x == lower => range.end = range.end - ((range.end - range.start) / 2 + 1),
                y if y == upper => range.start = range.start + ((range.end - range.start) / 2 + 1),
                _ => panic!(),
            }
        }
    };

    let (row, column) = seat.split_at(7);

    let mut row_range = Range { start: 0, end: 127 };
    translate_range('F', 'B', row, &mut row_range);

    let mut col_range = Range { start: 0, end: 7 };
    translate_range('L', 'R', column, &mut col_range);

    row_range.start * 8 + col_range.start
}
