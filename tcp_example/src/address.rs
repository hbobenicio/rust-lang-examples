pub struct Address {
    pub ip: String,
    pub port: String,
}

impl Address {
    pub fn new<Str>(ip: Str, port: Str) -> Address where Str: Into<String> {
        Address {
            ip: ip.into(),
            port: port.into()
        }
    }
}

impl ToString for Address {
    fn to_string(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}
