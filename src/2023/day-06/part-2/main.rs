use std::fs;

const INPUT: &str = "input";

fn main() {
    let input = fs::read_to_string(INPUT)
        .expect("Failed to read input file");

    let time: f64 = input.lines()
        .nth(0).unwrap()
        .split(":").nth(1).unwrap()
        .replace(char::is_whitespace, "")
        .parse::<f64>().unwrap();

    let distance: f64 = input.lines()
        .nth(1).unwrap()
        .split(":").nth(1).unwrap()
        .replace(char::is_whitespace, "")
        .parse::<f64>().unwrap();

    let (mut root1, mut root2) = calculate_roots(-1.0, time, -distance);

    // As we want the result to be greater than distance (not greater or equal),
    // we need to increase/decrease the bounds when the roots are integers.
    root1 += if root1.fract() == 0.0 { 1.0 } else { 0.0 };
    root2 -= if root2.fract() == 0.0 { 1.0 } else { 0.0 };

    println!(
        "The number of ways you can beat the record in the race is: {}",
        root2.floor() - root1.ceil() + 1.0
    );
}


// We can use the quadratic formula to calculate the roots of a quadratic equation,
// cause (time - hold) * hold > distance is a quadratic equation.
fn calculate_roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = b.powi(2) - 4.0 * a * c;
    let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
    (root1, root2)
}
