use std::f64::consts::PI;
use std::io::{self, BufRead};


fn calculate_area_and_perimeter(dimensions: (u32, u32)) -> (f64, f64) {
    let (width, height) = dimensions;
    let diameter = width.min(height) as f64;
    let radius = diameter / 2.0;
    let area = PI * radius * radius;
    let perimeter = 2.0 * PI * radius;
    (area, perimeter)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let num_tests: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..num_tests {
        if let Some(Ok(line)) = lines.next() {
            let numbers: Vec<u32> = line.split_whitespace()
                                        .map(|s| s.parse().unwrap())
                                        .collect();
            let dimensions = (numbers[0], numbers[1]);
            let (area, perimeter) = calculate_area_and_perimeter(dimensions);
            println!("{:.2} {:.2}", area, perimeter);
        }
    }
}
