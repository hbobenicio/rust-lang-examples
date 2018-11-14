//! Example module that demonstrates how to use reqwest to make simple
//! http requests.

/// Checks the status of the specified links (reqwest implementation)
/// 
/// # Arguments
/// 
/// * `links` - The list of links to be checked
pub fn check_links(links: &Vec<String>) -> Result<(), String> {
    println!("--- reqwest (with redirect) ---");

    for link in links {
        let response: reqwest::Response = reqwest::get(link).map_err(|e|
            format!("could not get link {}: {}", link, e)
        )?;

        println!("{} - {}", link, response.status());
    }

    println!("");
    Ok(())
}

/// Checks the status of the specified links (reqwest implementation).
/// No redirects!
/// 
/// # Arguments
/// 
/// * `links` - The list of links to be checked
pub fn check_links_no_redirect(links: &Vec<String>) -> Result<(), String> {
    println!("--- reqwest (no redirect) ---");

    for link in links {
        let client = reqwest::Client::builder()
            .redirect(reqwest::RedirectPolicy::none())
            .build()
            .map_err(|e: reqwest::Error|
                format!("could not build client: {}", e)
            )?;

        let response: reqwest::Response = client.get(link).send().map_err(|e|
            format!("could not get link {}: {}", link, e)
        )?;

        println!("{} - {}", link, response.status());
    }

    println!("");
    Ok(())
}
