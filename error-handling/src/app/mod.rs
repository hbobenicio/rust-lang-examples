pub mod error;

pub fn start() -> Result<(), Box<std::error::Error>> {
    println!("Hello, Error Handling!");

    Err(Box::new(error::Error{
        code: 42,
        description: "unimplemented".to_string(),
    }))
}
