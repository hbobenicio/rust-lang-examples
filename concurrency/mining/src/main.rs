extern crate rand;

mod app;

fn main() {
    if let Err(msg) = app::start(3) {
        eprintln!("Error: {}", msg);
        std::process::exit(1);
    }
}
