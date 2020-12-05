use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn _is_password_valid(password: &str, character: char, min: u32, max: u32) -> bool {
    let ocurrences = password.chars().filter(|c| *c == character).count() as u32;
    return ocurrences >= min && ocurrences <= max;
}

fn _is_password_in_line_valid(line: &str) -> bool {
    let re = Regex::new(r"([^-]+)-([^\s]+) (.): (.*)").unwrap();
    let captured_groups = re.captures(&line).unwrap();
    let min = &captured_groups[1].parse::<u32>().unwrap();
    let max = &captured_groups[2].parse::<u32>().unwrap();
    let character = &captured_groups[3].parse::<char>().unwrap();
    let password = &captured_groups[4];
    return _is_password_valid(password, *character, *min, *max);
}

fn is_password_valid(password: &str, character: char, first: u32, second: u32) -> bool {
    let a = password.chars().nth(first as usize - 1).unwrap();
    let b = password.chars().nth(second as usize - 1).unwrap();
    return a != b && (a == character || b == character);
}

fn is_password_in_line_valid(line: &str) -> bool {
    let re = Regex::new(r"([^-]+)-([^\s]+) (.): (.*)").unwrap();
    let captured_groups = re.captures(&line).unwrap();
    let first = &captured_groups[1].parse::<u32>().unwrap();
    let second = &captured_groups[2].parse::<u32>().unwrap();
    let character = &captured_groups[3].parse::<char>().unwrap();
    let password = &captured_groups[4];
    let result = is_password_valid(password, *character, *first, *second);
    return result;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let correct_passwords = reader
        .lines()
        .filter(|line| is_password_in_line_valid(line.as_ref().unwrap()))
        .count();
    println!("Result: {}", correct_passwords);
}
