use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::cmp::Ordering;

#[derive(Eq)]
struct BusData((u32, u32));

impl PartialOrd for BusData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BusData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for BusData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

fn get_first_bus(time_to_leave: u32, buses: &Vec<u32>) -> BusData {
    buses
        .into_iter()
        .map(|bus_id| {
            let mut current_time_to_leave = *bus_id;
            while current_time_to_leave < time_to_leave {
                current_time_to_leave += bus_id;
            }
            BusData((current_time_to_leave, *bus_id))
        })
        .min()
        .unwrap()
}

fn main() {
    let lines = BufReader::new(File::open("input_test.txt").unwrap())
        .lines()
        .into_iter()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let time_to_leave = lines[0].parse::<u32>().unwrap();
    let buses = lines[1]
        .split(',')
        .filter(|bus_id| *bus_id != "x")
        .map(|bus_id_str| bus_id_str.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let bus = get_first_bus(time_to_leave, &buses);
    let result = bus.0.1 * (bus.0.0 - time_to_leave);
    println!("Result: {}", result);
}
