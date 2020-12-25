use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn part_1(lines: Vec<String>) -> u32 {
    let rules_lines = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| line.splitn(2, ": ").nth(1).unwrap())
        .map(|line| {
            line.splitn(2, " or ")
                .map(|pair| {
                    let mut splitted = pair.splitn(2, "-");
                    (
                        splitted.next().unwrap().parse::<u32>().unwrap(),
                        splitted.next().unwrap().parse::<u32>().unwrap(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .fold(vec![], |acc, value| {
            acc.into_iter().chain(value.into_iter()).collect::<Vec<_>>()
        });
    let result = lines
        .into_iter()
        .skip_while(|line| line != "nearby tickets:")
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|value| value.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .fold(vec![], |acc, value| {
            acc.into_iter().chain(value.into_iter()).collect::<Vec<_>>()
        })
        .into_iter()
        .filter(|value| {
            for restriction in &rules_lines {
                if *value >= restriction.0 && *value <= restriction.1 {
                    return false;
                }
            }
            true
        })
        .sum();
    result
}

fn filter_valid_values(
    all_tickets: Vec<Vec<u32>>,
    rules: HashMap<&str, Vec<(u32, u32)>>,
) -> Vec<Vec<u32>> {
    all_tickets
        .into_iter()
        .filter(|values| {
            values.into_iter().all(|value| {
                rules.iter().any(|(_rule_name, rules)| {
                    for restriction in rules {
                        if *value >= restriction.0 && *value <= restriction.1 {
                            return true;
                        }
                    }
                    false
                })
            })
        })
        .collect::<Vec<_>>()
}

fn remove_index_from_result<'a>(
    map: HashMap<&'a str, Vec<u32>>,
    property: &str,
    value: u32,
) -> HashMap<&'a str, Vec<u32>> {
    map.into_iter()
        .map(|pair| {
            if property == pair.0 {
                (pair.0, pair.1)
            } else {
                (
                    pair.0,
                    pair.1
                        .into_iter()
                        .filter(|index| *index != value)
                        .collect::<Vec<_>>(),
                )
            }
        })
        .collect::<HashMap<_, _>>()
}

fn get_ordered_properties(
    values: Vec<Vec<u32>>,
    rules: HashMap<&str, Vec<(u32, u32)>>,
) -> Vec<&str> {
    let mut possible_properties = (0..values[0].len())
        .map(|_| rules.iter().map(|(key, _value)| *key).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for i in 0..values.len() {
        for j in 0..values[0].len() {
            let current_value = values[i][j];
            for (key, rules_values) in &rules {
                let mut found = false;
                for rule_value in rules_values {
                    if current_value >= rule_value.0 && current_value <= rule_value.1 {
                        found = true;
                    }
                }
                if !found {
                    possible_properties[j] = possible_properties[j]
                        .clone()
                        .into_iter()
                        .filter(|v| v != key)
                        .collect::<Vec<_>>();
                }
            }
        }
    }
    let mut partial_result: HashMap<&str, Vec<u32>> = HashMap::new();
    possible_properties
        .iter()
        .enumerate()
        .for_each(|(index, vector)| {
            vector
                .iter()
                .for_each(|property| match partial_result.get_mut(*property) {
                    Some(ref mut indices) => {
                        indices.push(index as u32);
                    }
                    None => {
                        partial_result.insert(property, vec![index as u32]);
                    }
                })
        });
    let mut checked_properties: HashSet<&str> = HashSet::new();
    while partial_result.iter().any(|pair| pair.1.len() != 1) {
        for pair in partial_result.clone() {
            if pair.1.len() == 1 && !checked_properties.contains(pair.0) {
                partial_result =
                    remove_index_from_result(partial_result.clone(), pair.0, pair.1[0]);
                checked_properties.insert(pair.0);
                break;
            }
        }
    }
    partial_result
        .into_iter()
        .map(|pair| (pair.1[0], pair.0))
        .collect::<BTreeMap<_, _>>()
        .into_iter()
        .map(|pair| pair.1)
        .collect::<Vec<_>>()
}

fn part_2(lines: Vec<String>) -> u64 {
    let rules = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| line.splitn(2, ": "))
        .map(|mut splitted| {
            let title = splitted.next().unwrap();
            let values = splitted
                .next()
                .unwrap()
                .splitn(2, " or ")
                .map(|pair| {
                    let mut splitted = pair.splitn(2, "-");
                    (
                        splitted.next().unwrap().parse::<u32>().unwrap(),
                        splitted.next().unwrap().parse::<u32>().unwrap(),
                    )
                })
                .collect::<Vec<_>>();
            (title, values)
        })
        .collect::<HashMap<_, _>>();
    let your_ticket = lines
        .iter()
        .skip_while(|line| *line != "your ticket:")
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(|number| number.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let all_tickets = lines
        .iter()
        .skip_while(|line| *line != "nearby tickets:")
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|value| value.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let filtered_values = filter_valid_values(all_tickets, rules.clone());
    let ordered_properties = get_ordered_properties(filtered_values, rules);
    let mut count = 0;
    let mut result: u64 = 1;
    for (index, property) in ordered_properties.into_iter().enumerate() {
        if property.starts_with("departure") {
            result *= your_ticket[index] as u64;
            count += 1;
            if count >= 6 {
                break;
            }
        }
    }
    result
}

fn main() {
    let lines = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|value| value.unwrap())
        .collect::<Vec<_>>();
    let result = part_1(lines.clone());
    println!("Result: {}", result);
    let result = part_2(lines.clone());
    println!("Result: {}", result);
}
