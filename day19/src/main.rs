use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct IntermediateRule {
    first_part: Vec<u32>,
    second_part: Option<Vec<u32>>,
}

#[derive(Debug)]
enum Rule {
    Final(char),
    Intermediate(IntermediateRule),
}

type Rules = HashMap<u32, Rule>;

fn main() {
    let lines = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .into_iter()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let result = part_1(&lines);
    println!("Part 1 result: {}", result);
    let result = part_2(&lines);
    println!("Part 2 result: {}", result);
}

fn part_1(lines: &Vec<String>) -> u32 {
    let rules = parse_rules(lines);
    let built_strings = build_strings_from_rules(&rules);
    let parsed_messages = parse_messages(lines);
    parsed_messages
        .into_iter()
        .filter(|message| built_strings.contains(message))
        .count() as u32
}

fn part_2(lines: &Vec<String>) -> u32 {
    let rules = parse_rules(lines);
    let built_strings = build_strings_from_rules(&rules);
    let parsed_messages = parse_messages(lines);
    // These are the strings that can be looped
    let strings_42 = build_strings_from_rule(rules.get(&42).unwrap(), &rules);
    let strings_31 = build_strings_from_rule(rules.get(&31).unwrap(), &rules);
    parsed_messages
        .into_iter()
        .filter(|message| {
            built_strings.contains(message) || check_loops(&message, &strings_42, &strings_31)
        })
        .count() as u32
}

fn check_loops(
    message: &str,
    beggining_patterns: &Vec<String>,
    ending_patterns: &Vec<String>,
) -> bool {
    let mut message = message;
    let mut ending_matches = 0;
    let mut begining_matches = 0;
    loop {
        match ending_patterns
            .into_iter()
            .find(|string| message.ends_with(*string))
        {
            Some(string) => {
                message = message.split_at(message.len() - string.len()).0;
                ending_matches += 1;
            }
            None => break,
        }
    }
    if ending_matches == 0 {
        return false;
    }
    loop {
        match beggining_patterns
            .into_iter()
            .find(|string| message.starts_with(*string))
        {
            Some(string) => {
                message = message.split_at(string.len()).1;
                begining_matches += 1;
            }
            None => break,
        }
    }
    begining_matches > ending_matches && message.is_empty()
}

fn parse_messages(lines: &Vec<String>) -> Vec<String> {
    lines
        .clone()
        .into_iter()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .collect::<Vec<_>>()
}

fn parse_rules(lines: &Vec<String>) -> Rules {
    lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| parse_rule(line.as_str()))
        .collect::<Rules>()
}

fn parse_rule(line: &str) -> (u32, Rule) {
    let mut splitted = line.split(": ");
    let index = splitted
        .next()
        .map(|number| number.parse::<u32>().unwrap())
        .unwrap();
    let rule_str = splitted.next().unwrap();
    let rule = if rule_str.chars().next().unwrap() == '\"' {
        Rule::Final(rule_str.chars().nth(1).unwrap())
    } else {
        parse_intermediate_rule(rule_str)
    };
    (index, rule)
}

fn parse_intermediate_rule(rule_str: &str) -> Rule {
    let mut rule_splitted = rule_str.split(" | ");
    let first_part = rule_splitted
        .next()
        .unwrap()
        .split(" ")
        .map(|number| number.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let second_part = rule_splitted.next().map(|line| {
        line.split(" ")
            .map(|number| number.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    });
    Rule::Intermediate(IntermediateRule {
        first_part,
        second_part,
    })
}

fn build_strings_from_rules(rules: &Rules) -> HashSet<String> {
    let first_rule = rules.get(&0).unwrap();
    let strings = build_strings_from_rule(first_rule, rules);
    strings.into_iter().collect::<_>()
}

fn build_strings_from_rule(rule: &Rule, rules: &Rules) -> Vec<String> {
    match rule {
        Rule::Final(value) => vec![format!("{}", value)],
        Rule::Intermediate(intermediate_rule) => {
            let mut result = build_strings_from_intermediate(&intermediate_rule.first_part, rules);
            if intermediate_rule.second_part.is_some() {
                result = result
                    .into_iter()
                    .chain(build_strings_from_intermediate(
                        intermediate_rule.second_part.as_ref().unwrap(),
                        rules,
                    ))
                    .collect::<Vec<_>>();
            }
            result
        }
    }
}

fn build_strings_from_intermediate(indices: &Vec<u32>, rules: &Rules) -> Vec<String> {
    let results = indices
        .iter()
        .map(|index| build_strings_from_rule(rules.get(index).unwrap(), rules))
        .collect::<Vec<_>>();
    results.into_iter().fold(vec![], |acc, current_value| {
        if acc.is_empty() {
            current_value
        } else {
            let mut result = vec![];
            for acc in acc {
                for cv in &current_value {
                    result.push(format!("{}{}", acc, cv));
                }
            }
            result
        }
    })
}
