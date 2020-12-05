use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_numbers_in_file(filename: String) -> Vec<i32> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let numbers = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
    numbers
}

fn get_two_numbers_that_sum(numbers: &Vec<i32>, sum: i32) -> (i32, i32) {
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if numbers[i] + numbers[j] == sum {
                return (numbers[i] as i32, numbers[j] as i32);
            }
        }
    }
    return (0, 0);
}

fn get_three_numbers_that_sum(numbers: &Vec<i32>, sum: i32) -> (i32, i32, i32) {
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            for k in j + 1..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == sum {
                    return (numbers[i] as i32, numbers[j] as i32, numbers[k] as i32);
                }
            }
        }
    }
    return (0, 0, 0);
}

fn main() {
    let values = vec![1721, 979, 366, 299, 675, 1456];
    let result = get_two_numbers_that_sum(&values, 2020);
    println!("Result: {:?}", result);
    let result = result.0 * result.1;
    println!("Result: {}", result);
    let result = get_three_numbers_that_sum(&values, 2020);
    println!("Result: {:?}", result);
    let result = result.0 * result.1 * result.2;
    println!("Result: {}", result);
    let file = "input.txt";
    let numbers = read_numbers_in_file(file.to_string());
    let result = get_two_numbers_that_sum(&numbers, 2020);
    println!("Result: {:?}", result);
    let result = result.0 * result.1;
    println!("Result: {}", result);
    let result = get_three_numbers_that_sum(&numbers, 2020);
    println!("Result: {:?}", result);
    let result = result.0 * result.1 * result.2;
    println!("Result: {}", result);
}
