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
            let counts = str_to_map(line.splitn(2, ":").collect::<Vec<_>>()[1]);
            Some(power(counts))
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

fn power(counts: HashMap<String, i32>) -> i32 {
    let mut power: i32 = 1;

    for (color, count) in counts {
        match color.as_str() {
            "red" => power *= count,
            "green" => power *= count,
            "blue" => power *= count,
            _ => (),
        }
    }

    return power;
}