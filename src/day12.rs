use std::{collections::HashMap, collections::HashSet, fs};

fn is_valid(grid: &Vec<Vec<&str>>, x: i64, y: i64, target_symbol: &str) -> bool {
    if x < 0 || y < 0 || x >= grid[0].len() as i64 || y >= grid.len() as i64 {
        return false;
    }
    return grid[y as usize][x as usize] == target_symbol;
}

fn is_corner(grid: &Vec<Vec<&str>>, x: i64, y: i64) -> bool {
    let symbol = grid[y as usize][x as usize];

    // Top left
    if is_valid(grid, x + 1, y, symbol)
        && is_valid(grid, x, y + 1, symbol)
        && !is_valid(grid, x - 1, y, symbol)
        && !is_valid(grid, x, y - 1, symbol)
    {
        return true;
    }
    // Top right
    if is_valid(grid, x - 1, y, symbol)
        && is_valid(grid, x, y + 1, symbol)
        && !is_valid(grid, x + 1, y, symbol)
        && !is_valid(grid, x, y - 1, symbol)
    {
        return true;
    }
    // Bottom left
    if is_valid(grid, x + 1, y, symbol)
        && is_valid(grid, x, y - 1, symbol)
        && !is_valid(grid, x - 1, y, symbol)
        && !is_valid(grid, x, y + 1, symbol)
    {
        return true;
    }
    // Bottom right
    if is_valid(grid, x - 1, y, symbol)
        && is_valid(grid, x, y - 1, symbol)
        && !is_valid(grid, x + 1, y, symbol)
        && !is_valid(grid, x, y + 1, symbol)
    {
        return true;
    }

    return false;
}

fn get_neighbours(grid: &Vec<Vec<&str>>, x: usize, y: usize) -> (Vec<(usize, usize)>, i64) {
    let mut neighbours = vec![];
    let (w, h) = (grid[0].len() as usize, grid.len() as usize);

    let mut invalids = 0;

    if x > 0 {
        neighbours.push((x - 1, y));
    } else {
        invalids += 1;
    }

    if y > 0 {
        neighbours.push((x, y - 1));
    } else {
        invalids += 1;
    }

    if x < w - 1 {
        neighbours.push((x + 1, y));
    } else {
        invalids += 1;
    }

    if y < h - 1 {
        neighbours.push((x, y + 1));
    } else {
        invalids += 1;
    }

    return (neighbours, invalids);
}

fn flood(
    grid: &Vec<Vec<&str>>,
    symbol: &str,
    x: usize,
    y: usize,
    areas: &mut HashMap<i64, i64>,
    id: i64,
    visited: &mut HashSet<(usize, usize)>,
    perims: &mut HashMap<i64, i64>,
    corners: &mut HashMap<i64, i64>,
) {
    let this_tile = grid[y][x];
    if visited.contains(&(x, y)) || (symbol != "" && this_tile != symbol) {
        return;
    }
    visited.insert((x, y));

    areas.entry(id).and_modify(|a| *a += 1).or_insert(1);

    let corner = if is_corner(grid, x as i64, y as i64) {
        1
    } else {
        0
    };

    corners.entry(id).and_modify(|c| *c += 1).or_insert(corner);

    let (neighbours, invalids) = get_neighbours(&grid, x, y);

    for (nx, ny) in neighbours {
        let on_edge: i64 = if this_tile == grid[ny][nx] { 0 } else { 1 };
        perims
            .entry(id)
            .and_modify(|n| *n += on_edge)
            .or_insert(on_edge);

        flood(
            &grid, this_tile, nx, ny, areas, id, visited, perims, corners,
        );
    }

    perims
        .entry(id)
        .and_modify(|n| *n += invalids)
        .or_insert(invalids);
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

    let grid: Vec<Vec<&str>> = res
        .lines()
        .map(|line| line.split("").filter(|c| *c != "").collect())
        .collect();

    let mut areas = HashMap::new();
    let mut perims = HashMap::new();
    let mut corners = HashMap::new();
    let mut visited = HashSet::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let id = y * grid.len() + x;
            flood(
                &grid,
                "",
                x,
                y,
                &mut areas,
                id as i64,
                &mut visited,
                &mut perims,
                &mut corners,
            );
        }
    }

    //println!("Num areas {}", areas.len());

    println!("{:?}", areas);
    println!("{:?}", perims);
    println!("{:?}", corners);

    let mut total_cost = 0;
    let mut total_cost_2 = 0;

    for id in areas.keys() {
        total_cost += areas.get(&id).unwrap() * perims.get(&id).unwrap();
        total_cost_2 += areas.get(&id).unwrap() * corners.get(&id).unwrap();
    }

    println!("Price 1: {}", total_cost);
    println!("Price 2: {}", total_cost_2);
}
