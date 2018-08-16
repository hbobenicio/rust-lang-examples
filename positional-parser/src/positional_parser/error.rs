use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub code: String,
    pub message: String
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "[{}] {}", self.code, self.message)
    }
}
