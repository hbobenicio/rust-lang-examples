extern crate gtk;
extern crate gdk;

mod app;

fn main() {
    let app = app::App::new();

    if let Err(reason) = app {
        eprintln!("Application initialization error: {}", reason);
        std::process::exit(1);
    };

    let app = app.unwrap();

    if let Err(reason) = app.start() {
        eprintln!("Application Error: {}", reason);
        std::process::exit(1);
    }
}
