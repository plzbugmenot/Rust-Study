use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();

    loop {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let parts: Vec<u32> = input
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if parts.len() != 2 {
            println!("Please enter two numbers.");
            continue;
        }

        let a = parts[0];
        let b = parts[1];

        if a == 0 && b == 0 {
            break;
        }

        let (min, max) = if a < b { (a, b) } else { (b, a) };

        let random_number = rng.gen_range(min..=max);
        println!("{}", random_number);
    }
}
