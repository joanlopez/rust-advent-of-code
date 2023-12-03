use std::fs;
use std::io;

const INPUT: &str = "input";
const NULL: char = '\0';

fn main() {
    let input = fs::read_to_string(INPUT)
        .expect("Failed to read input file");

    let lines = input.lines();
    let mut numbers: Vec<i32> = Vec::new();

    for line in lines {
        let (mut i, mut j): (usize, usize) = (0, line.chars().count() - 1);
        let (mut x, mut y): (char, char) = (NULL, NULL);

        loop {
            // First, we define the escape conditions
            if (x != NULL && y != NULL) || (i == line.chars().count() || j < 0) {
                break;
            }

            if x == NULL {
                let ci = line.chars().nth(i).unwrap();
                if ci.is_digit(10) {
                    x = ci;
                } else {
                    i += 1;
                }
            }

            if y == NULL {
                let ci = line.chars().nth(j).unwrap();
                if ci.is_digit(10) {
                    y = ci;
                } else {
                    j -= 1;
                }
            }
        }

        numbers.push(format!("{}{}", x, y).parse::<i32>().unwrap())
    }

    println!("The sum of all of the calibration values is: {}", numbers.iter().sum::<i32>());
}

fn other() -> Result<(), io::Error> {
    let input = fs::read_to_string(INPUT)?;

    let numbers: Vec<i32> = input
        .lines()
        .filter_map(|line| {

            match  {  }
            
            let first_digit = line.chars().find(|c| c.is_digit(10));
            let last_digit = line.chars().rev().find(|c| c.is_digit(10));

            match (first_digit, last_digit) {
                (Some(x), Some(y)) => Some(format!("{}{}", x, y).parse::<i32>().ok()?),
                _ => None,
            }
        })
        .collect();

    let sum: i32 = numbers.iter().sum();
    println!("The sum of all of the calibration values is: {}", sum);

    Ok(())
}