use std::fs;
use std::str::FromStr;

pub fn solve() -> Result<(usize, usize), &'static str> {
    let contents = fs::read_to_string("src/day03/input.txt").unwrap();
    let tree_area: TreeArea = contents.parse().unwrap();

    let part1 = solve_part1(&tree_area)?;
    let part2 = solve_part2(&tree_area)?;

    Ok((part1, part2))
}

fn solve_part1(tree_area: &TreeArea) -> Result<usize, &'static str> {
    let collisions = collisions_per_slope(tree_area, &(3, 1));
    Ok(collisions)
}

fn solve_part2(tree_area: &TreeArea) -> Result<usize, &'static str> {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    Ok(slopes
        .iter()
        .map(|slope| collisions_per_slope(tree_area, slope))
        .product())
}

fn collisions_per_slope(tree_area: &TreeArea, (x, y): &(usize, usize)) -> usize {
    let mut collisions = 0;
    let mut pos_x = 0;
    let mut pos_y = 0;
    let width = tree_area.locations[0].len();

    while let Some(row) = tree_area.locations.get(pos_y) {
        if row[pos_x % width] {
            collisions += 1;
        }
        pos_x += x;
        pos_y += y;
    }

    collisions
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