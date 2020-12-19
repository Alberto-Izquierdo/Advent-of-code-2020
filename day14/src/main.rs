use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
fn main() {
    let lines = BufReader::new(File::open("input.txt").unwrap()).lines();
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
    println!("{}", values.iter().map(|(_, value)| *value).sum::<i64>());
}
