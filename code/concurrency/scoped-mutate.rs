use std::thread;
use std::sync::{Arc,Mutex};

struct Data { acc: i32, seq : Vec<i32> }

fn main() {
    let state = Arc::new(Mutex::new( Data{acc: 0, seq: vec![0;5]} ));

    (0..5).map(|i| {
        let mutex = state.clone();

        thread::scoped(move || {
            let mut data = mutex.lock().unwrap();
            data.acc += 1; data.seq[i] += data.acc;
        })
    }).collect::<Vec<_>>();

    println!("{:?}", state.lock().unwrap().seq);
}

