#[allow(unused_imports)]

use std::net::{TcpStream, TcpListener};

mod address;
use address::Address;

fn serve(listener: &TcpListener) {
    match listener.accept() {
        Ok((_socket, addr)) =>
            println!("Conexão estabelecida com cliente: {:?}", addr),
        Err(error) =>
            println!("Erro ao conectar com cliente. Erro: {:?}", error),
    }
}

fn main() {
    let server_addr = Address::new("127.0.0.1", "8082");

    let listener = TcpListener::bind(server_addr.to_string())
        .expect("Servidor escutando conexões na porta 8082");

    loop {
        serve(&listener);
    }
}
