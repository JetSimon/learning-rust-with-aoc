use std::fs;

fn search(
    grid: &Vec<Vec<&str>>,
    pos: (i32, i32),
    target: &str,
    found: &str,
    size: (i32, i32),
    velocity: (i32, i32),
) -> bool {
    let (x, y) = pos;
    let (w, h) = size;

    if found == target {
        return true;
    }

    if x < 0 || x >= w || y < 0 || y >= h {
        return false;
    }

    let (dx, dy) = velocity;

    let current_tile = grid[y as usize][x as usize];
    let new_found = found.to_owned() + current_tile;

    return search(&grid, (x + dx, y + dy), target, &new_found, size, velocity);
}

fn is_mas(subset: Vec<Vec<&str>>) -> bool {
    if subset.len() != 3 || subset[0].len() != 3 {
        return false;
    }

    let left = subset[0][0].to_owned() + subset[1][1] + subset[2][2];
    let right = subset[0][2].to_owned() + subset[1][1] + subset[2][0];

    return (left == "MAS" || left == "SAM") && (right == "MAS" || right == "SAM");
}

pub fn run(path: String) {
    let contents = fs::read_to_string(path);

    let res = if let Ok(res) = contents {
        res
    } else {
        println!("Could not open input file");
        return;
    };

    let target = "XMAS";
    let grid: Vec<Vec<&str>> = res
        .lines()
        .map(|line| line.split("").filter(|c| *c != "").collect())
        .collect();

    let w = grid.get(0).unwrap().len() as i32;
    let h = grid.len() as i32;

    let mut times_found_1 = 0;
    let mut times_found_2 = 0;

    for y in 0..h {
        for x in 0..w {
            for dy in [-1, 0, 1] {
                for dx in [-1, 0, 1] {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let found = "";
                    if search(&grid, (x, y), target, &found, (w, h), (dx, dy)) {
                        times_found_1 += 1;
                    }
                }
            }

            let (start_row, num_rows) = (y as usize, 3);
            let (start_col, num_cols) = (x as usize, 3);

            let subset: Vec<Vec<&str>> = grid
                .iter()
                .skip(start_row)
                .take(num_rows)
                .map(|row| row.iter().skip(start_col).take(num_cols).cloned().collect())
                .collect();

            if is_mas(subset) {
                times_found_2 += 1;
            }
        }
    }

    println!("Day 4 Part 1: {times_found_1}");
    println!("Day 4 Part 1: {times_found_2}");
}
