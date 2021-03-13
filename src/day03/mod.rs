use std::fs;
use std::str::FromStr;

pub fn solve() -> Result<(usize, usize), &'static str> {
    let part1 = solve_part1()?;
    let part2 = solve_part2()?;
    Ok((part1, part2))
}

pub fn solve_part1() -> Result<usize, &'static str> {
    let contents = fs::read_to_string("src/day03/input.txt").unwrap();
    let tree_area: TreeArea = contents.parse().unwrap();
    println!("{:?}", tree_area.locations[0]);
    let mut collisions = 0;
    let mut pos_x = 0;
    let mut pos_y = 0;
    let width = tree_area.locations[0].len();

    while let Some(row) = tree_area.locations.get(pos_y) {
        if row[pos_x % width] {
            collisions += 1;
        }
        pos_y += 1;
        pos_x += 3;
    }

    Ok(collisions)
}

pub fn solve_part2() -> Result<usize, &'static str> {
    Ok(2)
}

#[derive(Debug)]
struct TreeArea {
    locations: Vec<Vec<bool>>,
}

impl FromStr for TreeArea {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut locations = Vec::new();
        for line in s.lines() {
            let mut v = Vec::new();
            for char in line.chars() {
                v.push(char == '#');
            }
            locations.push(v);
        }

        Ok(TreeArea { locations })
    }
}
