/// Módulo de erros da aplicação

/// Representa o erro da aplicação
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    
    /// Caso usuário não forneça a quantidade de argumentos CLI necessários
    WrongNumberOfArgs(usize),

    /// Em caso de erros de IO em geral
    IOError(String),

    /// Em caso de erros de Parsing em geral
    ParsingError(String),

    /// Caso os horários não estejam ordenados de forma crescente
    NotOrdered(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::WrongNumberOfArgs(n) =>
                write!(f, "número errado de argumentos: necessários 1, 2 ou 4. passados: {}", n),

            Error::IOError(s) =>
                write!(f, "io: {}", s),

            Error::ParsingError(s) =>
                write!(f, "parsing: {}", s),
                
            Error::NotOrdered(s) =>
                write!(f, "horários não estão em ordem crescente: {}", s)
        }
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Error::IOError(format!("IOError: {}", other))
    }
}

impl std::convert::From<std::string::ParseError> for Error {
    fn from(other: std::string::ParseError) -> Self {
        Error::ParsingError(format!("ParseError: {}", other.to_string()))
    }
}

impl std::convert::From<std::num::ParseIntError> for Error {
    fn from(other: std::num::ParseIntError) -> Self {
        Error::ParsingError(format!("ParseError: {}", other.to_string()))
    }
}
