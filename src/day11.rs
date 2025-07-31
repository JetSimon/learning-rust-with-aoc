use std::{collections::HashMap, fs};

pub fn dp(
    n: i64,
    blinks: i64,
    stones_made: i64,
    next_lookup: &mut HashMap<(i64, i64), i64>,
) -> i64 {
    let key = (n, blinks);
    if next_lookup.contains_key(&key) {
        return *next_lookup.get(&key).unwrap();
    }

    let stone_string = n.to_string();
    let mut new_stones_made = stones_made;

    if blinks > 0 {
        if n == 0 {
            new_stones_made = dp(1, blinks - 1, stones_made, next_lookup);
        } else if stone_string.len() % 2 == 0 {
            let (l, r) = stone_string.split_at(stone_string.len() / 2);
            let ln = l.parse::<i64>().unwrap();
            let rn = r.parse::<i64>().unwrap();
            new_stones_made = dp(ln, blinks - 1, stones_made, next_lookup)
                + dp(rn, blinks - 1, stones_made, next_lookup);
        } else {
            new_stones_made = dp(n * 2024, blinks - 1, stones_made, next_lookup);
        }
    }

    next_lookup.insert(key, new_stones_made);
    return new_stones_made;
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

    let stones: Vec<i64> = res
        .split(" ")
        .filter(|c| *c != "")
        .map(|c| c.parse::<i64>().unwrap())
        .collect();

    let blinks = 75;

    let mut total_stones: i64 = 0;
    let mut lookup = HashMap::new();

    for stone in stones {
        total_stones += dp(stone, blinks, 1, &mut lookup);
    }

    println!("After {} there are {} stones", blinks, total_stones);
}
