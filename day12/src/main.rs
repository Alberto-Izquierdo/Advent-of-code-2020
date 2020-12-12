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
    position: (i32, i32),
    waypoint_position: (i32, i32),
}

impl Ship {
    fn new() -> Ship {
        Ship {
            position: (0, 0),
            waypoint_position: (10, 1),
        }
    }

    fn set_position(self, position: (i32, i32)) -> Ship {
        Ship {
            position,
            waypoint_position: self.waypoint_position,
        }
    }

    fn set_waypoint_position(self, waypoint_position: (i32, i32)) -> Ship {
        Ship {
            position: self.position,
            waypoint_position,
        }
    }

    fn move_waypoint_in_direction(
        self,
        cardinal_direction: CardinalDirection,
        distance: u32,
    ) -> Ship {
        let direction = cardinal_direction.get_direction();
        let previous_position = self.waypoint_position;
        self.set_waypoint_position((
            previous_position.0 + direction.0 * distance as i32,
            previous_position.1 + direction.1 * distance as i32,
        ))
    }

    fn move_towards_waypoint(self, distance: u32) -> Ship {
        let distance = (
            self.waypoint_position.0 * distance as i32,
            self.waypoint_position.1 * distance as i32,
        );
        let new_position = (self.position.0 + distance.0, self.position.1 + distance.1);
        self.set_position(new_position)
    }

    fn turn(self, degrees: i32) -> Ship {
        let degrees = if degrees >= 0 {
            degrees % 360
        } else {
            (degrees % 360) + 360
        } as u32;
        let (cos, sin) = match degrees {
            0 => (1, 0),
            90 => (0, 1),
            180 => (-1, 0),
            270 => (0, -1),
            _ => panic!(),
        };
        let new_waypoint_position = (
            cos * self.waypoint_position.0 - sin * self.waypoint_position.1,
            cos * self.waypoint_position.1 + sin * self.waypoint_position.0,
        );
        self.set_waypoint_position((new_waypoint_position.0, new_waypoint_position.0))
    }

    fn apply_action(self, action: Action) -> Ship {
        match action.action {
            'L' => self.turn(action.times as i32),
            'R' => self.turn(-(action.times as i32)),
            'F' => self.move_towards_waypoint(action.times),
            _ => {
                self.move_waypoint_in_direction(CardinalDirection::new(action.action), action.times)
            }
        }
    }
}

fn main() {
    let directions: Vec<Action> = BufReader::new(File::open("input.txt").unwrap())
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
    let final_state = directions
        .into_iter()
        .fold(Ship::new(), |ship, action| ship.apply_action(action));
    println!("Final ship state: {:?}", final_state);
    println!(
        "Result: {}",
        final_state.position.0.abs() + final_state.position.1.abs()
    );
}
