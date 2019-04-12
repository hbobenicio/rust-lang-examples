extern crate prettytable;

mod app;

fn main() {
    match app::run() {
        Ok(total) => println!("total de horas trabalhadas: {}h", total),
        Err(reason) => {
            eprintln!("erro: {}", reason);
            std::process::exit(1);
        }
    };
}
