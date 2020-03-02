#[derive(Debug)]
pub struct Config {
    pub server_addr: String,
}

pub enum ConfigError {
    MissingRequiredVariable {
        missing_variables: Vec<String>,
    },
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        // let mut missing_variables: Vec<String> = vec!();
        let missing_variables: Vec<String> = vec!();

        let server_addr: String = std::env::var("SERVER_ADDR")
            .unwrap_or("0.0.0.0:8080".to_string());

        if missing_variables.len() > 0 {
            let error = ConfigError::MissingRequiredVariable { missing_variables };
            return Err(error);
        }

        Ok(Config{
            server_addr,
        })
    }
}
