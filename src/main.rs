use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn load_input() -> Vec<i32> {
    let f = File::open("input.txt").expect("Failed to open input.txt");
    BufReader::new(f)
        .lines()
        .map(|l| i32::from_str(&l.unwrap()).unwrap()).collect()
}

fn fuel(mass: i32) -> i32 {
    let val = mass / 3 - 2;
    if val <= 0 {
        0
    } else {
        val + fuel(val)
    }
}

fn main() {
    let masses = load_input();
    let fuel: i32 = masses.iter().map(|mass| fuel(*mass)).sum();
    println!("{}", fuel);
}

#[cfg(test)]
mod tests {
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