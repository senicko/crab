#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Misc
    Illegal,
    Eof,

    // Identifiers + literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keyowrds
    Fn,
    Let,
}

impl TokenType {
    pub fn resolve_type(ident: &str) -> TokenType {
        match ident {
            "fn" => TokenType::Fn,
            "let" => TokenType::Let,
            _ => TokenType::Ident,
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Option<String>,
}

impl Token {
    pub fn from_char(token_type: TokenType, literal: char) -> Token {
        Token {
            token_type,
            literal: Some(String::from(literal)),
        }
    }

    pub fn from_str(token_type: TokenType, literal: &str) -> Token {
        Token {
            token_type,
            literal: Some(String::from(literal)),
        }
    }
}
