use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};
fn part_1(lines: Lines<BufReader<File>>) -> i64 {
    let mut and_mask = 0;
    let mut or_mask = 0;
    let mut values: HashMap<i64, i64> = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        if line.starts_with("mask") {
            let mask = line.split(" = ").skip(1).next().unwrap();
            and_mask = 2i64.pow(37) - 1;
            or_mask = 0;
            mask.chars()
                .rev()
                .enumerate()
                .for_each(|(index, value)| match value {
                    '0' => and_mask -= 2i64.pow(index as u32),
                    '1' => or_mask += 2i64.pow(index as u32),
                    _ => {}
                });
        } else if line.starts_with("mem") {
            let number = line
                .split(" = ")
                .skip(1)
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let number = number & and_mask;
            let number = number | or_mask;
            let memory_address = line
                .split("[")
                .skip(1)
                .next()
                .unwrap()
                .split("]")
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            values.insert(memory_address, number);
        } else {
            panic!("Line not valid");
        }
    }
    values.iter().map(|(_, value)| *value).sum::<i64>()
}

fn get_memory_addresses(original_address: &str, mask: &str) -> Vec<i64> {
    if mask.contains("X") {
        let mut masks = mask.splitn(2, "X");
        let first_part = masks.next().unwrap();
        let second_part = masks.next().unwrap();
        let new_mask = format!("{}0{}", first_part, second_part);
        let first = original_address
            .chars()
            .take(first_part.len())
            .chain("0".chars())
            .chain(original_address.chars().skip(first_part.len() + 1))
            .collect::<String>();
        let second = original_address
            .chars()
            .take(first_part.len())
            .chain("1".chars())
            .chain(original_address.chars().skip(first_part.len() + 1))
            .collect::<String>();
        get_memory_addresses(first.as_str(), new_mask.as_str())
            .into_iter()
            .chain(get_memory_addresses(second.as_str(), new_mask.as_str()))
            .collect::<Vec<_>>()
    } else {
        let mask_int = i64::from_str_radix(mask, 2).unwrap();
        let address_int = i64::from_str_radix(original_address, 2).unwrap();
        vec![mask_int | address_int]
    }
}

fn part_2(lines: Lines<BufReader<File>>) -> i64 {
    let mut mask = String::from("");
    let mut values: HashMap<i64, i64> = HashMap::new();
    for line in lines {
        let line_unwrapped = line.unwrap();
        if line_unwrapped.starts_with("mask") {
            mask = line_unwrapped
                .split(" = ")
                .skip(1)
                .next()
                .unwrap()
                .to_string();
        } else if line_unwrapped.starts_with("mem") {
            let number = line_unwrapped
                .split(" = ")
                .skip(1)
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let memory_address = line_unwrapped
                .split("[")
                .skip(1)
                .next()
                .unwrap()
                .split("]")
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let memory_addresses =
                get_memory_addresses(format!("{:036b}", memory_address).as_str(), mask.as_str());
            for address in memory_addresses {
                values.insert(address, number);
            }
        } else {
            panic!("Line not valid");
        }
    }
    values.iter().map(|(_, value)| *value).sum::<i64>()
}

fn main() {
    let lines = BufReader::new(File::open("input.txt").unwrap()).lines();
    println!("Part 1 result: {}", part_1(lines));
    let lines = BufReader::new(File::open("input.txt").unwrap()).lines();
    println!("Part 2 result: {}", part_2(lines));
}
