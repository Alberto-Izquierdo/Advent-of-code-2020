use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

fn get_sum_data_from_groups(lines: Lines<BufReader<File>>) -> u32 {
    let mut current_group_data = HashMap::new();
    let mut lines_in_this_group = 0;
    lines
        .map(|line| {
            if line.as_ref().unwrap().is_empty() {
                let result = current_group_data
                    .iter()
                    .filter(|pair| *pair.1 == lines_in_this_group)
                    .count() as u32;
                current_group_data.clear();
                lines_in_this_group = 0;
                result
            } else {
                line.unwrap().chars().for_each(|letter| {
                    match current_group_data.clone().get(&letter) {
                        Some(value) => current_group_data.insert(letter, value + 1),
                        None => current_group_data.insert(letter, 1),
                    };
                });
                lines_in_this_group += 1;
                0
            }
        })
        .sum()
}

fn main() {
    let lines = BufReader::new(File::open("input.txt").unwrap()).lines();
    println!("{}", get_sum_data_from_groups(lines));
}
