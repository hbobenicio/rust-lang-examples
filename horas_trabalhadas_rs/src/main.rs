mod app;

fn main() {
    match app::run() {
        Ok(total) => println!("horas trabalhadas: {}h", total),
        Err(reason) => {
            eprintln!("erro: {}", reason);
            std::process::exit(1);
        }
    };
}
