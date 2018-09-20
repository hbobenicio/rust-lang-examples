use std::io;

#[derive(Debug)]
pub struct Scanner {
  stdin: io::Stdin,
  buffer: String
}

// TODO Add ScannerError
// TODO Add impl From<io::Error> for ScannerError
// TODO Add impl From<parser::Error> for ScannerError

impl Scanner {
    pub fn new() -> Scanner {
      Scanner {
        stdin: io::stdin(),
        buffer: String::new()
      }
    }

    pub fn next_line(&mut self) -> Result<String, io::Error> {
      self.stdin.read_line(&mut self.buffer)?;

      let resp = self.buffer.clone();
      let resp = resp.trim().to_string();

      Ok(resp)
    }
}
