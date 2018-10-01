/// Basic Thread usage example.
/// 
/// @see https://doc.rust-lang.org/book/second-edition/ch16-01-threads.html
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Counting number {} from new thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..5 {
        println!("Counting number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().expect("It was not possible to join thread");
}
