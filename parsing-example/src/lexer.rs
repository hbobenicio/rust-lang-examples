use regex::Regex;

use crate::token::{self, Token};

#[derive(Debug)]
pub(crate) enum Error {
    UnexpectedToken {
        line: usize,
        column: usize,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::UnexpectedToken{ line, column } =>
                write!(f, "unexpected token at line {}, column {}", line, column)
        }
    }
}

impl std::error::Error for Error {}

#[derive(Clone)]
struct Pattern {
    regex: Regex,
    token_kind: token::Kind,
}

pub(crate) struct Lexer<'input> {
    /// input source
    input: &'input str,
    /// regex of patterns to be ignored
    ignored: Regex,
    /// regex patterns for Token scanning
    patterns: Vec<Pattern>,
    /// byte offset from input
    pos: usize,
    /// line counter for the input source. Starts with 1
    line: usize,
    /// column counter for the input source. Starts with 1
    column: usize,
}

impl<'input> Lexer<'input> {
    /// Creates a new Lexer from the given input
    pub(crate) fn new(input: &'input str) -> Self {
        Self {
            input,
            ignored: Regex::new(r"^[ \t]+").unwrap(),
            patterns: vec![
                Pattern {
                    regex: Regex::new(r"^\r?\n").unwrap(),
                    token_kind: token::Kind::EOL,
                },
                Pattern {
                    regex: Regex::new(r"^END").unwrap(),
                    token_kind: token::Kind::Keyword { kind: token::KeywordKind::END },
                },
                Pattern {
                    regex: Regex::new(r"^\d+").unwrap(),
                    token_kind: token::Kind::Integer,
                },
            ],
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    /// scans for the next token.
    ///
    /// If the end of input is found, the EOF token is returned.
    pub(crate) fn next_token(&mut self) -> Result<Token, Box<dyn std::error::Error>> {
        if self.eof() {
            return Ok(Token::eof());
        }

        // skip ignored symbols
        if let Some(m) = self.ignored.find(&self.input[self.pos..]) {
            let lexeme = m.as_str();
            let offset = lexeme.chars().count();
            self.advance(offset);
        };

        // after skiping  we can reach eof. check it again.
        if self.eof() {
            return Ok(Token::eof());
        }

        // iterate thru all regex patterns and check for the first pattern that matches the remaining input.
        // The match will indicate what kind of token should be returned.
        for pattern in &self.patterns {
            if let Some(m) = pattern.regex.find(&self.input[self.pos..]) {
                let lexeme = m.as_str();

                let tok = Token {
                    kind: pattern.token_kind,
                    len: lexeme.len(),
                };

                let offset = lexeme.chars().count();
                if pattern.token_kind == token::Kind::EOL {
                    self.advance_newline(offset);
                } else {
                    self.advance(offset);
                }

                return Ok(tok);
            }
        }

        Err(Box::new(Error::UnexpectedToken {
            line: self.line,
            column: self.column,
        }))
    }

    #[inline]
    fn advance(&mut self, offset: usize) {
        self.pos += offset;
        self.column += offset;
    }

    #[inline]
    fn advance_newline(&mut self, newline_size: usize) {
        self.pos += newline_size;
        self.column = 1;
        self.line += 1;
    }

    fn eof(&mut self) -> bool {
        self.pos >= self.input.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        let input = "";
        let mut lexer = Lexer::new(input);

        assert!(lexer.next_token().expect("some token").is_eof());
        assert!(lexer.next_token().expect("some token").is_eof());
    }

    #[test]
    fn single_tokens() {
        struct TestCase {
            input: &'static str,
            expected_token: Token,
        }
        let test_cases = {
            struct TestInput {
                input: &'static str,
                expected_token_kind: token::Kind,
            }
            let inputs = vec![
                TestInput { input: "42", expected_token_kind: token::Kind::Integer },
                TestInput { input: "\n", expected_token_kind: token::Kind::EOL },
                TestInput { input: "\r\n", expected_token_kind: token::Kind::EOL },
                TestInput { input: "END", expected_token_kind: token::Kind::Keyword { kind: token::KeywordKind::END } },
            ];
            inputs.into_iter().map(|t| TestCase {
                input: t.input,
                expected_token: Token {
                    kind: t.expected_token_kind,
                    len: t.input.len(),
                },
            })
        };
        for tc in test_cases {
            let mut lexer = Lexer::new(tc.input);

            let actual_token = lexer.next_token().expect("some token");
            assert_eq!(actual_token, tc.expected_token);

            let expected_eof = lexer.next_token().expect("some token");
            assert!(expected_eof.is_eof());
        }
    }
}