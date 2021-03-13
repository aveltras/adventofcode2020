use std::str::FromStr;
use std::{fs, io};

pub fn solve() -> (usize, usize) {
    let input = fs::read_to_string("src/day04/input.txt").unwrap();
    let lines = input.split("\n\n");
    let mut valid_passports = 0;

    for line in lines {
        match line.parse::<Passport>() {
            Ok(_) => valid_passports += 1,
            _ => continue,
        }
    }

    (valid_passports, 2)
}

struct Passport {
    birth_year: String,
    issue_year: String,
    expiration_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: Option<String>,
}

impl FromStr for Passport {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace();

        let mut birth_year = String::new();
        let mut issue_year = String::new();
        let mut expiration_year = String::new();
        let mut height = String::new();
        let mut hair_color = String::new();
        let mut eye_color = String::new();
        let mut passport_id = String::new();
        let mut country_id = None;

        for part in parts {
            let mut field = part.splitn(2, ':');

            match field.next().unwrap() {
                "byr" => birth_year = field.next().unwrap().to_owned(),
                "iyr" => issue_year = field.next().unwrap().to_owned(),
                "eyr" => expiration_year = field.next().unwrap().to_owned(),
                "hgt" => height = field.next().unwrap().to_owned(),
                "hcl" => hair_color = field.next().unwrap().to_owned(),
                "ecl" => eye_color = field.next().unwrap().to_owned(),
                "pid" => passport_id = field.next().unwrap().to_owned(),
                "cid" => country_id = Some(field.next().unwrap().to_owned()),
                _ => panic!(),
            };
        }

        if vec![
            &birth_year,
            &issue_year,
            &expiration_year,
            &height,
            &hair_color,
            &eye_color,
            &passport_id,
        ]
        .iter()
        .any(|&field| *field == String::new())
        {
            Err("could not parse passport data")
        } else {
            Ok(Passport {
                birth_year,
                issue_year,
                expiration_year,
                height,
                hair_color,
                eye_color,
                passport_id,
                country_id,
            })
        }
    }
}
