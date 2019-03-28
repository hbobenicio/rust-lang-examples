extern crate futures;
extern crate hyper;
extern crate reqwest;

mod example;
mod example2;

fn main() {
    example2::run();
    // let links = [
    //     String::from("http://google.com"),
    //     String::from("http://yahoo.com"),
    //     String::from("http://stackoverflow.com"),
    // ];

    // let mut failed = false;

    // if let Err(e) = example::hyper::check_links(&links) {
    //     eprintln!("hyper example failed: {}", e);
    //     failed = true;
    // }

    // if let Err(e) = example::reqwest::check_links_no_redirect(&links) {
    //     eprintln!("reqwest (no redirect) example failed: {}", e);
    //     failed = true;
    // }

    // if let Err(e) = example::reqwest::check_links(&links) {
    //     eprintln!("reqwest example failed: {}", e);
    //     failed = true;
    // }

    // if failed {
    //     std::process::exit(1);
    // }
}
