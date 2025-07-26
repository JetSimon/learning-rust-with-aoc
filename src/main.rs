mod day1;
mod day2;
mod day3;
fn main() {
    let day = 3;
    match day {
        1 => crate::day1::run("./input/1.txt".to_string()),
        2 => crate::day2::run("./input/2.txt".to_string()),
        3 => crate::day3::run("./input/3.txt".to_string()),
        _ => println!("Day not impl"),
    }
}
