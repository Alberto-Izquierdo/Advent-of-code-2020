use std::collections::HashMap;
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
    all_values: Vec<Vec<u32>>,
    rules: HashMap<&str, Vec<(u32, u32)>>,
) -> Vec<Vec<u32>> {
    all_values
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

fn classify_values(values: Vec<Vec<u32>>, rules: HashMap<&str, Vec<(u32, u32)>>) -> Vec<Vec<&str>> {
    let all_labels: Vec<&str> = rules.iter().map(|(key, _value)| *key).collect::<Vec<_>>();
    let mut result = (0..values[0].len())
        .map(|_| all_labels.clone())
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
                    result[j] = result[j]
                        .clone()
                        .into_iter()
                        .filter(|v| v != key)
                        .collect::<Vec<_>>();
                }
            }
        }
    }
    result
}

fn part_2(lines: Vec<String>) -> u32 {
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
    let all_values = lines
        .iter()
        .skip_while(|line| *line != "nearby tickets:")
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|value| value.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let filtered_values = filter_valid_values(all_values, rules.clone());
    println!("Filtered values: {:?}", filtered_values);
    let classified_values = classify_values(filtered_values, rules);
    println!("Classified values: {:?}", classified_values);
    0
}

fn main() {
    let lines = BufReader::new(File::open("input_test.txt").unwrap())
        .lines()
        .map(|value| value.unwrap())
        .collect::<Vec<_>>();
    let result = part_1(lines.clone());
    println!("Result: {}", result);
    let result = part_2(lines.clone());
    println!("Result: {}", result);
}
