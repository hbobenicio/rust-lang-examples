mod bench;
mod constant;
mod implementation;

use std::fs::File;

fn main() {
    match bench::run() {
        Ok(_) => {
            // Dump the report to disk
            let mut report_file = File::create("flame-graph.html").unwrap();
            flame::dump_html(&mut report_file).unwrap();
        },

        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    }
}
