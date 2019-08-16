use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {

    #[serde(default = "Config::default_server_host")]
    pub server_host: String,

    #[serde(default = "Config::default_server_port")]
    pub server_port: String,
}

impl Config {
    pub fn from_env() -> Self {
        match envy::prefixed("APP_").from_env() {
            Ok(config) => config,
            Err(err) => {
                eprintln!("config error: {:#?}", err);
                std::process::exit(1);
            }
        }
    }

    fn default_server_host() -> String {
        String::from("0.0.0.0")
    }

    fn default_server_port() -> String {
        String::from("8080")
    }
}
