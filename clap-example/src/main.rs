#[macro_use]
extern crate log;
extern crate env_logger;
extern crate colored;
extern crate clap;

mod constants;
// mod scanner;

use std::process;
use colored::*;
use clap::{Arg, SubCommand};

fn log(msg: &str) {
    // println!("{} {}", constants::PREFIX.green().bold(), msg.white().bold());
    info!("{}", msg);
}

fn error(msg: &str) {
    // println!("{} {}", constants::PREFIX.red().bold(), msg.white().bold());
    error!("{}", msg);
}

fn ceph_delete_bucket(bucket_name: &str) -> Result<(), String> {
    log(&format!("Removendo bucket \"{}\"", bucket_name));

    let status: std::process::ExitStatus = process::Command::new("python3")
        .env("PYTHONPATH", ".")
        .arg("scripts/ceph-delete-bucket.py")
        .arg(bucket_name)
        .status()
        .map_err(|_e: std::io::Error| "Não foi possível iniciar processo python".to_string())?;

    if status.success() {
        Ok(())
    } else {
        let error_message = String::from("Processo filho retornou com erro");
        error(&error_message);
        Err(error_message)
    }
}

fn main() {
    env_logger::init();

    let cli_matches = clap::App::new("octopus")
        .version("0.0.1")
        .author("Hugo Benício <hugo.oliveira@serpro.gov.br>")
        .about("CLI for Octopus")
        .subcommand(SubCommand::with_name("ceph")
            .subcommand(SubCommand::with_name("bucket")
                .subcommand(SubCommand::with_name("delete")
                    .arg(Arg::with_name("bucket-name")
                        .help("Nome do bucket a ser removido")
                        .takes_value(true)
                    )
                    .arg(Arg::with_name("all")
                        .help("Remove todos os buckets")
                        .short("a")
                        .long("all")
                        .conflicts_with("bucket-name")
                    )
                    .arg(Arg::with_name("control")
                        .help("Remove o bucket de controle")
                        .short("out")
                        .long("control")
                        .conflicts_with("bucket-name")
                    )
                    .arg(Arg::with_name("output")
                        .help("Remove o bucket de saída")
                        .short("ctrl")
                        .long("output")
                        .conflicts_with("bucket-name")
                    )
                )
            )
        )
        .get_matches();

    if let Some(ceph_matches) = cli_matches.subcommand_matches("ceph") {
        if let Some(bucket_matches) = ceph_matches.subcommand_matches("bucket") {
            if let Some(delete_matches) = bucket_matches.subcommand_matches("delete") {
                if let Some(bucket_name) = delete_matches.value_of("bucket-name") {
                    println!("Removendo bucket \"{}\"...", bucket_name);
                }
                
                if delete_matches.is_present("all") {
                    println!("Removendo todos os buckets...")

                } else if delete_matches.is_present("control") {

                    ceph_delete_bucket("octopus-control")
                        .expect("Não foi possível remover bucket de controle");

                } else if delete_matches.is_present("output") {

                    ceph_delete_bucket("octopus-output")
                        .expect("Não foi possível remover bucket de saída");
                }
            }
        }
    }

    // let mut scanner = scanner::Scanner::new();

    // let line = scanner.next_line().expect("Não foi possível obter linha da entrada padrão (stdin)");
    // log(&line);

    // let line = scanner.next_line().expect("Não foi possível obter linha da entrada padrão (stdin)");
    // log(&line);
}
