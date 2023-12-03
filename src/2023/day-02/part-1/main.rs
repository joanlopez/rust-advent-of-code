use std::fs;
use regex::Regex;
use std::collections::HashMap;

const INPUT: &str = "input";

fn main() {
    let input = fs::read_to_string(INPUT)
        .expect("Failed to read input file");

    let numbers: Vec<i32> = input
        .lines()
        .filter_map(|line| {
            let id = Regex::new(r"Game\s+(\d+)").unwrap().captures(line).unwrap().get(1)?.as_str().parse::<i32>().unwrap();
            let counts = str_to_map(line.splitn(2, ":").collect::<Vec<_>>()[1]);

            match satisfy_rules(counts) {
                true => Some(id),
                _ => None,
            }
        })
        .collect();

    let sum: i32 = numbers.iter().sum();
    println!("The sum of the IDs of games that satisfy the rules is: {}", sum);
}

fn str_to_map(s: &str) -> HashMap<String, i32> {
    let re = Regex::new(r"(\d+)\s+([a-z]+)").unwrap();
    let mut counts: HashMap<String, i32> = HashMap::new();

    for round in s.split(";") {
        for cap in re.captures_iter(round) {
            let count: i32 = cap[1].parse().unwrap_or(0);
            let color = &cap[2];

            if !counts.contains_key(color) ||
                (counts.contains_key(color) && counts.get(color).unwrap() < &count) {
                counts.insert(color.to_string(), count);
            }
        }
    }

    return counts;
}

fn satisfy_rules(counts: HashMap<String, i32>) -> bool {
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    for (color, count) in counts {
        if color != "red" && color != "green" && color != "blue" {
            return false;
        }

        if color == "red" && count > 12 {
            return false;
        }

        if color == "green" && count > 13 {
            return false;
        }

        if color == "blue" && count > 14 {
            return false;
        }
    }

    return true;
}