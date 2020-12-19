use primes::{PrimeSet, Sieve};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

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

fn get_prime_factorization(number: usize) -> HashMap<usize, usize> {
    let mut pset = Sieve::new();
    let mut partial_result = number;
    let mut result = HashMap::new();
    while !primes::is_prime(partial_result as u64) && partial_result != 1 {
        let prime = pset
            .iter()
            .skip_while(|value| partial_result % *value as usize != 0)
            .next()
            .unwrap() as usize;
        if result.contains_key(&prime) {
            result.insert(prime, result[&prime] + 1);
        } else {
            result.insert(prime, 1);
        }
        partial_result = partial_result / prime;
    }
    if result.contains_key(&partial_result) {
        result.insert(partial_result, result[&partial_result] + 1);
    } else {
        result.insert(partial_result, 1);
    }
    result
}

fn get_least_common_multiple(numbers: Vec<usize>) -> usize {
    let prime_numbers = numbers
        .into_iter()
        .map(|number| get_prime_factorization(number))
        .collect::<Vec<_>>();
    let mut partial_result = HashMap::new();
    for map in prime_numbers {
        for pair in map {
            if !partial_result.contains_key(&pair.0) || partial_result[&pair.0] < pair.1 {
                partial_result.insert(pair.0, pair.1);
            }
        }
    }
    partial_result
        .into_iter()
        .fold(1, |acc, pair| acc * pair.0.pow(pair.1 as u32))
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

fn get_first_time_that_matches_the_sequence(buses: Vec<Option<usize>>) -> usize {
    let vector = buses
        .iter()
        .enumerate()
        .filter(|pair| pair.1.is_some())
        .map(|pair| (pair.0, pair.1.unwrap()))
        .collect::<Vec<_>>();
    let mut time = 1;
    let mut iteration = 1;
    for (offset, bus) in vector {
        while ((time + offset) % bus) != 0 {
            time += iteration;
        }
        iteration = get_least_common_multiple(vec![iteration, bus]);
    }
    return time;
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

    let lines = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .into_iter()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let buses = lines[1]
        .split(',')
        .map(|bus_id_str| {
            if bus_id_str == "x" {
                None
            } else {
                Some(bus_id_str.parse::<usize>().unwrap())
            }
        })
        .collect::<Vec<Option<usize>>>();
    let result = get_first_time_that_matches_the_sequence(buses);
    println!("Result: {}", result);
}
