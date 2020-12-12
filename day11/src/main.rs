use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, PartialEq, Copy, Clone)]
enum State {
    Floor,
    Empty,
    Occupied,
}

fn get_adjacent_occupied_seats(seats: &Vec<Vec<State>>, x: i32, y: i32) -> u32 {
    let len_x = seats.get(0).unwrap().len();
    let len_y = seats.len();
    let mut result = 0;
    for current_y in (y - 1).max(0)..(y + 2).min(len_y as i32) {
        for current_x in (x - 1).max(0)..(x + 2).min(len_x as i32) {
            if (current_x != x || current_y != y)
                && seats[current_y as usize][current_x as usize] == State::Occupied
            {
                result += 1;
            }
        }
    }
    result
}

fn step_one_callback(input: &Vec<Vec<State>>, x: usize, y: usize) -> State {
    let seat = input[y][x];
    match seat {
        State::Floor => State::Floor,
        State::Occupied => State::Occupied,
        State::Empty => {
            if get_adjacent_occupied_seats(&input, x as i32, y as i32) == 0 {
                State::Occupied
            } else {
                State::Empty
            }
        }
    }
}

fn step_two_callback(input: &Vec<Vec<State>>, x: usize, y: usize) -> State {
    let seat = input[y][x];
    match seat {
        State::Floor => State::Floor,
        State::Occupied => {
            if get_adjacent_occupied_seats(&input, x as i32, y as i32) >= 4 {
                State::Empty
            } else {
                State::Occupied
            }
        }
        State::Empty => State::Empty,
    }
}

fn apply_step(
    input: Vec<Vec<State>>,
    callback: impl Fn(&Vec<Vec<State>>, usize, usize) -> State,
) -> Vec<Vec<State>> {
    let len_x = input.get(0).unwrap().len();
    let len_y = input.len();
    let mut result: Vec<Vec<State>> = vec![];
    for y in 0..len_y {
        let mut current_line = vec![];
        for x in 0..len_x {
            current_line.push(callback(&input, x, y));
        }
        result.push(current_line);
    }
    result
}

#[allow(dead_code)]
fn draw_seats(input: &Vec<Vec<State>>) {
    println!();
    input.iter().for_each(|seats| {
        let rendered_seats = seats
            .iter()
            .map(|seat| match *seat {
                State::Floor => '.',
                State::Occupied => '#',
                State::Empty => 'L',
            })
            .collect::<String>();
        println!("{}", rendered_seats);
    });
    println!();
}

fn get_total_occupied_seats(input: Vec<Vec<State>>) -> usize {
    input.into_iter().fold(0, |acc, value| {
        acc + value
            .into_iter()
            .filter(|seat| *seat == State::Occupied)
            .count()
    })
}

fn main() {
    let mut seats: Vec<Vec<State>> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|value| {
            value
                .unwrap()
                .chars()
                .into_iter()
                .map(|character| match character {
                    '.' => State::Floor,
                    'L' => State::Empty,
                    '#' => State::Occupied,
                    _ => panic!("The character is not correct"),
                })
                .collect::<Vec<State>>()
        })
        .collect();
    let mut previous_state = vec![];
    while previous_state != seats {
        previous_state = seats.clone();
        seats = apply_step(seats, step_one_callback);
        seats = apply_step(seats, step_two_callback);
    }
    let occupied_seats = get_total_occupied_seats(seats);
    println!("Occupied seats: {:?}", occupied_seats);
}
