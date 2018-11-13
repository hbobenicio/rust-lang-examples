//! Example module that demonstrates how to use reqwest to make simple
//! http requests.

/// Checks the status of the specified links (reqwest implementation)
/// 
/// # Arguments
/// 
/// * `links` - The list of links to be checked
pub fn check_links(links: &Vec<String>) -> Result<(), String> {

    for link in links {
        let response: reqwest::Response = reqwest::get(link).map_err(|e|
            format!("could not get link {}: {}", link, e)
        )?;

        println!("{} - {}", link, response.status());
    }

    Ok(())
}
