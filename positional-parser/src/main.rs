#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod positional_parser;

fn print_usage() {
    println!("positional-parser <SCHEMA_FILE> <DATA_FILE>");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        print_usage();
        std::process::exit(1);
    }
    
    let schema_filepath: &String = &args[1];
    let data_filepath: &String = &args[2];

    match positional_parser::start(schema_filepath, data_filepath) {
        Ok(_) => println!("Sucesso."),
        Err(error) => {
            println!("{}", error);
            std::process::exit(1);
        }
    }
}
