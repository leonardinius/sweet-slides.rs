use std::thread;
use std::sync::{Arc,Mutex};

fn main() {
    let state = Arc::new(Mutex::new( (0,vec![0;5]) ));

    let handles: Vec<_> = (0..5).map(|i| {
        let mutex = state.clone();

        thread::spawn(move || {
            let mut data = mutex.lock().unwrap();
            data.0 += 1; data.1[i] += data.0;
        })
    }).collect();

    handles.into_iter()
        .map(|g| g.join())
        .collect::<Vec<_>>();

    println!("{:?}", state.lock().unwrap().1);
}

