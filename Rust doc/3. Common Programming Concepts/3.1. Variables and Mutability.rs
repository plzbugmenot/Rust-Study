use std::io;

fn main() {
    
    let mut x = 5;
    println!("The value of x is: {}", x);
    
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    x = input.trim().parse().expect("Please type a number!");

    // This is same as above code.
    // x = input.trim().parse() {
    //   Ok(num) => num,
    //     Err(_) => {
    //       panic!("Please type a number!");
    //     }
    // }
    println!("The value of x is updated: {}", x);
}