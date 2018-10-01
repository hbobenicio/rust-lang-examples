use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn example1() {
    let (tx, rx) = mpsc::channel();

    // Spawns a new producer thread
    let _producer_handle = thread::spawn(move || {

        let value = String::from("Hi!");

        if let Err(error) = tx.send(value) {
            eprintln!("[PRODUCER] Couldn't send message over channel: {}", error);
        };
    });

    // block the main thread’s execution and wait until a value is sent down the channel
    match rx.recv() {
        Ok(msg) => println!("[MAIN] {}", msg),
        Err(error) => eprintln!("[MAIN] Couldn't receive message over channel: {}", error)
    };
}

fn example2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for value in values {
            tx.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Blocks the main thread until the channel closes (i.e. tx has drops)
    for value in rx {
        println!("[MAIN] New Value: {}", value);
    }
}

fn example3() {
    let (tx, rx) = mpsc::channel();

    // let x = "foo".to_string();
    // let x = 10;
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        // println!("[T1] X: {}", x);
        let values = vec![
            String::from("[T1] hi"),
            String::from("[T1] from"),
            String::from("[T1] the"),
            String::from("[T1] thread"),
        ];

        for value in values {
            tx1.send(value).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });
    thread::spawn(move || {
        // println!("[T2] X: {}", x);
        let values = vec![
            String::from("[T2] hi"),
            String::from("[T2] from"),
            String::from("[T2] the"),
            String::from("[T2] thread"),
        ];

        for value in values {
            tx.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Blocks the main thread until the channel closes (i.e. tx has drops)
    for value in rx {
        println!("[MAIN] New Value: {}", value);
    }
}

fn main() {
    example1();
    example2();
    example3();
}
