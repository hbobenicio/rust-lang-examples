mod app;

fn main() {
    if let Err(err) = app::start() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
