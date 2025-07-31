mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let day = 12;
    match day {
        1 => crate::day1::run("./input/1.txt".to_string()),
        2 => crate::day2::run("./input/2.txt".to_string()),
        3 => crate::day3::run("./input/3.txt".to_string()),
        4 => crate::day4::run("./input/4.txt".to_string()),
        5 => crate::day5::run("./input/5.txt".to_string()),
        6 => crate::day6::run("./input/6.txt".to_string()),
        7 => crate::day7::run("./input/7.txt".to_string()),
        8 => crate::day8::run("./input/8.txt".to_string()),
        9 => crate::day9::run("./input/9.txt".to_string()),
        10 => crate::day10::run("./input/10.txt".to_string()),
        11 => crate::day11::run("./input/11.txt".to_string()),
        12 => crate::day12::run("./input/12.txt".to_string()),
        _ => println!("Day not impl"),
    }
}
