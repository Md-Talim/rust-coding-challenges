#[derive(Debug, PartialEq)]
pub enum Token {
    LBrace,
    RBrace,
    Colon,
    Comma,
    StringLit(String),
    EOF,
}
