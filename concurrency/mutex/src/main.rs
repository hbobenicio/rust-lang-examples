// use std::rc::Rc;
use std::thread::{self};
use std::sync::{Mutex, Arc};

fn single_thread_mutex() {
    let m = Mutex::new(5);

    {
        let mut x = m.lock().unwrap();
        *x = 6;
    }
    println!("Number: {:?}", m);
}

fn mutex_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let num_threads = 10;

    for _ in 0..num_threads {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // Wait until all threads finish their work
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total: {}", *counter.lock().unwrap());
}

fn main() {
    single_thread_mutex();
    mutex_example();
}
