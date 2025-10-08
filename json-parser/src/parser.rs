use crate::ast::JsonValue;
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

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.peek() == &expected {
            self.bump();
            return Ok(());
        }

        return Err(format!("Expected {:?}, got {:?}", expected, self.peek()));
    }

    pub fn parse_value(&mut self) -> Result<JsonValue, String> {
        match self.peek() {
            Token::LBrace => self.parse_object(),
            Token::StringLit(s) => {
                let s2 = s.clone();
                self.bump();
                Ok(JsonValue::String(s2))
            }
            other => Err(format!("Unexpected token {:?}", other)),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
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
                other => {
                    return Err(format!(
                        "Unexpected token '{:?}' at position {}",
                        other, self.pos
                    ));
                }
            }
            if self.peek() == &Token::Comma {
                self.bump();
                continue;
            } else if self.peek() == &Token::RBrace {
                self.bump();
                break;
            } else {
                return Err(format!("Expected ',' or '}}', got {:?}", self.peek()));
            }
        }

        return Ok(JsonValue::Object(map));
    }
}
