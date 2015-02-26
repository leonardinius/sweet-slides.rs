use std::thread;

fn main() {
    for _ in 0..5 {
        thread::scoped(|| {
            println!("Hello, world!");
        });
    };
}

