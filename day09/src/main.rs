use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn does_queue_contain_sum(queue: &VecDeque<usize>, sum: usize) -> bool {
    for i in 0..queue.len() - 1 {
        for j in i + 1..queue.len() {
            if queue.get(i).unwrap() + queue.get(j).unwrap() == sum {
                return true;
            }
        }
    }
    false
}

fn find_first_wrong_number(queue: VecDeque<usize>, preamble: usize) -> usize {
    let value = *queue.get(preamble).unwrap();

    match does_queue_contain_sum(&queue, value) {
        false => value,
        true => find_first_wrong_number(
            queue.into_iter().skip(1).collect::<VecDeque<usize>>(),
            preamble,
        ),
    }
}

fn find_consecutive_numbers_that_sum(queue: VecDeque<usize>, sum: usize) -> Vec<usize> {
    let mut current_sum = 0;
    let result: Vec<usize> = queue
        .iter()
        .take_while(|value| {
            current_sum += *value;
            current_sum < sum
        })
        .map(|value| *value)
        .collect();
    if current_sum == sum {
        result
    } else {
        find_consecutive_numbers_that_sum(queue.into_iter().skip(1).collect(), sum)
    }
}

fn main() {
    let lines: VecDeque<usize> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|value| value.unwrap().parse::<usize>().unwrap())
        .collect();
    let result = find_first_wrong_number(lines.clone(), 25);
    println!("Result: {}", result);
    let result = find_consecutive_numbers_that_sum(lines, result);
    println!(
        "result: {}",
        result.iter().min().unwrap() + result.iter().max().unwrap()
    );
}
