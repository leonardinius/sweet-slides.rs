use std::thread;
use std::sync::{Arc,Mutex};

fn main() {
    let numbers = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));

    let guards: Vec<_> = (0..10).map (|i| {
        let number = numbers.clone();

        thread::spawn(move || {
            let mut array = number.lock().unwrap();
            array[i] += 1;
            println!("numbers[{}] is {}", i, array[i]);
        })

        // the return value is the join handle returned by thread::spawn
        //
        // alternatively we could make explicit return here
    }).collect();

    guards.into_iter().map(|g| g.join()).collect::<Vec<_>>();
}

