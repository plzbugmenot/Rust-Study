use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_cases: usize = input.trim().parse().expect("Please enter a valid number");

    for _ in 0..num_cases {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() != 2 {
            println!("Invalid input format");
            continue;
        }

        let number_str = parts[0];
        let base_from: u32 = parts[1].parse().expect("Please enter a valid base");

        let decimal_value = to_decimal(number_str, base_from);

        let base_to = 10;
        let output_str = from_decimal(decimal_value, base_to);

        println!("{}", output_str);
    }
}

fn to_decimal(number_str: &str, base_from: u32) -> u32 {
    let mut decimal_value = 0;
    for (i, ch) in number_str.chars().rev().enumerate() {
        let digit = ch.to_digit(base_from).unwrap();
        decimal_value += digit * base_from.pow(i as u32);
    }
    decimal_value
}

fn from_decimal(mut value: u32, base_to: u32) -> String {
    if value == 0 {
        return "0".to_string();
    }

    let mut digits = Vec::new();
    while value > 0 {
        digits.push((value % base_to).to_string());
        value /= base_to;
    }

    digits.reverse();
    digits.concat()
}
