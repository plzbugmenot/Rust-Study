fn calc(x:i64) -> i64 { // This is comment
  return (1..=x).product();
}

fn main() {
  let x = 3;
  let y = calc(x);
  println!("The value of y is: {}", y);
}