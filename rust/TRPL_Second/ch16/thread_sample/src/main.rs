use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from th spawned thread!", i);
        }
    });

    let _ = handle.join();

    for i in 1..5 {
        println!("Hi number {} from th main thread!", i);
    }

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // This closure has ownership of v.
        println!("Here's a vector: {:?}", v);
    });

    let _ = handle.join();
}
