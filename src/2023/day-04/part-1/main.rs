use std::fs;
use std::collections::HashSet;

const INPUT: &str = "input";

fn main() {
    let input = fs::read_to_string(INPUT)
        .expect("Failed to read input file");

    let sum: i32 = input
        .lines()
        .map(|line| {
            let numbers: &str = line.splitn(2, ":").collect::<Vec<_>>()[1];

            let chunks = numbers.splitn(2, "|").collect::<Vec<_>>();
            let winning = chunks[0];
            let hand = chunks[1];

            card_points(winning, hand)
        }).reduce(|a, b| a + b).unwrap();

    println!("They are worth {} points", sum);
}

fn card_points(winning: &str, hand: &str) -> i32 {
    let set: HashSet<i32> = winning
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut points = 0;
    hand.split_whitespace().for_each(|s| {
        let value: i32 = s.parse().unwrap();
        if set.contains(&value) {
            if points == 0 {
                points = 1;
            } else {
                points *= 2;
            }
        }
    });

    return points;
}