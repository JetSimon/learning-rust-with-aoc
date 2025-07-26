use std::fs;

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

    let mut valids = 0;
    for report in reports {
        let mut asc = report.clone();
        asc.sort();
        let des: Vec<i32> = asc.iter().copied().rev().collect();

        if !report.eq(&asc) && !report.eq(&des) {
            continue;
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

        //let srep: String = report.iter().map(|&id| id.to_string() + ",").collect();
        //println!("{srep} {is_valid}");

        if is_valid {
            valids += 1;
        }
    }

    println!("Day 2 Part 1: {valids}");
}
