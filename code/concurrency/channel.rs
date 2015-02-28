use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::*;

fn main() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    let thread_tx = tx.clone();
    thread::spawn(move || {
        thread_tx.send(10).ok().expect("Unable to send message");
    });

    println!("{:?}", rx.recv());
}