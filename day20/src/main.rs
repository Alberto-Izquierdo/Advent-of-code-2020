use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Map {
    width: u32,
    height: u32,
    tiles: Vec<Tile>,
}

struct Tile {
    id: String,
    data: Vec<bool>,
    width: usize,
    rotation_degrees: u32,
    flipped: (bool, bool),
    position: Option<(i32, i32)>,
}

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Tile {
    fn new(id: &str) -> Self {
        Tile {
            id: id.to_string(),
            data: vec![],
            width: 0,
            rotation_degrees: 0,
            flipped: (false, false),
            position: None,
        }
    }

    fn get_value(&self, x: usize, y: usize) -> bool {
        let x = if !self.flipped.0 {
            x
        } else {
            (self.width - 1) - x
        };
        let y = if !self.flipped.1 {
            y
        } else {
            (self.width - 1) - y
        };
        let final_x = match self.rotation_degrees {
            0 => x,
            90 => (y),
            180 => (self.width - 1 - x),
            270 => (self.width - 1 - y),
            _ => panic!("Wrong rotation"),
        };
        let final_y = match self.rotation_degrees {
            0 => y,
            90 => (self.width - 1 - x),
            180 => (self.width - 1 - y),
            270 => (x),
            _ => panic!("Wrong rotation"),
        };
        self.data[final_x + final_y * self.width]
    }

    fn flip_x(&mut self) {
        self.flipped.0 = !self.flipped.0;
    }

    fn flip_y(&mut self) {
        self.flipped.1 = !self.flipped.1;
    }

    fn rotate_right(&mut self) {
        let new_rotation = (self.rotation_degrees + 90) % 360;
        self.rotation_degrees = new_rotation;
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nTile: {}", self.id)?;
        for i in 0..self.width {
            write!(f, "\n")?;
            for j in 0..self.width {
                let value = if self.get_value(j, i) { "#" } else { "." };
                write!(f, "{}", value)?;
            }
        }
        write!(f, "\n")
    }
}

fn main() {
    let lines = BufReader::new(File::open("input_test.txt").unwrap())
        .lines()
        .into_iter()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let tiles = load_tiles(&lines);
    println!("{:?}", tiles);
}

fn load_tiles(lines: &Vec<String>) -> Vec<Tile> {
    lines.iter().fold(vec![], |mut tiles, line| {
        if !line.is_empty() {
            if line.contains(":") {
                let id = line
                    .split(" ")
                    .nth(1)
                    .unwrap()
                    .split(":")
                    .next()
                    .unwrap()
                    .to_string();
                tiles.push(Tile::new(&id));
            } else {
                let last_data = tiles.last().unwrap().data.clone();
                tiles.last_mut().unwrap().data = last_data
                    .into_iter()
                    .chain(line.chars().map(|ch| match ch {
                        '.' => false,
                        '#' => true,
                        _ => panic!(),
                    }))
                    .collect::<Vec<_>>();
                tiles.last_mut().unwrap().width = line.len();
            }
        }
        tiles
    })
}

fn are_tiles_connected(lhs: &Tile, rhs: &Tile) -> Option<Vec<Direction>> {
    let mut result = vec![];
    // UP
    if (0..lhs.width).all(|x| lhs.get_value(x, 0) == rhs.get_value(x, rhs.width - 1)) {
        result.push(Direction::UP);
    }
    // DOWN
    if (0..lhs.width).all(|x| lhs.get_value(x, rhs.width - 1) == rhs.get_value(x, 0)) {
        result.push(Direction::DOWN);
    }
    // LEFT
    if (0..lhs.width).all(|y| lhs.get_value(0, y) == rhs.get_value(rhs.width - 1, y)) {
        result.push(Direction::LEFT);
    }
    // RIGHT
    if (0..lhs.width).all(|y| lhs.get_value(rhs.width - 1, y) == rhs.get_value(0, y)) {
        result.push(Direction::RIGHT);
    }
    if !result.is_empty() {
        Some(result)
    } else {
        None
    }
}
