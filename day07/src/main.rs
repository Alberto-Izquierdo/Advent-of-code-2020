use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

fn build_map(lines: Lines<BufReader<File>>) -> std::collections::HashMap<String, Vec<String>> {
    // shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    lines
        .map(|line| {
            let line_unwrapped = line.unwrap();
            let splitted = line_unwrapped.split(" contain ");
            // shiny gold bags|1 dark olive bag, 2 vibrant plum bags.
            let a = splitted
                .clone()
                .next()
                .unwrap()
                .strip_suffix(" bags")
                .unwrap();
            // shiny gold
            let b = splitted.skip(1).next().unwrap().split(", ");
            // 1 dark olive bag|2 vibrant plum bags.
            let v = b
                .map(|bag_info| {
                    /*
                    let quantity = String::from(bag_info.chars().next().unwrap())
                        .parse::<u32>()
                        .unwrap();
                        */
                    let bag_info = bag_info.split_at(2).1;
                    // dark olive bag
                    let color_index = bag_info.rfind(" bag").unwrap();
                    let color = bag_info.split_at(color_index).0;
                    String::from(color)
                })
                .collect::<Vec<String>>();
            (String::from(a), v)
        })
        .collect::<HashMap<String, Vec<String>>>()
}

const SHINY_GOLD: &str = "shiny gold";

fn entry_contains_shinny_gold(vector: &Vec<String>, map: &HashMap<String, Vec<String>>) -> bool {
    if vector.contains(&String::from(SHINY_GOLD)) {
        return true;
    } else {
        for entry in vector {
            if match map.get(entry) {
                Some(value) => entry_contains_shinny_gold(value, &map),
                None => false,
            } {
                return true;
            }
        }
    }
    return false;
}

fn process_map(map: &std::collections::HashMap<String, Vec<String>>) -> u32 {
    map.iter()
        .filter(|pair| entry_contains_shinny_gold(pair.1, &map))
        .count() as u32
}

// SECOND PART

fn build_map2(
    lines: Lines<BufReader<File>>,
) -> std::collections::HashMap<String, Vec<(u32, String)>> {
    // shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    lines
        .map(|line| {
            let line_unwrapped = line.unwrap();
            let splitted = line_unwrapped.split(" contain ");
            // shiny gold bags|1 dark olive bag, 2 vibrant plum bags.
            let a = splitted
                .clone()
                .next()
                .unwrap()
                .strip_suffix(" bags")
                .unwrap();
            // shiny gold
            let b = splitted.skip(1).next().unwrap().split(", ");
            // 1 dark olive bag|2 vibrant plum bags.
            let v = b
                .filter(|bag_info| {
                    String::from(bag_info.chars().next().unwrap())
                        .parse::<u32>()
                        .is_ok()
                })
                .map(|bag_info| {
                    let quantity = String::from(bag_info.chars().next().unwrap())
                        .parse::<u32>()
                        .unwrap();
                    let bag_info = bag_info.split_at(2).1;
                    // dark olive bag
                    let color_index = bag_info.rfind(" bag").unwrap();
                    let color = bag_info.split_at(color_index).0;
                    (quantity, String::from(color))
                })
                .collect::<Vec<(u32, String)>>();
            (String::from(a), v)
        })
        .collect::<HashMap<String, Vec<(u32, String)>>>()
}

fn get_number_of_bags_inside(color: &str, map: &HashMap<String, Vec<(u32, String)>>) -> u32 {
    let v = map.get(color).unwrap();
    let q: u32 = v
        .iter()
        .map(|(quantity, color)| quantity * get_number_of_bags_inside(color, map))
        .sum();
    q + 1
}

fn get_number_of_bags_inside_shiny_gold(map: &HashMap<String, Vec<(u32, String)>>) -> u32 {
    get_number_of_bags_inside(SHINY_GOLD, map) - 1
}

fn main() {
    let lines = BufReader::new(File::open("input.txt").unwrap()).lines();
    let map = build_map2(lines);
    println!("{}", get_number_of_bags_inside_shiny_gold(&map));
}
