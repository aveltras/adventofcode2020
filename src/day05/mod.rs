use std::{fs, ops::Range};

pub fn solve() -> (usize, usize) {
    let input = fs::read_to_string("src/day05/input.txt").unwrap();
    let lines = input.lines();

    let max_seat_id = lines.map(compute_seat_id).max().unwrap();

    (max_seat_id, 1)
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
