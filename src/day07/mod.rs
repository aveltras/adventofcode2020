use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn solve() -> (usize, usize) {
    let input = fs::read_to_string("src/day07/input.txt").unwrap();
    let lines = input.lines();

    let mut color_map: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut reverse_color_map: HashMap<String, HashSet<String>> = HashMap::new();

    for line in lines {
        let line = line.replace(" bags", "").replace(" bag", "");
        let mut parts = line.split(" contain ");
        let container = parts.next().unwrap();
        let mut content = parts.next().unwrap().to_string();
        content.pop();
        let content: Vec<&str> = content.split(", ").collect();
        let mut subcolor_map: HashMap<String, usize> = HashMap::new();

        for item in content {
            let parts = item.splitn(2, |c| c == ' ').collect::<Vec<&str>>();
            match parts[0] {
                "no" => continue,
                count => {
                    subcolor_map.insert(parts[1].to_owned(), count.parse::<usize>().unwrap());
                    let color = parts[1].to_owned();
                    let colors = reverse_color_map.entry(color).or_insert(HashSet::new());
                    (*colors).insert(container.to_owned());
                }
            };
        }

        color_map.insert(container.to_owned(), subcolor_map);
    }

    let possible_colors: HashSet<String> =
        find_container("shiny gold".to_string(), &reverse_color_map);

    let total_bags = find_contained("shiny gold".to_string(), &color_map) - 1; // removing the top container

    (possible_colors.len(), total_bags)
}

fn find_contained(key: String, map: &HashMap<String, HashMap<String, usize>>) -> usize {
    let items = map.get(&key).unwrap();
    let mut total = 1;

    for (k, v) in items {
        let subtotal = find_contained(k.to_owned(), map);
        total += v * subtotal;
    }

    total
}

fn find_container(key: String, map: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    if let Some(keys) = map.get(&key) {
        let mut current_keys = keys.to_owned();
        for k in keys {
            let child_keys = find_container(k.to_owned(), map);
            current_keys = current_keys.union(&child_keys).cloned().collect();
        }

        current_keys
    } else {
        HashSet::new()
    }
}
