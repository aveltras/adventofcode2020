use regex;
use std::{borrow::Borrow, str::FromStr};
use std::{fs, io};

pub fn solve() -> (usize, usize) {
    let input = fs::read_to_string("src/day04/input.txt").unwrap();
    let lines = input.split("\n\n");
    let mut passports = 0;
    let mut valid_passports = 0;

    for line in lines {
        if let Ok(p) = line.parse::<Passport>() {
            passports += 1;
            match validate_passport(p) {
                Ok(_) => valid_passports += 1,
                Err(err) => {
                    // println!("{}", line);
                    // println!("{}", err);
                    // println!();
                }
            }
        }
    }

    (passports, valid_passports)
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

struct ValidPassport(Passport);

struct BirthYear(u32);

impl FromStr for BirthYear {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(year) = s.parse() {
            if (1920..=2002).contains(&year) {
                return Ok(BirthYear(year));
            }
        }

        Err("Could not parse birth year")
    }
}

struct IssueYear(u32);

impl FromStr for IssueYear {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(year) = s.parse() {
            if (2010..=2020).contains(&year) {
                return Ok(IssueYear(year));
            }
        }

        Err("Could not parse issue year")
    }
}

struct ExpirationYear(u32);

impl FromStr for ExpirationYear {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(year) = s.parse() {
            if (2020..=2030).contains(&year) {
                return Ok(ExpirationYear(year));
            }
        }

        Err("Could not parse expiration year")
    }
}

enum Height {
    Centimeters(u32),
    Inches(u32),
}

impl FromStr for Height {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut height = String::new();
        let mut unit = String::new();
        let mut target = &mut height;
        for char in s.chars() {
            if !char.is_digit(10) {
                target = &mut unit;
            }

            target.push(char);
        }

        if let Ok(height) = height.parse::<u32>() {
            match unit.borrow() {
                "cm" if (150..=193).contains(&height) => return Ok(Height::Centimeters(height)),
                "in" if (59..=76).contains(&height) => return Ok(Height::Inches(height)),
                _ => {}
            }
        }

        Err("Could not parse height")
    }
}

struct HairColor(u8, u8, u8);

impl FromStr for HairColor {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"^\#([a-z0-9]{2})([a-z0-9]{2})([a-z0-9]{2})$").unwrap();
        if let Some(caps) = re.captures(s) {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(caps.get(1).unwrap().as_str(), 16),
                u8::from_str_radix(caps.get(2).unwrap().as_str(), 16),
                u8::from_str_radix(caps.get(3).unwrap().as_str(), 16),
            ) {
                return Ok(HairColor(r, g, b));
            }
        }

        Err("Could not parse hair color")
    }
}

enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazelnut,
    Other,
}

impl FromStr for EyeColor {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" => Ok(EyeColor::Amber),
            "blu" => Ok(EyeColor::Blue),
            "brn" => Ok(EyeColor::Brown),
            "gry" => Ok(EyeColor::Gray),
            "grn" => Ok(EyeColor::Green),
            "hzl" => Ok(EyeColor::Hazelnut),
            "oth" => Ok(EyeColor::Other),
            _ => Err("Could not parse eye color"),
        }
    }
}

struct PassportId(String);

impl FromStr for PassportId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"^\d{9}$").unwrap();
        if re.is_match(s) {
            Ok(PassportId(s.to_owned()))
        } else {
            Err("Could not parse passport id")
        }
    }
}

fn validate_passport(passport: Passport) -> Result<ValidPassport, &'static str> {
    let birth_year = passport.birth_year.parse::<BirthYear>()?;
    let issue_year = passport.issue_year.parse::<IssueYear>()?;
    let exp_year = passport.expiration_year.parse::<ExpirationYear>()?;
    let height = passport.height.parse::<Height>()?;
    let hair_color = passport.hair_color.parse::<HairColor>()?;
    let eye_color = passport.eye_color.parse::<EyeColor>()?;
    let passport_id = passport.passport_id.parse::<PassportId>()?;

    Ok(ValidPassport(passport))
}
