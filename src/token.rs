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
    Minus,
    Bang,
    Asterisk,
    Slash,
    LesserThan,
    GreaterThan,
    Eq,
    NotEq,

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
    True,
    False,
    If,
    Else,
    Return,
}

impl TokenType {
    pub fn resolve_type(ident: &str) -> TokenType {
        match ident {
            "fn" => TokenType::Fn,
            "let" => TokenType::Let,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,
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
