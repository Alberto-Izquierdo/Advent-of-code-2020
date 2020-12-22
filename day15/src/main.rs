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

fn get_last_number(
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
        get_last_number(current_number, current_turn + 1, max_turn, previous_numbers)
    }
}

fn main() {
    let input = vec![0, 5, 4, 1, 10, 14, 7];
    let initial_turn = input.len() as u32;
    let (last_number, map) = get_initial_values(input);
    let result = get_last_number(last_number, initial_turn, 2020, map);
    println!("Result: {}", result);
}
