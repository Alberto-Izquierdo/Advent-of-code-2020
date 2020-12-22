use std::collections::HashMap;

fn get_initial_values(values: Vec<u32>) -> (u32, HashMap<u32, u32>) {
    let values_len = values.len();
    (
        *values.last().unwrap(),
        values
            .into_iter()
            .take(values_len - 1)
            .enumerate()
            .map(|(index, value)| (value, index as u32))
            .collect::<HashMap<_, _>>(),
    )
}

fn part_1(
    last_number: u32,
    current_turn: u32,
    max_turn: u32,
    mut previous_numbers: HashMap<u32, u32>,
) -> u32 {
    if current_turn >= max_turn {
        last_number
    } else {
        let current_number = match previous_numbers.get(&last_number) {
            Some(value) => {
                let result = current_turn - 1 - value;
                previous_numbers.insert(last_number, current_turn - 1);
                result
            }
            None => {
                previous_numbers.insert(last_number, current_turn - 1);
                0
            }
        };
        part_1(current_number, current_turn + 1, max_turn, previous_numbers)
    }
}

fn part_2(
    last_number: u32,
    first_turn: u32,
    max_turn: u32,
    mut previous_numbers: HashMap<u32, u32>,
) -> u32 {
    let mut last_number = last_number;
    for current_turn in first_turn..max_turn {
        last_number = match previous_numbers.get(&last_number) {
            Some(value) => {
                let result = current_turn - 1 - value;
                previous_numbers.insert(last_number, current_turn - 1);
                result
            }
            None => {
                previous_numbers.insert(last_number, current_turn - 1);
                0
            }
        };
    }
    return last_number;
}

fn main() {
    let input = vec![0, 5, 4, 1, 10, 14, 7];
    let initial_turn = input.len() as u32;
    let (last_number, map) = get_initial_values(input);
    let result = part_2(last_number, initial_turn, 30000000, map);
    println!("Result: {}", result);
}
