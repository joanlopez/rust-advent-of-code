use std::fs;
use std::cmp;
use std::collections::HashMap;
use std::convert::TryInto;

const INPUT: &str = "input";

fn main() {
    let input = fs::read_to_string(INPUT)
        .expect("Failed to read input file");

    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut gear_ratios: Vec<i32> = Vec::new();
    let mut adjacent: HashMap<String, Vec<i32>> = HashMap::new();

    for (i, row) in matrix.iter().enumerate() {
        let last = row.len() - 1;
        let mut part_number_str = String::new();
        for (j, col) in row.iter().enumerate() {
            // If number, compile string
            if is_digit(*col) {
                part_number_str.push(*col);
            }

            // If last or next is not a digit, compute gears
            let next = matrix[i][cmp::min(last, j + 1)];
            if j == last || !is_digit(next) {
                computer_adjacent(&mut adjacent, &matrix, &part_number_str, j, i);
                part_number_str = String::new();
            }
        }
    }

    adjacent.keys().for_each(|key| {
        if adjacent[key].len() == 2 {
            gear_ratios.push(adjacent[key][0] * adjacent[key][1]);
        }
    });


    let sum: i32 = gear_ratios.iter().sum();
    println!("The sum of all of the gear ratios in the engine schematic is: {}", sum);
}

fn computer_adjacent(adjacent: &mut HashMap<String, Vec<i32>>, matrix: &Vec<Vec<char>>, part_number: &String, x: usize, y: usize) {
    if part_number.len() == 0 {
        return;
    }

    let as_num = part_number.parse::<i32>().unwrap();

    let from_x = cmp::max(x as i32 - <usize as TryInto<i32>>::try_into(part_number.len()).unwrap(), 0) as usize;
    let to_x = cmp::min(x as i32 + 1, (matrix[0].len() - 1).try_into().unwrap()) as usize;

    let from_y = cmp::max(y as i32 - 1, 0) as usize;
    let to_y = cmp::min(y as i32 + 1, (matrix.len() - 1).try_into().unwrap()) as usize;

    let mut adjacent_to: Vec<String> = Vec::new();

    if is_gear(matrix[y][from_x]) {
        adjacent_to.push(to_id(from_x, y));
    }

    if is_gear(matrix[y][to_x]) {
        adjacent_to.push(to_id(to_x, y));
    }

    for x in from_x..to_x + 1 {
        if is_gear(matrix[from_y][x]) {
            adjacent_to.push(to_id(x, from_y));
        }

        if is_gear(matrix[to_y][x]) {
            adjacent_to.push(to_id(x, to_y));
        }
    }

    for id in adjacent_to {
        if adjacent.contains_key(&id) {
            adjacent.get_mut(&id).unwrap().push(as_num);
        } else {
            adjacent.insert(id, vec![as_num]);
        }
    }
}

fn to_id(x: usize, y: usize) -> String {
    return format!("{}-{}", x, y);
}

fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}

fn is_gear(c: char) -> bool {
    return c == '*';
}