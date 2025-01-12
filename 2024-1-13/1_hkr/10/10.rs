use std::io;

fn main() {
    println!("Enter the value of N:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid positive integer.");
            return;
        }
    };

    let triplets = find_pythagorean_triplets(n);
    
    if triplets.is_empty() {
        println!("No Pythagorean triplets found for N = {}.", n);
    } else {
        for triplet in triplets {
            println!("{}: {{ {}, {}, {} }}", n, triplet.0, triplet.1, triplet.2);
        }
    }
}

fn find_pythagorean_triplets(n: u32) -> Vec<(u32, u32, u32)> {
    let mut triplets = Vec::new();
    
    for a in 1..=n / 3 {
        for b in (a + 1)..=n / 2 {
            let c = n - a - b;
            if a < b && b < c && a * a + b * b == c * c {
                triplets.push((a, b, c));
            }
        }
    }

    triplets
}
