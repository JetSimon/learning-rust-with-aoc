use regex::Regex;
use std::fs;

enum Instruction {
    Do,
    Dont,
    Mult,
}

fn extract_mul_expressions(input: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut results = vec![];

    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        results.push((a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()));
    }

    return results;
}

fn extract_instructions(input: &str) -> Vec<(Instruction, i32, i32)> {
    let re = Regex::new(r"(mul)\(\d+,\d+\)|(do)\(\)|(don't)\(\)").unwrap();
    let mut instructions = vec![];

    for (m, [t]) in re.captures_iter(input).map(|c| c.extract()) {
        match t {
            "do" => instructions.push((Instruction::Do, -1, -1)),
            "don't" => instructions.push((Instruction::Dont, -1, -1)),
            "mul" => {
                let extracted = extract_mul_expressions(m);
                let (a, b) = extracted.get(0).unwrap();
                instructions.push((Instruction::Mult, *a, *b));
            }
            _ => panic!("Bad type!"),
        }
    }

    return instructions;
}

pub fn run(path: String) {
    let contents = fs::read_to_string(path);

    let res = if let Ok(res) = contents {
        res
    } else {
        println!("Could not open input file");
        return;
    };

    let exp = extract_mul_expressions(&res);

    let mut total_1 = 0;

    for (a, b) in exp {
        total_1 += a * b;
    }

    let mut total_2 = 0;
    let mut should_mul = true;
    let inst = extract_instructions(&res);

    for i in inst {
        match i {
            (Instruction::Do, _, _) => should_mul = true,
            (Instruction::Dont, _, _) => should_mul = false,
            (Instruction::Mult, a, b) => {
                if should_mul {
                    total_2 += a * b;
                }
            }
        }
    }

    println!("Day 3 Part 1: {total_1}");
    println!("Day 3 Part 2: {total_2}");
}
