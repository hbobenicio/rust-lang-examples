mod config;

fn main() {
    let config = config::Config::from_env();

    println!("config: {:#?}", config);
}
