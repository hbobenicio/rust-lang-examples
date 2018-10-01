use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // Closure captures environment by reference, by default.
    // But Rust can't tell if this context would live enough for the other thread to finish.
    // Closure can also capture by moving the environment, taking ownership of it.
    let handle = thread::spawn(move || {
        println!("[WORKER] here's a vector: {:?}", v);
    });

    // drop(v);

    handle.join().unwrap();
}
