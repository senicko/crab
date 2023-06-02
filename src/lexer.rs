use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    ch: char,
    position: usize,
    read_position: usize,
}

impl Default for Lexer {
    fn default() -> Self {
        Self {
            input: String::from(""),
            ch: 0 as char,
            position: 0,
            read_position: 0,
        }
    }
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            ..Default::default()
        };

        // We are pre-loading the first character.
        lexer.read_char();

        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => Token::from_char(TokenType::Assign, self.ch),
            ';' => Token::from_char(TokenType::Semicolon, self.ch),
            '(' => Token::from_char(TokenType::LParen, self.ch),
            ')' => Token::from_char(TokenType::RParen, self.ch),
            ',' => Token::from_char(TokenType::Comma, self.ch),
            '+' => Token::from_char(TokenType::Plus, self.ch),
            '{' => Token::from_char(TokenType::LBrace, self.ch),
            '}' => Token::from_char(TokenType::RBrace, self.ch),
            _ => {
                // Handle longer lexemes
                if Lexer::is_letter(self.ch) {
                    let literal = self.read_identifier();

                    return Token {
                        token_type: TokenType::resolve_type(&literal),
                        literal: Some(literal),
                    };
                } else if Lexer::is_digit(self.ch) {
                    return Token {
                        token_type: TokenType::Int,
                        literal: Some(self.read_number()),
                    };
                }

                if self.ch == 0 as char {
                    Token {
                        token_type: TokenType::Eof,
                        literal: None,
                    }
                } else {
                    Token {
                        token_type: TokenType::Illegal,
                        literal: None,
                    }
                }
            }
        };

        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> String {
        let start_position = self.position;

        while Lexer::is_digit(self.ch) {
            self.read_char();
        }

        String::from(&self.input[start_position..self.position])
    }

    fn read_identifier(&mut self) -> String {
        let start_position = self.position;

        while Lexer::is_letter(self.ch) {
            self.read_char();
        }

        String::from(&self.input[start_position..self.position])
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn is_letter(c: char) -> bool {
        c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }
}

#[cfg(test)]
mod test {
    use crate::token::{Token, TokenType};

    use super::Lexer;

    #[test]
    fn test_next_token() {
        let input = String::from(
            "let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y;
};
let result = add(five, ten);
",
        );

        let tests: Vec<Token> = vec![
            Token::from_str(TokenType::Let, "let"),
            Token::from_str(TokenType::Ident, "five"),
            Token::from_str(TokenType::Assign, "="),
            Token::from_str(TokenType::Int, "5"),
            Token::from_str(TokenType::Semicolon, ";"),
            Token::from_str(TokenType::Let, "let"),
            Token::from_str(TokenType::Ident, "ten"),
            Token::from_str(TokenType::Assign, "="),
            Token::from_str(TokenType::Int, "10"),
            Token::from_str(TokenType::Semicolon, ";"),
            Token::from_str(TokenType::Let, "let"),
            Token::from_str(TokenType::Ident, "add"),
            Token::from_str(TokenType::Assign, "="),
            Token::from_str(TokenType::Fn, "fn"),
            Token::from_str(TokenType::LParen, "("),
            Token::from_str(TokenType::Ident, "x"),
            Token::from_str(TokenType::Comma, ","),
            Token::from_str(TokenType::Ident, "y"),
            Token::from_str(TokenType::RParen, ")"),
            Token::from_str(TokenType::LBrace, "{"),
            Token::from_str(TokenType::Ident, "x"),
            Token::from_str(TokenType::Plus, "+"),
            Token::from_str(TokenType::Ident, "y"),
            Token::from_str(TokenType::Semicolon, ";"),
            Token::from_str(TokenType::RBrace, "}"),
            Token::from_str(TokenType::Semicolon, ";"),
            Token::from_str(TokenType::Let, "let"),
            Token::from_str(TokenType::Ident, "result"),
            Token::from_str(TokenType::Assign, "="),
            Token::from_str(TokenType::Ident, "add"),
            Token::from_str(TokenType::LParen, "("),
            Token::from_str(TokenType::Ident, "five"),
            Token::from_str(TokenType::Comma, ","),
            Token::from_str(TokenType::Ident, "ten"),
            Token::from_str(TokenType::RParen, ")"),
            Token::from_str(TokenType::Semicolon, ";"),
            Token {
                token_type: TokenType::Eof,
                literal: None,
            },
        ];

        let mut lexer = Lexer::new(input);

        for (i, test) in tests.iter().enumerate() {
            let token = lexer.next_token();

            assert_eq!(
                test.literal, token.literal,
                "[{}] want={:?}, got={:?}",
                i, test.literal, token.literal
            );

            assert_eq!(
                test.token_type, token.token_type,
                "[{}] want={:?}, got={:?}",
                i, test.token_type, token.token_type
            );
        }
    }
}
