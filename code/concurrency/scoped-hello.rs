use std::thread;

#[allow(unused_variables)]
fn main() {
    let guards: Vec<_> = (0..5).map(|_| {
        thread::scoped(|| {
            println!("Hello, world!");
        })
    }).collect();

    // We do't need to join guards here
    //  due to nature of thread::scoped.
    //  It gets auto joined on Drop
    //guards.into_iter().map(|g| g.join()).collect::<Vec<_>>();
}

