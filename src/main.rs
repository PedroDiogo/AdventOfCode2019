pub mod days;
use days::*;
use std::env;

fn main() {
    match get_day() {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),
        5 => day5::run(),
        6 => day6::run(),
        _ => println!("Unknown day - {}", get_day())
    }
}

fn get_day() -> i32 {
    return env::args()
        .collect::<Vec<String>>()
        .get(1)
        .expect("Need to pass one argument as the day number. Example: cargo run 1")
        .parse::<i32>()
        .expect("Expecting first argument to be an integer");
}