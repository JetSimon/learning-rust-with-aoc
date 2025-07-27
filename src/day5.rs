use std::{
    collections::{HashMap, HashSet},
    fs,
};

use regex::Regex;

fn get_middle_number(update: &Vec<i32>) -> i32 {
    return update[update.len().midpoint(0)];
}

fn insert_before_first_occurance(n: i32, update: &mut Vec<i32>, prereqs: &HashSet<i32>) {
    let first_occurances: Vec<usize> = prereqs
        .iter()
        .map(|a| update.iter().position(|b| *b == *a))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
    let smallest_index = first_occurances.iter().min();

    match smallest_index {
        Some(index) => update.insert(*index, n),
        None => (),
    }
}

fn correct_update(update: Vec<i32>, must_come_before: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut new_update = update.clone();

    for n in update {
        let prereqs = must_come_before.get(&n);
        match prereqs {
            Some(set) => {
                new_update.remove(new_update.iter().position(|x| *x == n).unwrap());
                insert_before_first_occurance(n, &mut new_update, &set);
            }
            None => println!("No rules for {n}"),
        }
    }

    return new_update;
}

fn in_correct_order(update: &Vec<i32>, must_come_before: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut already_printed = HashSet::new();
    let all_values: HashSet<i32> = HashSet::from_iter(update.iter().cloned());
    for n in update {
        already_printed.insert(n);
        let prereqs = must_come_before.get(&n);
        match prereqs {
            Some(set) => {
                for m in set {
                    if all_values.contains(m) && !already_printed.contains(m) {
                        return false;
                    }
                }
            }
            None => println!("No rules for {n}"),
        }
    }
    return true;
}

pub fn run(path: String) {
    let contents = fs::read_to_string(path);

    let res = if let Ok(res) = contents {
        res
    } else {
        println!("Could not open input file");
        return;
    };

    let mut must_come_before: HashMap<i32, HashSet<i32>> = HashMap::new(); // X|Y -> X must come before Y
    let mut updates: Vec<Vec<i32>> = vec![];

    let mut adding_updates = false;

    for line in res.lines() {
        if adding_updates {
            updates.push(line.split(",").map(|n| n.parse::<i32>().unwrap()).collect());
        } else {
            if line.trim() == "" {
                adding_updates = true;
                continue;
            }

            let re = Regex::new(r"(\d+)\|(\d+)").unwrap();
            let (_, [x, y]) = re.captures_iter(line).map(|c| c.extract()).next().unwrap();

            let x_n = x.parse::<i32>().unwrap();
            let y_n = y.parse::<i32>().unwrap();
            if !must_come_before.contains_key(&y_n) {
                must_come_before.insert(y_n, HashSet::new());
            }
            must_come_before.get_mut(&y_n).unwrap().insert(x_n);
        }
    }

    let mut total = 0;
    for update in updates {
        if in_correct_order(&update, &must_come_before) {
            total += get_middle_number(&update);
        } else {
            let corrected = correct_update(update, &must_come_before);
            println!("corrected to {:?}", corrected);
        }
    }

    println!("Day 5 Part 1: {total}");
}
