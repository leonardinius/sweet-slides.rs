fn main() {
    let mut numbers = vec![1, 2, 3];

    for _ in 0..3 {
        std::thread::scoped(move || {
            for j in 0..3 { numbers[j] += 1 }
        });
    }
}

