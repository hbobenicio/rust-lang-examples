use super::post::Post;

pub const BASE_URL: &'static str = "https://jsonplaceholder.typicode.com";

pub enum Error {
    GenericError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("jsonplaceholder service error: ")?;
        match self {
            Error::GenericError(err_msg) => f.write_str(&err_msg),
        }
    }
}

pub trait Service {
    fn base_url(&self) -> String {
        String::from(BASE_URL)
    }

    fn path(&self, p: &str) -> String {
        format!("{}{}", self.base_url(), p)
    }

    fn get_posts(&self) -> Result<Vec<Post>, Error>;
}

