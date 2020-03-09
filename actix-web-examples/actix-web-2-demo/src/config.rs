use log::error;

#[derive(Debug)]
pub struct Config {
    pub server_addr: String,
    pub database_url: String,
}

pub enum ConfigError {
    MissingRequiredVariable {
        missing_variables: Vec<String>,
    },
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut missing_variables: Vec<String> = vec!();

        // optional
        let server_addr: String = std::env::var("SERVER_ADDR")
            .unwrap_or("0.0.0.0:8080".to_string());

        // required
        let database_url: String = match std::env::var("DATABASE_URL") {
            Ok(value) => value,
            Err(err) => {
                error!("error: {}", err);
                missing_variables.push(String::from("DATABASE_URL"));
                String::from("")
            }
        };

        if missing_variables.len() > 0 {
            let error = ConfigError::MissingRequiredVariable { missing_variables };
            return Err(error);
        }

        Ok(Config{
            server_addr,
            database_url,
        })
    }
}

fn _load_env(key: &str, default: Option<String>) -> Result<String, ConfigError> {
    let env_value = std::env::var(key);

    if let Ok(value) = env_value {
        return Ok(value);
    }

    if let Some(default_value) = default {
        return Ok(default_value);
    }

    let error = ConfigError::MissingRequiredVariable{
        missing_variables: vec![ key.to_string() ]
    };

    Err(error)
}
