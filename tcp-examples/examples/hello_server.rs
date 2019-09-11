use std::io::prelude::*;
use log::{info, error};

fn main() {
    init_log();

    let addr = get_address();
    let listener = std::net::TcpListener::bind(addr.clone())
        .expect(&format!("couldn't bind to address '{}'", addr));

    for stream in listener.incoming() {
        match stream {
            Err(err) => {
                error!("couldn't connect to client: {:?}", err);
                continue;
            },
            Ok(stream) => {
                info!("new client connection: {}", stream.peer_addr().unwrap());

                if let Err(err) = handle_client(stream) {
                    error!("error: {:?}", err);
                }
            }
        }
    }
}

fn init_log() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
}

fn get_address() -> String {
    let var = "SERVER_ADDRESS";
    let default_value = String::from("127.0.0.1:8080");

    match std::env::var(var) {
        Ok(host) => host,
        Err(_) => {
            info!("environment variable {} was not defined. considering default value: {}", var, default_value);
            default_value
        }
    }
}

fn handle_client(mut stream: std::net::TcpStream) -> std::io::Result<()> {
    writeln!(stream, "Hello, Client!")?;

    Ok(())
}
