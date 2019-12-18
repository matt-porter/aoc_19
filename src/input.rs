use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

pub fn load_input_lines(filename: &str) -> Vec<i32> {
    let f = File::open(format!("input/{}", filename)).expect("Failed to open file");
    BufReader::new(f)
        .lines()
        .map(|l| i32::from_str(&l.unwrap()).unwrap()).collect()
}

pub fn load_input_lines_str(filename: &str) -> Vec<String> {
    let f = File::open(format!("input/{}", filename)).expect("Failed to open file");
    BufReader::new(f)
        .lines()
        .filter_map(Result::ok)
        .collect()
}

pub fn split_commas(s: String) -> Vec<i32> {
    s.split(',')
        .map(|l| i32::from_str(&l)).filter_map(Result::ok).collect()
}
pub fn load_input_commas() -> Vec<i32> {
    let f = File::open("input/day2.txt").expect("Failed to open day2.txt");
    let mut s = String::new();
    BufReader::new(f)
        .read_to_string(&mut s).expect("Failed to read string");
    let s= s.trim();
    split_commas(s.to_owned())
}