use std::iter::iterate;

fn main() {
  let fibonacci_seq =
      iterate( (0u64, 1u64), |e| {(e.1, e.0 + e.1)} )
        .skip(1)
        .map(|e| e.1);

  let numbers: Vec<_> = fibonacci_seq.take(10).collect();

  println!("{:?}", numbers);
}
