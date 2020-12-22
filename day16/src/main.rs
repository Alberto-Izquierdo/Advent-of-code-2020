use std::fs::File;
use std::io::{prelude::*, BufReader};
fn part_1(lines: Vec<String>) -> u32 {
    let rules_lines = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| line.splitn(2, ": ").nth(1).unwrap())
        .map(|line| {
            line.splitn(2, " or ")
                .map(|pair| {
                    let mut splitted = pair.splitn(2, "-");
                    (
                        splitted.next().unwrap().parse::<u32>().unwrap(),
                        splitted.next().unwrap().parse::<u32>().unwrap(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .fold(vec![], |acc, value| {
            acc.into_iter().chain(value.into_iter()).collect::<Vec<_>>()
        });
    let result = lines
        .into_iter()
        .skip_while(|line| line != "nearby tickets:")
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|value| value.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .fold(vec![], |acc, value| {
            acc.into_iter().chain(value.into_iter()).collect::<Vec<_>>()
        })
        .into_iter()
        .filter(|value| {
            for restriction in &rules_lines {
                if *value >= restriction.0 && *value <= restriction.1 {
                    return false;
                }
            }
            true
        })
        .sum();
    result
}

fn main() {
    let lines = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|value| value.unwrap())
        .collect::<Vec<_>>();
    let result = part_1(lines);
    println!("Result: {}", result);
}
