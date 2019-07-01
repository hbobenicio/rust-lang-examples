
/// The Error type used to encapsulate all errors for the program.
///
///
#[derive(Debug)]
pub struct Error {
    pub code: u32,
    pub description: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::error::Error;

        write!(f, "error ER{:0>4}: {}", self.code, self.description)?;
        if let Some(cause) = self.source() {
            write!(f, "caused by: {}", cause)?;
        }

        Ok(())
    }
}

impl std::error::Error for Error {

}
