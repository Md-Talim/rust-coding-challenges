use crate::ast::JsonValue;
use crate::error::{Error, Result};
use crate::token::Token;
use std::collections::BTreeMap;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn bump(&mut self) {
        self.pos += 1
    }

    fn expect(&mut self, expected: Token) -> Result<()> {
        match self.peek() {
            token if token == &expected => {
                self.bump();
                return Ok(());
            }
            Token::EOF => Err(Error::UnexpectedEOF),
            _ => Err(Error::ExpectedToken(format!("{:?}", expected), self.pos)),
        }
    }

    pub fn parse_value(&mut self) -> Result<JsonValue> {
        match self.peek() {
            Token::LBrace => self.parse_object(),
            Token::StringLit(s) => {
                let s2 = s.clone();
                self.bump();
                Ok(JsonValue::String(s2))
            }
            Token::EOF => Err(Error::UnexpectedEOF),
            other => Err(Error::UnexpectedToken(format!("{:?}", other), self.pos)),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue> {
        self.expect(Token::LBrace)?;
        let mut map = BTreeMap::new();
        if self.peek() == &Token::RBrace {
            self.bump();
            return Ok(JsonValue::Object(map));
        }

        loop {
            match self.peek() {
                Token::StringLit(s) => {
                    let key = s.clone();
                    self.bump();
                    self.expect(Token::Colon)?;
                    let value = self.parse_value()?;
                    map.insert(key, value);
                }
                Token::EOF => return Err(Error::UnexpectedEOF),
                other => return Err(Error::UnexpectedToken(format!("{:?}", other), self.pos)),
            }

            match self.peek() {
                Token::Comma => {
                    self.bump();
                }
                Token::RBrace => {
                    self.bump();
                    break;
                }
                Token::EOF => return Err(Error::UnexpectedEOF),
                other => return Err(Error::UnexpectedToken(format!("{:?}", other), self.pos)),
            }
        }

        return Ok(JsonValue::Object(map));
    }
}
