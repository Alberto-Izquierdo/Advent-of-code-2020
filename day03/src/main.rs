use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

fn count_number_of_trees_found(
    lines: Lines<BufReader<File>>,
    x_step: usize,
    y_step: usize,
) -> usize {
    let mut x = 0;
    lines
        .step_by(y_step)
        .filter(|line| {
            let c = line.as_ref().unwrap().chars().nth(x).unwrap();
            x = (x + x_step) % line.as_ref().unwrap().len();
            c == '#'
        })
        .count()
}

fn main() {
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let result: usize = slopes
        .into_iter()
        .map(|slope| {
            count_number_of_trees_found(
                BufReader::new(File::open("input.txt").unwrap()).lines(),
                slope.0,
                slope.1,
            )
        })
        .product();
    println!("Result: {}", result);
}
