use std::collections::BTreeSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_partial_id(boarding_pass: &str, limit: usize, init: usize, letter: char) -> u32 {
    let mut id = 0;
    for i in 0..limit {
        if boarding_pass.chars().skip(init).nth(i).unwrap() == letter {
            id += 2i32.pow(limit as u32 - 1 - i as u32);
        }
    }
    id as u32
}

fn get_boarding_pass_id(boarding_pass: &str) -> u32 {
    get_partial_id(boarding_pass, 7, 0, 'B') * 8 + get_partial_id(boarding_pass, 3, 7, 'R')
}

fn main() {
    let lines = BufReader::new(File::open("input.txt").unwrap()).lines();
    let result = lines
        .map(|line| get_boarding_pass_id(line.unwrap().as_str()))
        .collect::<BTreeSet<u32>>();
    let mut previous_id = result.iter().next().unwrap().clone();
    let result = result.into_iter().skip(1).find(|id| {
        if *id != previous_id + 1 {
            true
        } else {
            previous_id = *id;
            false
        }
    }).unwrap();
    println!("Previous = {} current = {}", previous_id, result);
}
