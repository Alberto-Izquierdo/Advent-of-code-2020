use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Scene {
    data: HashSet<(i32, i32, i32)>,
    x_limits: (i32, i32),
    y_limits: (i32, i32),
    z_limits: (i32, i32),
}

struct Scene4D {
    data: HashSet<(i32, i32, i32, i32)>,
    x_limits: (i32, i32),
    y_limits: (i32, i32),
    z_limits: (i32, i32),
    w_limits: (i32, i32),
}

impl Scene {
    fn load(active_positions: Vec<(i32, i32)>) -> Self {
        let limits = active_positions.iter().fold(((0, 0), (0, 0)), |acc, current_value|
            ((i32::min(acc.0.0,current_value.0), i32::max(acc.0.1, current_value.0)),
            (i32::min(acc.1.0, current_value.1), i32::max(acc.1.1, current_value.1)))
        );
        Self {
            data: active_positions
                .iter()
                .map(|pair| (pair.0, pair.1, 0))
                .collect::<HashSet<_>>(),
            x_limits: limits.0,
            y_limits: limits.1,
            z_limits: (0, 0),
        }
    }

    fn new(data: HashSet<(i32, i32, i32)>, limits: ((i32, i32), (i32, i32), (i32, i32))) -> Self {
        Self{data, x_limits : limits.0, y_limits : limits.1, z_limits : limits.2}
    }

    fn get_adjacent_active_cubes(self: &Self, position: (i32, i32, i32)) -> u32 {
        let mut result = 0;
        for x in (position.0 - 1)..(position.0 + 2) {
            for y in (position.1 - 1)..(position.1 + 2) {
                for z in (position.2 - 1)..(position.2 + 2) {
                    if (x, y, z) != position && self.data.contains(&(x, y, z)) {
                        result += 1;
                    }
                }
            }
        }
        result
    }

    fn run_cycle(self: Self) -> Self {
        let mut new_data = HashSet::new();
        let mut limits = ((0, 0), (0, 0), (0, 0));
        for x in (self.x_limits.0 - 1)..(self.x_limits.1 + 2) {
            for y in (self.y_limits.0 - 1)..(self.y_limits.1 + 2) {
                for z in (self.z_limits.0 - 1)..(self.z_limits.1 + 2) {
                    let adjacent_active_cubes = self.get_adjacent_active_cubes((x, y, z));
                    if adjacent_active_cubes == 3 || (self.data.contains(&(x, y, z)) && adjacent_active_cubes == 2) {
                        new_data.insert((x, y, z));
                        limits = ((limits.0.0.min(x), limits.0.1.max(x)), (limits.1.0.min(y), limits.1.1.max(y)), (limits.2.0.min(z), limits.2.1.max(z)))
                    }
                }
            }
        }
        Self::new(new_data, limits)
    }

    fn render(self: &Self) {
        println!();
        for z in (self.z_limits.0)..(self.z_limits.1 + 1) {
            println!("\nz = {}", z);
            for y in (self.y_limits.0)..(self.y_limits.1 + 1) {
                for x in (self.x_limits.0)..(self.x_limits.1 + 1) {
                    if self.data.contains(&(x, y, z)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!()
            }
        }
    }
}

impl Scene4D {
    fn load(active_positions: Vec<(i32, i32)>) -> Self {
        let limits = active_positions.iter().fold(((0, 0), (0, 0)), |acc, current_value|
            ((i32::min(acc.0.0,current_value.0), i32::max(acc.0.1, current_value.0)),
            (i32::min(acc.1.0, current_value.1), i32::max(acc.1.1, current_value.1)))
        );
        Self {
            data: active_positions
                .iter()
                .map(|pair| (pair.0, pair.1, 0, 0))
                .collect::<HashSet<_>>(),
            x_limits: limits.0,
            y_limits: limits.1,
            z_limits: (0, 0),
            w_limits: (0, 0),
        }
    }

    fn new(data: HashSet<(i32, i32, i32, i32)>, limits: ((i32, i32), (i32, i32), (i32, i32), (i32, i32))) -> Self {
        Self{data, x_limits : limits.0, y_limits : limits.1, z_limits : limits.2, w_limits: limits.3}
    }

    fn get_adjacent_active_cubes(self: &Self, position: (i32, i32, i32, i32)) -> u32 {
        let mut result = 0;
        for x in (position.0 - 1)..(position.0 + 2) {
            for y in (position.1 - 1)..(position.1 + 2) {
                for z in (position.2 - 1)..(position.2 + 2) {
                    for w in (position.3 - 1)..(position.3 + 2) {
                        if (x, y, z, w) != position && self.data.contains(&(x, y, z, w)) {
                            result += 1;
                        }
                    }
                }
            }
        }
        result
    }

    fn run_cycle(self: Self) -> Self {
        let mut new_data = HashSet::new();
        let mut limits = ((0, 0), (0, 0), (0, 0), (0, 0));
        for x in (self.x_limits.0 - 1)..(self.x_limits.1 + 2) {
            for y in (self.y_limits.0 - 1)..(self.y_limits.1 + 2) {
                for z in (self.z_limits.0 - 1)..(self.z_limits.1 + 2) {
                    for w in (self.w_limits.0 - 1)..(self.w_limits.1 + 2) {
                        let adjacent_active_cubes = self.get_adjacent_active_cubes((x, y, z, w));
                        if adjacent_active_cubes == 3 || (self.data.contains(&(x, y, z, w)) && adjacent_active_cubes == 2) {
                            new_data.insert((x, y, z, w));
                            limits = ((limits.0.0.min(x), limits.0.1.max(x)), (limits.1.0.min(y), limits.1.1.max(y)), (limits.2.0.min(z), limits.2.1.max(z)), (limits.2.0.min(w), limits.2.1.max(w)))
                        }
                    }
                }
            }
        }
        Self::new(new_data, limits)
    }
}

fn main() {
    let values = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|value| value.unwrap())
        .enumerate()
        .fold(vec![], |acc, (y, current_line)| {
            acc.into_iter()
                .chain(
                    current_line
                        .chars()
                        .enumerate()
                        .filter(|(_, current_char)| *current_char == '#')
                        .map(|(x, _)| (x as i32, y as i32)),
                )
                .collect::<Vec<_>>()
        });
    let final_state = (0..6).fold(Scene::load(values.clone()), |scene, _| {
        scene.run_cycle()
    });
    println!("Result: {}", final_state.data.len());

    let final_state = (0..6).fold(Scene4D::load(values.clone()), |scene, _| {
        scene.run_cycle()
    });
    println!("Result: {}", final_state.data.len());
}
