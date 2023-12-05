use std::fs;
use std::collections::HashSet;

const INPUT: &str = "input";

fn main() {
    let input = fs::read_to_string(INPUT)
        .expect("Failed to read input file");

    let mut cards_summary: Vec<(usize, i32)> = input
        .lines()
        .map(|line| {
            let numbers: &str = line.splitn(2, ":").collect::<Vec<_>>()[1];
            let chunks = numbers.splitn(2, "|").collect::<Vec<_>>();
            let winning = chunks[0];
            let hand = chunks[1];
            (matches(winning, hand), 1)
        }).collect();

    for i in 0..cards_summary.len() {
        for j in 1..cards_summary[i].0 + 1 {
            let next = i + j as usize;
            if next == cards_summary.len() {
                break;
            }
            cards_summary[next].1 += cards_summary[i].1;
        }
    }

    let sum: i32 = cards_summary.iter().map(|s| s.1).sum();
    println!("The total scratchcards you end up with is: {}", sum);
}

fn matches(winning: &str, hand: &str) -> usize {
    let set: HashSet<i32> = winning
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    return hand.split_whitespace().filter(|s| {
        let value: i32 = s.parse().unwrap();
        return set.contains(&value);
    }).count();
}