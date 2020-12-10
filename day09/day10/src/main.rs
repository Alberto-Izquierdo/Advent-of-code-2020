use std::collections::BTreeSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct JoltDifferences {
    one_jolt: u32,
    three_jolt: u32,
}

fn get_jolt_differences(input: BTreeSet<usize>) -> JoltDifferences {
    let mut previous_jolt = 0;
    input.into_iter().fold(
        JoltDifferences {
            one_jolt: 0,
            // The 1 represents the built-in joltage adapter (which is always 3 higher than the higher number)
            three_jolt: 1,
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
    let input: BTreeSet<usize> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|value| value.unwrap().parse::<usize>().unwrap())
        .collect();
    let jolt_differences = get_jolt_differences(input);
    println!("{:?}", jolt_differences);
    println!(
        "Result: {}",
        jolt_differences.one_jolt * jolt_differences.three_jolt
    );
}
