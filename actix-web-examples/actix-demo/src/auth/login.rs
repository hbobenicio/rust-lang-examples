use actix_web::Json;

mod payload {

    #[derive(Deserialize)]
    pub struct BasicLoginCredentials {
        pub email: String,
        pub password: String,
    }

    impl std::fmt::Display for BasicLoginCredentials {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

            let password_size = self.password.len();
            let mut masked_password = String::with_capacity(password_size);
            for _ in 0..password_size {
                masked_password.push_str("*");
            }

            write!(f, "BasicLoginCredentials{{email: {}, password: {}}}", self.email, masked_password)
        }
    }
}

pub fn basic_login(credencials: Json<payload::BasicLoginCredentials>) -> actix_web::Result<String> {
    info!("{}", credencials);

    Ok(format!("Got it: {} / {}", credencials.email, credencials.password))
}
