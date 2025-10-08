use std::collections::BTreeMap;
use std::io::{self, Read};
use std::process::exit;

#[derive(Debug, PartialEq)]
enum JsonValue {
    String(String),
    Object(BTreeMap<String, JsonValue>),
}

#[derive(Debug, PartialEq)]
enum Token {
    LBrace,
    RBrace,
    Colon,
    Comma,
    StringLit(String),
    EOF,
}

struct Lexer<'a> {
    input: &'a str,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self { input }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = self.input.char_indices().peekable();

        while let Some((pos, ch)) = chars.next() {
            match ch {
                // Skip whitespace
                ' ' | '\t' | '\n' | '\r' => continue,

                // Special characters
                '{' => tokens.push(Token::LBrace),
                '}' => tokens.push(Token::RBrace),
                ':' => tokens.push(Token::Colon),
                ',' => tokens.push(Token::Comma),

                // String literals
                '"' => {
                    let string_content = self.parse_string(&mut chars, pos)?;
                    tokens.push(Token::StringLit(string_content));
                }

                _ => return Err(format!("Unexpected character '{}' at position {}", ch, pos)),
            }
        }

        tokens.push(Token::EOF);
        return Ok(tokens);
    }

    fn parse_string(
        &self,
        chars: &mut std::iter::Peekable<std::str::CharIndices>,
        start_pos: usize,
    ) -> Result<String, String> {
        let mut result = String::new();

        while let Some((_, ch)) = chars.next() {
            match ch {
                '"' => return Ok(result),
                _ => result.push(ch),
            }
        }

        Err(format!(
            "Unterminated string starting at position {}",
            start_pos
        ))
    }
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
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

    fn parse_value(&mut self) -> Result<JsonValue, String> {
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

fn parse_json(input: &str) -> Result<JsonValue, String> {
    let mut lexer = Lexer::new(&input);

    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(msg) => return Err(format!("Lexer error: {}", msg)),
    };

    let mut parser = Parser::new(tokens);
    match parser.parse_value() {
        Ok(v) => Ok(v),
        Err(msg) => Err(format!("Parse error: {}", msg)),
    }
}

fn main() {
    let mut input = String::new();
    if let Err(err) = io::stdin().read_to_string(&mut input) {
        eprintln!("Failed to read input: {}", err);
        exit(1);
    }

    match parse_json(&input) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::JsonValue;
    use crate::Lexer;
    use crate::Token;
    use crate::parse_json;

    #[test]
    fn test_empty_object() {
        let result = parse_json("{}");
        assert_eq!(result, Ok(JsonValue::Object(BTreeMap::new())));
    }

    #[test]
    fn test_invalid_object() {
        let result = parse_json("{");
        let expected_result = String::from("Parse error: Unexpected token 'EOF' at position 1");
        assert_eq!(result, Err(expected_result));
    }

    #[test]
    fn test_lexer_empty_object() {
        let mut lexer = Lexer::new("{}");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 3); // LBrace, RBrace, EOF
        match &tokens[0] {
            Token::LBrace => {}
            _ => panic!("Expected LBrace"),
        }
        match &tokens[1] {
            Token::RBrace => {}
            _ => panic!("Expected RBrace"),
        }
        match &tokens[2] {
            Token::EOF => {}
            _ => panic!("Expected EOF"),
        }
    }
}
