use std::io;

fn collatz_steps(mut n: u32) -> u32 {
    let mut steps = 0;

    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        steps += 1;
    }

    steps
}

fn main() {
    println!("Enter a positive integer:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid positive integer.");
            return;
        }
    };

    let steps = collatz_steps(n);
    println!("The number of steps to reach 1 from {} is {}.", n, steps);
}
