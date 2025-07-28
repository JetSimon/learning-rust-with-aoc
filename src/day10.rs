use core::num;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs,
    i64::MAX,
    vec,
};

type Position = (i64, i64);

// https://doc.rust-lang.org/std/collections/binary_heap/index.html
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i64,
    position: Position,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbours(grid: &Vec<Vec<i64>>, position: Position) -> Vec<Position> {
    let (w, h) = (grid[0].len() as i64, grid.len() as i64);
    let mut neighbours = vec![];
    let (x, y) = position;

    if x > 0 {
        neighbours.push((x - 1, y));
    }

    if y > 0 {
        neighbours.push((x, y - 1));
    }

    if x < w - 1 {
        neighbours.push((x + 1, y));
    }

    if y < h - 1 {
        neighbours.push((x, y + 1));
    }

    let current_height = grid[y as usize][x as usize];
    return neighbours
        .iter()
        .filter(|(nx, ny)| grid[*ny as usize][*nx as usize] - current_height == 1)
        .map(|p| *p)
        .collect();
}

fn get_longest_path(grid: &Vec<Vec<i64>>, start: Position, end: Position) -> Vec<(i64, i64)> {
    let current_pos = start;

    let mut dist = HashMap::new();
    let mut prev = HashMap::new();
    dist.insert(start, 0);

    let mut q = BinaryHeap::new();
    q.push(State {
        position: start,
        cost: 0,
    });

    while let Some(State { cost, position }) = q.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == end {
            break;
        }

        // Important as we may have already found a better way
        if cost > *dist.entry(position).or_insert(MAX) {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for neighbour_position in get_neighbours(&grid, position) {
            let (x, y) = neighbour_position;
            let neighbour_height = grid[y as usize][x as usize];

            let next = State {
                cost: cost - neighbour_height,
                position: neighbour_position,
            };

            // If so, add it to the frontier and continue
            if next.cost < *dist.entry(neighbour_position).or_insert(MAX) {
                q.push(next);
                // Relaxation, we have now found a better way
                dist.insert(next.position, next.cost);
                prev.insert(next.position, position);
            }
        }
    }

    let mut path = vec![];

    let mut previous_state = end;
    let mut next_state = prev.get(&previous_state);

    loop {
        match next_state {
            Some(state) => {
                path.push(previous_state);
                previous_state = *state;
                next_state = prev.get(&previous_state)
            }
            None => break,
        }
    }

    if path.len() > 0 && *path.last().unwrap() != start {
        print!()
        return vec![];
    }

    return path;
}

fn print_grid(grid: Vec<Vec<i64>>) {
    for row in grid {
        println!("{:?}", row);
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

    let mut grid: Vec<Vec<i64>> = res
        .lines()
        .map(|line| {
            line.split("")
                .filter(|c| *c != "")
                .map(|c| c.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let mut starts = vec![];
    let mut ends = vec![];

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            match grid[y][x] {
                9 => ends.push((x as i64, y as i64)),
                0 => starts.push((x as i64, y as i64)),
                _ => (),
            }
        }
    }

    let mut score = 0;

    for start in starts {
        let mut num_reached = 0;
        for end in ends.clone() {
            if get_longest_path(&grid, start, end).len() > 0 {
                num_reached = 1;
            }
        }
        score += num_reached;
    }

    println!("Day 10 Part 1: {score}");
}
