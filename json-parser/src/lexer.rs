use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
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
