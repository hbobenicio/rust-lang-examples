use super::service;
use super::post::Post;

pub struct Service {
    http_client: reqwest::Client,
}

impl Service {
    pub fn new() -> Self {
        Service {
            http_client: reqwest::Client::new(),
        }
    }
}

impl service::Service for Service {
    fn get_posts(&self) -> Result<Vec<Post>, service::Error> {
        let url: String = self.path("/posts");
        println!("GET {}", url);
        
        let mut response: reqwest::Response = self.http_client.get(&url)
            .send()?;

        let posts: Vec<Post> = response.json()?;
        Ok(posts)
    }
}

impl std::convert::From<reqwest::Error> for service::Error {
    fn from(other: reqwest::Error) -> Self {
        let err_msg = format!("reqwest error: {}", other);
        service::Error::GenericError(err_msg)
    }
}
