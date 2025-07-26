use std::fs;

fn validate_report(report: &Vec<i32>) -> bool {
    let mut asc = report.clone();
    asc.sort();
    let des: Vec<i32> = asc.iter().copied().rev().collect();

    if !report.eq(&asc) && !report.eq(&des) {
        return false;
    }

    let mut is_valid = true;

    for i in 0..report.len() {
        let prev = if i > 0 { report.get(i - 1) } else { None };
        let current = report.get(i);
        let next = report.get(i + 1);

        match (prev, current) {
            (Some(a), Some(b)) => is_valid &= ((a - b).abs() >= 1) && ((a - b).abs() <= 3),
            _ => (),
        }

        match (current, next) {
            (Some(a), Some(b)) => is_valid &= ((a - b).abs() >= 1) && ((a - b).abs() <= 3),
            _ => (),
        }

        if !is_valid {
            break;
        }
    }

    return is_valid;
}

fn remove_and_validate_report(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let mut subset = report.clone();
        subset.remove(i);
        if validate_report(&subset) {
            return true;
        }
    }

    return false;
}

pub fn run(path: String) {
    let contents = fs::read_to_string(path);

    let res = if let Ok(res) = contents {
        res
    } else {
        println!("Could not open input file");
        return;
    };

    let reports: Vec<Vec<i32>> = res
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut day_1 = 0;
    let mut day_2 = 0;
    for report in reports {
        if validate_report(&report) {
            day_1 += 1;
            day_2 += 1;
        } else if remove_and_validate_report(&report) {
            day_2 += 1;
        }
    }

    println!("Day 1 Part 1: {day_1}");
    println!("Day 1 Part 2: {day_2}");
}
