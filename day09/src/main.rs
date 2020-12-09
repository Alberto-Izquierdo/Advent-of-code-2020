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
    let value = queue.get(preamble).unwrap().clone();

    match does_queue_contain_sum(&queue, value) {
        false => value,
        true => find_first_wrong_number(
            queue.into_iter().skip(1).collect::<VecDeque<usize>>(),
            preamble,
        ),
    }
}

fn main() {
    let lines: VecDeque<usize> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|value| value.unwrap().parse::<usize>().unwrap())
        .collect();
    let result = find_first_wrong_number(lines.clone(), 25);
    println!("Result: {}", result);
}
