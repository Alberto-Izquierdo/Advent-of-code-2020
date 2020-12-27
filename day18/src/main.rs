use std::fs::File;
use std::io::{prelude::*, BufReader};

enum Operation {
    Product,
    Addition,
}

fn calculate(line: &str) -> (i64, i64) {
    let mut current_index = 0;
    let mut result = 0;
    let mut previous_operation: Operation = Operation::Addition;
    while current_index < line.len() {
        let current_char = line.chars().nth(current_index).unwrap();
        match current_char {
            '+' => {
                previous_operation = Operation::Addition;
            }
            '*' => {
                previous_operation = Operation::Product;
            }
            '(' => {
                let new_line = line.chars().skip(current_index + 1).collect::<String>();
                let inner_value = calculate(&new_line);
                match previous_operation {
                    Operation::Product => result *= inner_value.0,
                    Operation::Addition => result += inner_value.0,
                }
                current_index += inner_value.1 as usize + 1;
            }
            ')' => {
                return (result, current_index as i64);
            }
            value => {
                let v = value.to_string().parse::<i64>().unwrap();
                match previous_operation {
                    Operation::Product => result *= v,
                    Operation::Addition => result += v,
                }
            }
        }
        current_index += 1
    }
    (result, current_index as i64)
}

fn part_1(line: &str) -> i64 {
    let line = line.replace(" ", "");
    calculate(&line).0
}

fn main() {
    let result = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .into_iter()
        .fold(0, |result, line| {
            result + part_1(line.unwrap().as_str())
        });
    println!("Result: {}", result);
}
