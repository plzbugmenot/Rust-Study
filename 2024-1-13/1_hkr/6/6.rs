use std::io;

fn is_armstrong_number(number: u32) -> bool {
    let digits: Vec<u32> = number.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let num_digits = digits.len() as u32;
    let sum_of_powers: u32 = digits.iter().map(|&d| d.pow(num_digits)).sum();
    sum_of_powers == number
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_cases: usize = input.trim().parse().expect("Please enter a valid number");

    for _ in 0..num_cases {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let number: u32 = input.trim().parse().expect("Please enter a valid number");

        if is_armstrong_number(number) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
