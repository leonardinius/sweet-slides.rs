use std::thread;

fn main() {
    let mut numbers = vec![1, 2, 3];
    for i in 0..3 {
        thread::scoped(move || {
            for j in 0..3 { numbers[j] += 1 }
        });
    }
}

