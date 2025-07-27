use std::{
    collections::{HashMap, HashSet},
    fs, vec,
};

use regex::Regex;

fn get_middle_number(update: &Vec<i32>) -> i32 {
    return update[update.len().midpoint(0)];
}

fn get_valid_update(
    original_update: &Vec<i32>,
    must_come_before: &HashMap<i32, HashSet<i32>>,
) -> Vec<i32> {
    let mut pool: Vec<i32> = original_update.clone();
    let all_values: HashSet<i32> = HashSet::from_iter(original_update.iter().cloned());
    let mut new_update = vec![];

    while pool.len() > 0 {
        for i in 0..pool.len() {
            let n = pool[i];
            let mut candidate = new_update.clone();
            candidate.push(n);
            let can_insert = in_correct_order(&candidate, must_come_before, &all_values);

            if can_insert {
                new_update.push(n);
                pool.remove(i);
                break;
            }
        }
    }

    return new_update;
}

fn in_correct_order(
    update: &Vec<i32>,
    must_come_before: &HashMap<i32, HashSet<i32>>,
    all_values: &HashSet<i32>,
) -> bool {
    let mut already_printed = HashSet::new();
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
            None => (), // println!("No rules for {n}"),
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
    let mut total_2 = 0;
    for update in updates {
        let all_values: HashSet<i32> = HashSet::from_iter(update.iter().cloned());
        if in_correct_order(&update, &must_come_before, &all_values) {
            total += get_middle_number(&update);
        } else {
            let corrected = get_valid_update(&update, &must_come_before);
            total_2 += get_middle_number(&corrected);
            //println!("{:?} corrected to {:?}", update, corrected);
        }
    }

    println!("Day 5 Part 1: {total}");
    println!("Day 5 Part 2: {total_2}");
}
