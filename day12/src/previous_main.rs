use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Action {
    action: char,
    times: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum CardinalDirection {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl CardinalDirection {
    fn new(input: char) -> CardinalDirection {
        match input {
            'N' => CardinalDirection::NORTH,
            'S' => CardinalDirection::SOUTH,
            'E' => CardinalDirection::EAST,
            'W' => CardinalDirection::WEST,
            _ => panic!(),
        }
    }

    fn add(self, value: i32) -> CardinalDirection {
        let directions = vec![
            CardinalDirection::NORTH,
            CardinalDirection::EAST,
            CardinalDirection::SOUTH,
            CardinalDirection::WEST,
        ];
        let previous_index = directions
            .iter()
            .position(|direction| *direction == self)
            .unwrap() as i32;
        let new_index = (previous_index + value) % 4;
        directions[if new_index < 0 {
            (new_index + 4) as usize
        } else {
            new_index as usize
        }]
    }

    fn get_direction(&self) -> (i32, i32) {
        match self {
            CardinalDirection::NORTH => (0, 1),
            CardinalDirection::SOUTH => (0, -1),
            CardinalDirection::EAST => (1, 0),
            CardinalDirection::WEST => (-1, 0),
        }
    }
}

#[derive(Debug)]
struct Ship {
    facing: CardinalDirection,
    position: (i32, i32),
}

impl Ship {
    fn new() -> Ship {
        Ship {
            facing: CardinalDirection::EAST,
            position: (0, 0),
        }
    }

    fn set_facing(self, direction: CardinalDirection) -> Ship {
        Ship {
            facing: direction,
            position: self.position,
        }
    }

    fn set_position(self, position: (i32, i32)) -> Ship {
        Ship {
            facing: self.facing,
            position,
        }
    }

    fn turn(self, degrees: i32) -> Ship {
        let new_cardinal_direction = self.facing.add(degrees / 90);
        self.set_facing(new_cardinal_direction)
    }

    fn move_in_direction(self, cardinal_direction: CardinalDirection, distance: u32) -> Ship {
        let direction = cardinal_direction.get_direction();
        let previous_position = self.position;
        self.set_position((
            previous_position.0 + direction.0 * distance as i32,
            previous_position.1 + direction.1 * distance as i32,
        ))
    }

    fn move_forward(self, distance: u32) -> Ship {
        let facing = self.facing;
        self.move_in_direction(facing, distance)
    }

    fn apply_action(self, action: Action) -> Ship {
        match action.action {
            'L' => self.turn(-(action.times as i32)),
            'R' => self.turn(action.times as i32),
            'F' => self.move_forward(action.times),
            _ => self.move_in_direction(CardinalDirection::new(action.action), action.times),
        }
    }
}

fn main() {
    let directions: Vec<Action> = BufReader::new(File::open("input_test.txt").unwrap())
        .lines()
        .map(|value| {
            let chars = value.unwrap();
            let (action, times) = chars.split_at(1);
            Action {
                action: action.chars().next().unwrap(),
                times: times.parse::<u32>().unwrap(),
            }
        })
        .collect();
    let final_state = directions.into_iter().fold(Ship::new(), |ship, action| {
        ship.apply_action(action)
    });
    println!("Final ship state: {:?}", final_state);
    println!(
        "Result: {}",
        final_state.position.0.abs() + final_state.position.1.abs()
    );
}
