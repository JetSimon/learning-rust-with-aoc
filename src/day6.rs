use std::{collections::HashSet, fs};
use tqdm::tqdm;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Floor,
    Wall,
}

/*
fn get_guard_symbol(guard_dx: i32, guard_dy: i32) -> String {
    match (guard_dx, guard_dy) {
        (0, -1) => return "^".to_string(),
        (1, 0) => return ">".to_string(),
        (0, 1) => return "<".to_string(),
        (-1, 0) => return "v".to_string(),
        _ => "?".to_string(),
    }
}

fn print_grid(grid: &Vec<Vec<Tile>>, guard_x: i32, guard_y: i32, guard_dx: i32, guard_dy: i32) {
    println!("----");
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if (x as i32, y as i32) == (guard_x, guard_y) {
                print!("{}", get_guard_symbol(guard_dx, guard_dy));
            } else {
                if grid[y][x] == Tile::Floor {
                    print!(".");
                } else if grid[y][x] == Tile::Wall {
                    print!("#");
                }
            }
        }
        print!("\n");
    }

    print!("\n");
}
*/

fn is_in_bounds(grid: &Vec<Vec<Tile>>, x: i32, y: i32) -> bool {
    return x >= 0
        && y >= 0
        && x < grid[0].len().try_into().unwrap()
        && y < grid.len().try_into().unwrap();
}

fn guard_can_be_at(grid: &Vec<Vec<Tile>>, x: i32, y: i32) -> bool {
    if !is_in_bounds(&grid, x, y) {
        return false;
    }

    return grid[y as usize][x as usize] == Tile::Floor;
}

fn turn_90_degrees(dx: i32, dy: i32) -> (i32, i32) {
    match (dx, dy) {
        (0, -1) => return (1, 0),
        (1, 0) => return (0, 1),
        (0, 1) => return (-1, 0),
        (-1, 0) => return (0, -1),
        _ => panic!("Cannot turn"),
    }
}

// -> x, y, dx, dy, is_in_bounds
fn get_next_guard_state(
    grid: &Vec<Vec<Tile>>,
    guard_x: i32,
    guard_y: i32,
    guard_dx: i32,
    guard_dy: i32,
) -> (i32, i32, i32, i32, bool) {
    let new_x = guard_x + guard_dx;
    let new_y = guard_y + guard_dy;

    if !is_in_bounds(grid, new_x, new_y) {
        return (new_x, new_y, guard_dx, guard_dy, false);
    }

    if !guard_can_be_at(grid, new_x, new_y) {
        let (new_dx, new_dy) = turn_90_degrees(guard_dx, guard_dy);
        return (guard_x, guard_y, new_dx, new_dy, true);
    }

    return (new_x, new_y, guard_dx, guard_dy, true);
}

fn run_sim(
    grid: &Vec<Vec<Tile>>,
    mut guard_x: i32,
    mut guard_y: i32,
    mut guard_dx: i32,
    mut guard_dy: i32,
) -> (usize, bool) {
    let mut visited = HashSet::new();

    let mut loop_tol = grid.len() * grid[0].len() * 10;

    while loop_tol > 0 {
        /*println!(
            "Guard is at ({}, {}) with velocity ({}, {})",
            guard_x, guard_y, guard_dx, guard_dy
        );*/

        //print_grid(&grid, guard_x, guard_y, guard_dx, guard_dy);

        visited.insert((guard_x, guard_y));
        let (new_x, new_y, new_dx, new_dy, in_bounds) =
            get_next_guard_state(&grid, guard_x, guard_y, guard_dx, guard_dy);

        if !in_bounds {
            //println!("oob");
            break;
        }

        guard_x = new_x;
        guard_y = new_y;
        guard_dx = new_dx;
        guard_dy = new_dy;

        loop_tol -= 1;
    }

    return (visited.len(), loop_tol == 0);
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

    let mut grid: Vec<Vec<Tile>> = vec![];
    let lines: Vec<Vec<&str>> = res
        .lines()
        .map(|line| line.trim().split("").filter(|c| *c != "").collect())
        .collect();

    //println!("{:?}", lines);

    let (mut guard_x, mut guard_y) = (0, 0);
    let (mut guard_dx, mut guard_dy) = (0, 0);

    let mut y = 0;
    for line in lines {
        let mut row = vec![];
        let mut x = 0;
        for c in line {
            match c {
                "." => row.push(Tile::Floor),
                "#" => row.push(Tile::Wall),
                "^" => {
                    guard_x = x;
                    guard_y = y;
                    guard_dx = 0;
                    guard_dy = -1;
                    row.push(Tile::Floor);
                }
                ">" => {
                    guard_x = x;
                    guard_y = y;
                    guard_dx = 1;
                    guard_dy = 0;
                    row.push(Tile::Floor);
                }
                "<" => {
                    guard_x = x;
                    guard_y = y;
                    guard_dx = -1;
                    guard_dy = 0;
                    row.push(Tile::Floor);
                }
                "v" => {
                    guard_x = x;
                    guard_y = y;
                    guard_dx = 0;
                    guard_dy = 1;
                    row.push(Tile::Floor);
                }
                _ => {
                    println!("Error not recogize token {c}")
                }
            }
            x += 1;
        }
        grid.push(row);
        y += 1;
    }

    let (times, _) = run_sim(&grid, guard_x, guard_y, guard_dx, guard_dy);

    println!("Day 6 Part 1: {}", times);

    let mut valid_placements = 0;

    for y in tqdm(0..grid.len()) {
        for x in 0..grid[0].len() {
            if y as i32 == guard_y && x as i32 == guard_x {
                continue;
            }

            let prev_value = grid[y][x];
            grid[y][x] = Tile::Wall;

            let (_, stuck_in_loop) = run_sim(&grid, guard_x, guard_y, guard_dx, guard_dy);

            if stuck_in_loop {
                valid_placements += 1;
            }

            grid[y][x] = prev_value;
        }
    }

    println!("Day 6 Part 2: {}", valid_placements);
}
