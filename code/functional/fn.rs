#![feature(core)]
use std::iter::{count,iterate,MultiplicativeIterator};

fn main() {
  let fibonacci = iterate((0,1), |e| {(e.1, e.0 + e.1)}).skip(1).map(|e| e.1);
  let first_10: Vec<_> = fibonacci.take(10).collect();
  println!("first_10: {:?}", first_10);

  let factorial_10 = count(1, 1).take_while(|&i| i <= 10).product();
  println!("factorial: {:?}", factorial_10);
}

