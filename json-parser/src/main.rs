mod ast;
mod lexer;
mod parser;
mod token;

use crate::lexer::Lexer;
use crate::parser::Parser;
use std::io::{self, Read};
use std::process::exit;

fn parse_json(input: &str) -> Result<ast::JsonValue, String> {
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

    use crate::ast::JsonValue;
    use crate::lexer::Lexer;
    use crate::parse_json;
    use crate::token::Token;

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
