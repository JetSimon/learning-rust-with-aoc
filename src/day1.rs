use std::{collections::HashMap, fs};

fn parse_input(line: &str) -> (i32, i32) {
    let entry: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let first = entry.get(0).unwrap();
    let last = entry.last().unwrap();
    return (*first, *last);
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

    let entries: Vec<(i32, i32)> = res.as_str().lines().map(|line| parse_input(line)).collect();
    let mut left = vec![];
    let mut right = vec![];

    let mut counts = HashMap::new();

    for (a, b) in entries {
        //println!("{a} {b}");
        left.push(a);
        right.push(b);

        counts.entry(b).and_modify(|x| *x += 1).or_insert(1);
    }

    left.sort();
    right.sort();

    let mut total_distance = 0;
    let mut total_distance_2 = 0;

    for n in 0..left.len() {
        let a = left.get(n).unwrap();
        let b = right.get(n).unwrap();
        total_distance += (a - b).abs();

        let mult = *counts.entry(*a).or_insert(0);
        let code = a * mult;

        //println!("{a} * {mult} = {code}");

        total_distance_2 += code;
    }

    println!("Part A: Total distance is {total_distance}");
    println!("Part B: Total distance is {total_distance_2}");
}
