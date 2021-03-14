use std::{collections::HashSet, fs, str::FromStr};

pub fn solve() -> (usize, usize) {
    let input = fs::read_to_string("src/day08/input.txt").unwrap();
    let lines = input.lines();

    let instructions: Vec<Instruction> = lines.map(|l| l.parse::<Instruction>().unwrap()).collect();

    let mut processed_instructions: HashSet<usize> = HashSet::new();

    let mut pointer = 0;
    let mut acc = 0;

    loop {
        let instr = &instructions[pointer];

        if processed_instructions.contains(&pointer) {
            println!("Instruction {:?} already processed.", instr);
            println!("Aborting.");
            break;
        }

        println!("Processing.. {:?}", instr);
        processed_instructions.insert(pointer);

        match instr {
            Instruction::NoOp(_) => pointer += 1,
            Instruction::Acc(x) => {
                pointer += 1;
                acc += x;
            }
            Instruction::Jump(x) => {
                pointer = (pointer as i32 + x) as usize;
            }
        }

        println!("Pointer is now.. {:?}", pointer);
    }

    (acc as usize, 1)
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Instruction {
    Acc(i32),
    Jump(i32),
    NoOp(i32),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = &s[0..3];
        let sign = &s[4..5];
        let arg = &s[5..].parse::<i32>().unwrap();

        let parsed_arg = if let "-" = sign { *arg * -1 } else { *arg };

        match op {
            "acc" => Ok(Instruction::Acc(parsed_arg)),
            "jmp" => Ok(Instruction::Jump(parsed_arg)),
            "nop" => Ok(Instruction::NoOp(parsed_arg)),
            _ => Err("Could not parse instruction"),
        }
    }
}
