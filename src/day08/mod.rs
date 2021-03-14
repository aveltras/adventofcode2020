use std::{collections::HashSet, fs, str::FromStr};

pub fn solve() -> (usize, usize) {
    let input = fs::read_to_string("src/day08/input.txt").unwrap();
    let lines = input.lines();

    let mut instructions: Vec<Instruction> =
        lines.map(|l| l.parse::<Instruction>().unwrap()).collect();
    let (pointer, acc) = test(&instructions).err().unwrap();
    let mut pointer = 0;
    let mut final_acc = 0;

    for pointer in 0..instructions.len() {
        match &instructions[pointer] {
            Instruction::Acc(_) => continue,
            instr => {
                let old_instruction = instructions[pointer].clone();
                let new_instruction = match instr {
                    Instruction::NoOp(x) => Instruction::Jump(*x),
                    Instruction::Jump(x) => Instruction::NoOp(*x),
                    _ => panic!(),
                };

                instructions[pointer] = new_instruction;

                match test(&instructions) {
                    Err((p, _)) => {
                        instructions[pointer] = old_instruction;
                        continue;
                    }
                    Ok(acc_ok) => {
                        final_acc = acc_ok;
                        break;
                    }
                }
            }
        }
    }

    (acc as usize, final_acc as usize)
}

fn test(instructions: &Vec<Instruction>) -> Result<i32, (usize, i32)> {
    let mut processed_instructions: HashSet<usize> = HashSet::new();
    let mut pointer = 0;
    let mut acc = 0;

    while let Some(instr) = instructions.get(pointer) {
        if processed_instructions.contains(&pointer) {
            return Err((pointer, acc));
        }

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
    }

    Ok(acc)
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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
