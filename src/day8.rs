use core::f32;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, PartialEq, Eq)]
struct Antenna {
    freq: String,
    x: i32,
    y: i32,
}

fn is_on_grid(pos: (i32, i32), bounds: (usize, usize)) -> bool {
    let (x, y) = pos;
    let (w, h) = bounds;
    return x >= 0 && y >= 0 && x < w as i32 && y < h as i32;
}

/*fn distance(a: (i32, i32), b: (i32, i32)) -> f32 {
    return f32::sqrt((a.0 as f32 - b.0 as f32).powi(2) + (a.1 as f32 - b.1 as f32).powi(2));
}*/

fn get_line_between_antennas(a: &Antenna, b: &Antenna, bounds: (usize, usize)) -> Vec<(i32, i32)> {
    let (w, _) = bounds;
    let slope = (b.y - a.y) as f32 / (b.x - a.x) as f32;
    let intercept = a.y as f32 - slope * a.x as f32;

    let mut points = vec![];
    //println!("Slope {slope}");

    for x in 0..(w as i32) {
        let y_maybe = (slope) * (x as f32) + (intercept);

        let mut y = -999;

        // If we are basically on a round grid pos
        if ((y_maybe).round() - y_maybe).abs() < 0.0001 {
            y = y_maybe.round() as i32;
        }

        //println!("{x}, {y}");
        let candidate_point = (x, y);

        if is_on_grid(candidate_point, bounds) {
            //println!("{x}, {y} is on grid");
            points.push(candidate_point);
        }
    }

    return points;
}

fn get_antinodes(a: &Antenna, b: &Antenna, bounds: (usize, usize)) -> Vec<(i32, i32)> {
    let mut antinodes = vec![];

    for candidate_point in get_line_between_antennas(a, b, bounds) {
        //let dist_a = distance((a.x, a.y), candidate_point);
        //let dist_b = distance((b.x, b.y), candidate_point);

        //if (dist_a - 2.00 * dist_b).abs() < 0.00001 || (dist_b - 2.00 * dist_a).abs() < 0.00001 {
        if is_on_grid(candidate_point, bounds) {
            /*println!(
                "{}, {} -> a:{} b:{}",
                candidate_point.0, candidate_point.1, dist_a, dist_b
            );*/
            antinodes.push(candidate_point);
        } else {
            println!("Point not on grid: {:?}", candidate_point);
        }
        //}
    }

    return antinodes;
}

fn print_map(
    ants: &HashMap<&str, Vec<Antenna>>,
    antinodes: &HashMap<&str, HashSet<(i32, i32)>>,
    bounds: (usize, usize),
) {
    let (w, h) = bounds;

    let mut to_print = vec![];

    for _ in 0..h {
        let mut row = vec![];
        for _ in 0..w {
            row.push(".");
        }
        to_print.push(row);
    }

    // Print ants
    for (freq, antennas) in ants {
        for antenna in antennas {
            to_print[antenna.y as usize][antenna.x as usize] = freq;
        }
    }

    // Print lines
    /*for (_, antennas) in ants {
        for antenna in antennas {
            for other_antenna in antennas {
                if antenna == other_antenna {
                    continue;
                }

                let points = get_line_between_antennas(antenna, other_antenna, bounds);

                for (x, y) in points {
                    if to_print[y as usize][x as usize] == "." {
                        to_print[y as usize][x as usize] = "%";
                    }
                }
            }
        }
    }*/

    // Print antinodes
    for (_, antinode_vec) in antinodes {
        for (x, y) in antinode_vec {
            to_print[*y as usize][*x as usize] = "#";
        }
    }

    for row in to_print {
        println!("{}", row.join(""))
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

    let lines: Vec<Vec<&str>> = res
        .lines()
        .map(|line| line.trim().split("").filter(|c| *c != "").collect())
        .collect();

    let mut unique_ants = HashSet::new();
    let mut ants: HashMap<&str, Vec<Antenna>> = HashMap::new();

    let (w, h) = (lines[0].len(), lines.len());

    for y in 0..h {
        for x in 0..w {
            let tile = lines[y][x];
            match tile {
                "." => (),
                freq => {
                    ants.entry(freq).or_default().push(Antenna {
                        freq: freq.to_string(),
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
    }

    let mut antinodes: HashMap<&str, HashSet<(i32, i32)>> = HashMap::new();

    for (freq, antennas) in ants.iter().clone() {
        for i in 0..antennas.len() {
            for j in 0..i {
                let a = &antennas[i];
                let b = &antennas[j];

                for antinode in get_antinodes(a, b, (w, h)) {
                    unique_ants.insert(antinode);
                    antinodes.entry(freq).or_default().insert(antinode);
                }
            }
        }
    }

    print_map(&ants, &antinodes, (w, h));

    /*let mut total_nodes = 0;
    for (_, nodes) in antinodes {
        total_nodes += nodes.len();
    }*/

    println!("Day 8 Part 1: {:?}", unique_ants.len());
}
