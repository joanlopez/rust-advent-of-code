use std::fs;
use std::cmp;
use std::convert::TryInto;

const INPUT: &str = "input";

fn main() {
    let input = fs::read_to_string(INPUT)
        .expect("Failed to read input file");

    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut part_numbers: Vec<i32> = Vec::new();

    for (i, row) in matrix.iter().enumerate() {
        let last = row.len() - 1;
        let mut part_number_str = String::new();
        for (j, col) in row.iter().enumerate() {
            // If number, compile string
            if is_digit(*col) {
                part_number_str.push(*col);
            }

            // If last or next is not a digit, check if part number and reset
            let next = matrix[i][cmp::min(last, j + 1)];
            if j == last || !is_digit(next) {
                if is_part_number(&matrix, &part_number_str, j, i) {
                    part_numbers.push(part_number_str.parse::<i32>().unwrap());
                }
                part_number_str = String::new();
            }
        }
    }


    let sum: i32 = part_numbers.iter().sum();
    println!("The sum of all of the part numbers in the engine schematic is: {}", sum);
}

fn is_part_number(matrix: &Vec<Vec<char>>, part_number: &String, x: usize, y: usize) -> bool {
    if part_number.len() == 0 {
        return false;
    }

    let from_x = cmp::max(x as i32 - <usize as TryInto<i32>>::try_into(part_number.len()).unwrap(), 0) as usize;
    let to_x = cmp::min(x as i32 + 1, (matrix[0].len() - 1).try_into().unwrap()) as usize;

    let from_y = cmp::max(y as i32 - 1, 0) as usize;
    let to_y = cmp::min(y as i32 + 1, (matrix.len() - 1).try_into().unwrap()) as usize;

    // First we check beginning and end
    if is_symbol(matrix[y][from_x]) || is_symbol(matrix[y][to_x]) {
        return true;
    }

    // Then, we check the top and the bottom
    for x in from_x..to_x + 1 {
        if is_symbol(matrix[from_y][x]) || is_symbol(matrix[to_y][x]) {
            return true;
        }
    }

    return false;
}

fn is_symbol(c: char) -> bool {
    return !is_digit(c) && c != '.';
}

fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}