use std::thread;

fn main() {
    let nthreads = 10;

    // This won't compile at first, because the compiler
    // doesn't know about vectors type. When it sees
    // the .push() method been called below, it will
    // infer the vectors type and our code become
    // compilable
    let mut threads = vec![];

    for _ in 0..nthreads {
        threads.push(thread::spawn(|| {
            println!("Hello, World!");
        }));
    }

    for t in threads {
        t.join().unwrap();
        // let _ = t.join();
    }
}

