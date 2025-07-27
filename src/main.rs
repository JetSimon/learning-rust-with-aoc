mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let day = 6;
    match day {
        1 => crate::day1::run("./input/1.txt".to_string()),
        2 => crate::day2::run("./input/2.txt".to_string()),
        3 => crate::day3::run("./input/3.txt".to_string()),
        4 => crate::day4::run("./input/4.txt".to_string()),
        5 => crate::day5::run("./input/5.txt".to_string()),
        6 => crate::day6::run("./input/6.txt".to_string()),
        _ => println!("Day not impl"),
    }
}
