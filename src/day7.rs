use regex::Regex;
use std::fs;
use tqdm::tqdm;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Operation {
    Add,
    Mul,
    Concat,
}

type Equation = (i64, Vec<i64>);

fn apply_operation(acc: i64, value: i64, operation: Operation) -> i64 {
    match operation {
        Operation::Add => return acc + value,
        Operation::Mul => return acc * value,
        Operation::Concat => return (acc.to_string() + &value.to_string()).parse().unwrap(),
    }
}

fn get_solutions(
    equation: &Equation,
    acc: i64,
    operations: &Vec<Operation>,
    solutions: &mut Vec<Vec<Operation>>,
    possible_operations: &Vec<Operation>,
) {
    let (target, values) = equation;

    if values.is_empty() {
        if acc == *target {
            //println!("{acc} == {target}");
            solutions.push(operations.to_vec());
        }
        return;
    }

    for operation in possible_operations {
        let mut new_values = values.clone();
        let value = new_values.remove(0);
        let new_acc = apply_operation(acc, value, *operation);
        let mut new_operations = operations.clone();
        new_operations.push(*operation);
        get_solutions(
            &(*target, new_values),
            new_acc,
            &new_operations,
            solutions,
            possible_operations,
        );
    }
}

pub fn run(path: String) {
    // --snip--
    println!("In file {path}");

    let contents = fs::read_to_string(path);

    let res = if let Ok(res) = contents {
        res
    } else {
        let error = contents.err();
        panic!("Problem opening the file: {error:?}");
    };

    let re = Regex::new(r"(\d+): (.+)").unwrap();
    let mut equations: Vec<Equation> = vec![];

    for line in res.lines() {
        for (_, [target, values]) in re.captures_iter(line).map(|c| c.extract()) {
            let values_vec: Vec<i64> = values
                .split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            equations.push((target.parse::<i64>().unwrap(), values_vec));
        }
    }

    let mut part_1_total = 0;

    for (target, mut values) in tqdm(equations.clone()) {
        //let values_copy = values.clone();
        let acc = values.remove(0);
        let mut solutions = vec![];
        let operations = vec![];

        let part_1_operations = vec![Operation::Add, Operation::Mul];
        get_solutions(
            &(target, values),
            acc,
            &operations,
            &mut solutions,
            &part_1_operations,
        );

        //println!("{} : ({:?}) => {:?}", target, values_copy, solutions);

        if solutions.len() > 0 {
            part_1_total += target;
        }
    }

    println!("Day 7 Part 1: {part_1_total}");

    let mut part_2_total = 0;

    for (target, mut values) in tqdm(equations.clone()) {
        //let values_copy = values.clone();
        let acc = values.remove(0);
        let mut solutions = vec![];
        let operations = vec![];

        let part_2_operations = vec![Operation::Add, Operation::Mul, Operation::Concat];
        get_solutions(
            &(target, values),
            acc,
            &operations,
            &mut solutions,
            &part_2_operations,
        );

        //println!("{} : ({:?}) => {:?}", target, values_copy, solutions);

        if solutions.len() > 0 {
            part_2_total += target;
        }
    }

    println!("Day 7 Part 2: {part_2_total}");
}
