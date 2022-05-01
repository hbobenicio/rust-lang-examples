use crate::ast::Program;
use crate::lexer::Lexer;

mod token;
mod lexer;
mod ast;

struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn parse(input: &str) -> Program {

    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
