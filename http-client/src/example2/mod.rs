pub mod jsonplaceholder;

use jsonplaceholder::{self as jph, service::Service};

pub fn run() {
    let s1 = jph::reqwest::Service::new();
    match s1.get_posts() {
        Ok(posts) => {
            for post in &posts {
                println!("{}", post);
            }
            println!("Total Posts: {}", posts.len());
        },
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
