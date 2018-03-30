#![allow(dead_code)]
#![allow(unused_variables)]

mod math;

use std::fs::File;
use std::io::ErrorKind;
use std::error;
use std::fmt;

// Unrecoverable Errors: The panic! macro

fn abort() {
    // Panicking from our program
    panic!("Must crash and burn!")
}

fn third_party_panic() {
    let v = vec![1, 2, 3];

    // Panicking from third-parties
    v[100];
}

// Recoverable Errors: Result<T, E>

fn fail_from_file() {
    let filename = "unknown-file.txt";

    let file = match File::open(filename) {
        Ok(f) => f,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("unknown-file.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    println!("Tried to create file without success:");
                    panic!(e.to_string());
                }
            }
        },
        Err(error) => {
            println!("It was not possible to open file: {:?}", filename);
            panic!(error.to_string());
        }
    };
}

/**
 * `unwrap` panics in case of Err.
 */
fn fail_from_file2() {
    let filename = "unknown-file.txt";
    let file = File::open(filename).unwrap();
}

/**
 * `expect` panics in case of Err, with message.
 */
fn fail_from_file3() {
    let filename = "unknown-file.txt";
    let file = File::open(filename).expect("Failed to open file");
}

#[derive(Debug, Clone)]
struct ArithmeticError;
impl fmt::Display for ArithmeticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot divide by zero")
    }
}
impl error::Error for ArithmeticError {
    fn description(&self) -> &str {
        "Cannot divide by zero"
    }
}
fn divide(x: i32, y: i32) -> Result<f32, ArithmeticError> {
    if y == 0 {
        Err(ArithmeticError)
    } else {
        Ok((x/y) as f32)
    }
}

// TODO Add a '?' macro example



fn main() {
    println!("Error Handling Example");
    // Topics:
    // - Unrecoverable Errors
    //   - the panic! macro
    // - Recoverable Errors
    //   - enum Result<T, E> { Ok(T), Err(E) }
    //   - Handling Recoverable Errors with pattern match
    //   - unwrap
    //   - expect
    //   - Error propagation
    //     - returning Results
    //     - the '?' macro

    // abort()
    // third_party_panic();
    // fail_from_file();
    // fail_from_file2();
    // fail_from_file3();
    println!("{:?}", divide(5, 3).unwrap());
}
