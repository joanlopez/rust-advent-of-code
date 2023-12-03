use std::fs;

const INPUT: &str = "input";

fn main() {
    let input = fs::read_to_string(INPUT)
        .expect("Failed to read input file");

    let lines = input.lines();
    let mut numbers: Vec<i32> = Vec::new();

    for line in lines {
        let x = find_digit_or_number(line, false).unwrap();
        let y = find_digit_or_number(&line.chars().rev().collect::<String>(), true).unwrap();

        numbers.push(format!("{}{}", x, y).parse::<i32>().unwrap())
    }

    println!("The sum of all of the calibration values is: {}", numbers.iter().sum::<i32>());
}

fn find_digit_or_number(s: &str, rev: bool) -> Option<char> {
    let digit_names = if !rev {
        ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
    } else {
        ["orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"]
    };

    for (index, _) in s.char_indices() {
        let char_at_index = s.chars().nth(index).unwrap();
        if char_at_index.is_digit(10) {
            return Some(char_at_index);
        }

        for &digit_name in &digit_names {
            if s[index..].starts_with(digit_name) {
                if rev {
                    return map_name_to_digit(&(digit_name.chars().rev().collect::<String>()));
                }
                return map_name_to_digit(digit_name);
            }
        }
    }

    None
}

fn map_name_to_digit(name: &str) -> Option<char> {
    match name {
        "zero" => Some('0'),
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None,
    }
}