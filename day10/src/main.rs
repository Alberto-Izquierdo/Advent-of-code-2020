use std::collections::BTreeSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct JoltDifferences {
    one_jolt: u32,
    three_jolt: u32,
}

fn get_list_of_consecutive_numbers_group_sizes(input: Vec<usize>) -> Vec<u32> {
    let mut current_t = 0;
    let mut result = vec![];
    for i in 1..input.len() {
        if *input.get(i).unwrap() - *input.get(i - 1).unwrap() == 1 {
            current_t += 1;
        } else {
            if current_t != 1 {
                result.push(current_t);
                current_t = 1;
            }
        }
    }
    if current_t != 1 {
        result.push(current_t);
    }
    result
}

fn get_number_of_possibilities(input: Vec<usize>) -> u64 {
    let consecutive_group_sizes = get_list_of_consecutive_numbers_group_sizes(input);
    let possibilities_by_group = consecutive_group_sizes
        .iter()
        .filter(|value| **value > 2)
        .map(|v| match *v {
            3 => 2,
            4 => 4,
            5 => 7,
            _ => 0,
        })
        .collect::<Vec<u64>>();
    possibilities_by_group.into_iter().fold(1, |v, acc| v * acc)
}

fn get_jolt_differences(input: BTreeSet<usize>) -> JoltDifferences {
    let mut previous_jolt = 0;
    input.into_iter().fold(
        JoltDifferences {
            one_jolt: 0,
            three_jolt: 0,
        },
        |acc, current_jolt| {
            let prev_tmp = previous_jolt;
            previous_jolt = current_jolt;
            match current_jolt - prev_tmp {
                1 => JoltDifferences {
                    one_jolt: acc.one_jolt + 1,
                    three_jolt: acc.three_jolt,
                },
                3 => JoltDifferences {
                    one_jolt: acc.one_jolt,
                    three_jolt: acc.three_jolt + 1,
                },
                _ => acc,
            }
        },
    )
}

fn main() {
    let input: BTreeSet<usize> = std::iter::once(0)
        .chain(
            BufReader::new(File::open("input.txt").unwrap())
                .lines()
                .map(|value| value.unwrap().parse::<usize>().unwrap()),
        )
        .collect();
    let vector: Vec<usize> = input.into_iter().collect();
    let result = get_number_of_possibilities(vector);
    println!("{}", result);
}