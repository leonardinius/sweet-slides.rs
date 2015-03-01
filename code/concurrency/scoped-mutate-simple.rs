use std::thread;
use std::sync::{Arc,Mutex};

fn main() {
    let data = Arc::new(Mutex::new( vec![0;5] ));

    (0..5).map(|i| {
        let mutex = data.clone();

        thread::scoped(move || {
            let mut vec = mutex.lock().unwrap();
            vec[i] += 1;
        })
    }).collect::<Vec<_>>();

    println!("{:?}", *data.lock().unwrap());
}

