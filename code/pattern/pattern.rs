fn main() {
    let x = 100;

    match x {
        1 | 2       => println!("one or two"),
        3           => println!("three"),
        e @ 1...5   => println!("one through five, {}", e),
        e if e < 10 => println!("less than 10, {}", e),
        _           => println!("anything"),
    }
}

