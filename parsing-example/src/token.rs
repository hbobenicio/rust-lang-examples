//! Defines the lexical tokens from the language.

use crate::token::Kind::Unknown;

/// Enumerates all token types.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) enum Kind {
    /// Unknown token. Useful for error reportings and unset tokens
    Unknown,
    /// End Of File. Represents the end of the input stream.
    EOF,
    /// End of Line. Represents a new line (`\n` or `\r\n`).
    EOL,
    /// Simple positive integer.
    Integer,
    /// Keywords
    Keyword { kind: KeywordKind },
}

impl Default for Kind {
    fn default() -> Self {
        Unknown
    }
}

/// Enumerates all keyword token types.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) enum KeywordKind {
    END,
}

/// The Token struct that represents any token
#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub(crate) struct Token {
    /// The kind (type) of token
    pub(crate) kind: Kind,
    /// The length of the token
    pub(crate) len: usize,
}

impl Token {
    /// Creates a new EOF Token that represents the end of the input file.
    #[inline]
    pub(crate) fn eof() -> Self {
        Token {
            kind: Kind::EOF,
            len: 0,
        }
    }

    /// Creates a new EOL Token that represents the end a line.
    #[inline]
    pub(crate) fn eol(len: usize) -> Self {
        Token {
            kind: Kind::EOL,
            len,
        }
    }

    /// Creates a new integer token.
    #[inline]
    pub(crate) fn integer(len: usize) -> Self {
        Token {
            kind: Kind::Integer,
            len,
        }
    }

    /// Creates a new END keyword token.
    #[inline]
    pub(crate) fn end() -> Self {
        Token {
            kind: Kind::Keyword {
                kind: KeywordKind::END,
            },
            len: 3,
        }
    }

    /// Checks whether the token kind is `Token::EOF`.
    #[inline]
    pub(crate) fn is_eof(&self) -> bool {
        self.kind == Kind::EOF
    }

    /// Checks whether the token kind is `Token::EOL`.
    #[inline]
    pub(crate) fn is_eol(&self) -> bool {
        self.kind == Kind::EOL
    }

    /// Checks whether the token kind is `Token::Integer`.
    #[inline]
    pub(crate) fn is_integer(&self) -> bool {
        self.kind == Kind::Integer
    }

    /// Checks whether the token kind is `Token::Keyword` and its kind is `KeywordKind::END`.
    #[inline]
    pub(crate) fn is_keyword_end(&self) -> bool { self.kind == Kind::Keyword { kind: KeywordKind::END } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eof() {
        let eof = Token::eof();

        assert_eq!(eof.kind, Kind::EOF);
        assert!(eof.is_eof());
    }

    #[test]
    fn integer_tests() {
        let token_len = 4;
        let tok = Token::integer(token_len);

        assert_eq!(tok.kind, Kind::Integer);
        assert_eq!(tok.len, token_len);
        assert!(tok.is_integer());
    }

    #[test]
    fn test_eol() {
        let eol = Token::eol(1);
        assert!(eol.is_eol());
        assert_eq!(eol.len, 1);

        let eol = Token::eol(2);
        assert!(eol.is_eol());
        assert_eq!(eol.len, 2);
    }

    #[test]
    fn test_keyword_end() {
        let end = Token::end();

        assert!(end.is_keyword_end());
        assert_eq!(end.len, 3);
    }
}
