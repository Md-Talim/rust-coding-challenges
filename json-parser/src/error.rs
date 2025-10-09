use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    UnexpectedChar(char, usize),
    UnterminatedString(usize),
    UnexpectedToken(String, usize),
    ExpectedToken(String, usize),
    UnexpectedEOF,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnexpectedChar(c, pos) => write!(f, "Unexpected character '{}' at position {}", c, pos),
            Error::UnterminatedString(pos) => write!(f, "Unterminated string starting at position {}", pos),
            Error::UnexpectedToken(token, pos) => write!(f, "Unexpected token '{}' at position {}", token, pos),
            Error::ExpectedToken(token, pos) => write!(f, "Expected token '{}' at position {}", token, pos),
            Error::UnexpectedEOF => write!(f, "Unexpected end of input"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
