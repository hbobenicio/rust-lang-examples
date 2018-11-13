//! Example module that demonstrates how to use reqwest to make simple
//! http requests.

use hyper::rt::Future;

/// Checks the status of the specified links (reqwest implementation)
/// 
/// # Arguments
/// 
/// * `links` - The list of links to be checked
pub fn check_links(links: &Vec<String>) -> Result<(), String> {

    let mut futures = vec![];

    for link in links {
        let uri: hyper::Uri = link.parse().map_err(|e|
            format!("could not parse link {}: {}", link, e)
        )?;
        let future = fetch_link(&uri);
        futures.push(Box::new(future));
    }

    let joined_future = futures::future::join_all(futures);

    // Tokio runtime need to execute a future like Future<(), ()>, so you have
    // to consume its item and it's error before executing it.
    let sink = joined_future.map(|_|{});

    hyper::rt::run(sink);

    Ok(())
}

fn fetch_link(url: &hyper::Uri) -> impl Future<Item = (), Error = ()> {
    let client = hyper::Client::new();

    client.get(url.clone())
        .map(|resp: hyper::Response<hyper::Body>| {
            println!("{}", resp.status());
        })
        .map_err(|e: hyper::Error| {
            eprintln!("error {}", e);
        })
}
