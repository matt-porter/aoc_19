mod input;
use input::load_input_lines;

mod wires;
use wires::day3;

mod passwords;
use passwords::day4;

mod intcode;
use intcode::{day2, day5};

fn fuel(mass: i32) -> i32 {
    let val = mass / 3 - 2;
    if val <= 0 {
        0
    } else {
        val + fuel(val)
    }
}

fn day1() {
    let masses = load_input_lines();
    let fuel: i32 = masses.iter().map(|mass| fuel(*mass)).sum();
    println!("Day 1: {}", fuel);
}

fn main() {
//    day1();
//    day2();
//    day3();
//    day4();
    day5();
}

#[cfg(test)]
mod day1 {
    use super::*;

    #[test]
    fn test_fuel_simple() {
        assert_eq!(fuel(14), 2);
    }

    #[test]
    fn test_fuel_long() {
        assert_eq!(fuel(1969), 966);
        assert_eq!(fuel(100756), 50346);
    }
}

