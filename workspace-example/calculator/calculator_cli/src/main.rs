extern crate colored;
extern crate clap;
extern crate calculator_core as calculator;

use clap::{SubCommand, Arg};

fn main() {
    let first_arg = Arg::with_name("x")
        .help("First argument")
        .required(true)
        .takes_value(true);

    let second_arg = Arg::with_name("y")
        .help("Second argument")
        .required(true)
        .takes_value(true);

    // TODO replace all 4 subcommand with just one argument: "operator"
    let add_subcommand = SubCommand::with_name("add")
        .arg(first_arg.clone())
        .arg(second_arg.clone());

    let sub_subcommand = SubCommand::with_name("sub")
        .arg(first_arg.clone())
        .arg(second_arg.clone());

    let mult_subcommand = SubCommand::with_name("mult")
        .arg(first_arg.clone())
        .arg(second_arg.clone());

    let div_subcommand = SubCommand::with_name("div")
        .arg(first_arg.clone())
        .arg(second_arg.clone());

    let cli_matches = clap::App::new("calculator")
        .version("0.1.0")
        .author("Hugo Benício <hugo.oliveira@serpro.gov.br>")
        .about("Calculator CLI")
        .subcommand(add_subcommand)
        .subcommand(sub_subcommand)
        .subcommand(mult_subcommand)
        .subcommand(div_subcommand)
        .get_matches();

    if let Some(add) = cli_matches.subcommand_matches("add") {
        if add.is_present("x") && add.is_present("y") {
            let x = add.value_of("x").unwrap();
            let y = add.value_of("y").unwrap();
            let xi: i32 = x.parse().unwrap();
            let yi: i32 = y.parse().unwrap();
            let total = calculator::add(xi, yi);

            println!("{}", total);
        }
    } else if let Some(sub) = cli_matches.subcommand_matches("sub") {
        if sub.is_present("x") && sub.is_present("y") {
            let x = sub.value_of("x").unwrap();
            let y = sub.value_of("y").unwrap();
            let xi: i32 = x.parse().unwrap();
            let yi: i32 = y.parse().unwrap();
            let total = calculator::sub(xi, yi);

            println!("{}", total);
        }
    } else if let Some(mult) = cli_matches.subcommand_matches("mult") {
        if mult.is_present("x") && mult.is_present("y") {
            let x = mult.value_of("x").unwrap();
            let y = mult.value_of("y").unwrap();
            let xi: i32 = x.parse().unwrap();
            let yi: i32 = y.parse().unwrap();
            let total = calculator::mult(xi, yi);

            println!("{}", total);
        }
    } else if let Some(div) = cli_matches.subcommand_matches("div") {
        if div.is_present("x") && div.is_present("y") {
            let x = div.value_of("x").unwrap();
            let y = div.value_of("y").unwrap();
            let xi: i32 = x.parse().unwrap();
            let yi: i32 = y.parse().unwrap();
            let total = calculator::div(xi, yi);

            println!("{}", total);
        }
    }
}
