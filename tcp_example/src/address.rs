pub struct Address {
    pub ip: String,
    pub port: String,
}

impl Address {

    pub fn new() -> Address {
        Address {
            ip: String::new(),
            port: String::new()
        }
    }

    pub fn ip<Str>(mut self, ip: Str) -> Address where Str: Into<String> {
        self.ip = ip.into();
        self
    }

    pub fn port<Str>(mut self, port: Str) -> Address where Str: Into<String> {
        self.port = port.into();
        self
    }
}

impl ToString for Address {
    fn to_string(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}
