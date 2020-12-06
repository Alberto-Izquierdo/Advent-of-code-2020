use regex::Regex;
use std::io::{prelude::*, BufReader, Lines};
use std::{collections::HashMap, fs::File};

struct Passport {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: u32,
    cid: Option<String>,
}

fn build_passport(map: &HashMap<String, String>) -> Option<Passport> {
    let height_regex: Regex = Regex::new(r"^([0-9]+)(cm|in)$").unwrap();
    let color_regex: Regex = Regex::new(r"^#([0-9|a-f]{6})$").unwrap();
    let eye_color_regex: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    Some(Passport {
        byr: map
            .get("byr")
            .map(|string| string.parse::<u32>().ok())?
            .filter(|value| *value >= 1920 && *value <= 2002)?,
        iyr: map
            .get("iyr")
            .map(|string| string.parse::<u32>().ok())?
            .filter(|value| *value >= 2010 && *value <= 2020)?,
        eyr: map
            .get("eyr")
            .map(|string| string.parse::<u32>().ok())?
            .filter(|value| *value >= 2020 && *value <= 2030)?,
        hgt: map
            .get("hgt")
            .map(|height| {
                height_regex.captures(height).map(|groups| {
                    groups[1].parse::<u32>().ok().map(|value| match &groups[2] {
                        "cm" => {
                            if value < 150 || value > 193 {
                                Err("Invalid height")
                            } else {
                                Ok(height)
                            }
                        }
                        "in" => {
                            if value < 59 || value > 76 {
                                Err("Invalid height")
                            } else {
                                Ok(height)
                            }
                        }
                        _ => {
                            Err("Something terrible happened")
                        }
                    })
                })
            })???
            .ok()?
            .clone(),
        hcl: map
            .get("hcl")
            .filter(|value| color_regex.is_match(value))?
            .clone(),
        ecl: map
            .get("ecl")
            .filter(|ecl| ecl.len() == 3 && eye_color_regex.is_match(ecl))?
            .clone(),
        pid: map
            .get("pid")
            .filter(|pid| pid.len() == 9)
            .map(|pid| pid.parse::<u32>().ok())??,
        cid: map.get("cid").map(|cid| cid.clone()),
    })
}

fn read_passports_data(lines: Lines<BufReader<File>>) -> Vec<Passport> {
    let mut result: Vec<Passport> = vec![];
    let mut last_map: HashMap<String, String> = HashMap::new();
    lines.for_each(|line| {
        let line = line.unwrap();
        if line.is_empty() {
            let passport = build_passport(&last_map);
            passport.map(|value| result.push(value));
            last_map.clear();
        } else {
            line.split(" ").for_each(|pair| {
                let values = pair.split(":");
                last_map.insert(
                    String::from(values.clone().next().unwrap()),
                    String::from(values.skip(1).next().unwrap()),
                );
            });
        }
    });
    result
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let result = read_passports_data(lines).len();
    println!("Result: {}", result);
}
