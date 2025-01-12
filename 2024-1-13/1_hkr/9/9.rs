use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    println!("Enter the number of test cases:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let test_cases: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    let mut phrases = Vec::new();

    for _ in 0..test_cases {
        println!("Enter a phrase:");

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        phrases.push(input.trim().to_string());
    }

    for phrase in phrases {
        let acronym = generate_acronym(&phrase);
        println!("{}", acronym);
    }
}

fn generate_acronym(phrase: &str) -> String {
    phrase.split(|c: char| c.is_whitespace() || c == '-')
        .filter(|word| !word.is_empty())
        .map(|word| word.chars().next().unwrap().to_ascii_uppercase())
        .collect()
}
