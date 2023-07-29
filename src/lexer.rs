#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    // Identifiers + literals
    Identifier(String),
    Int(i64),

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

    // Keywrds
    Fn,
    Let,
    True,
    False,
    If,
    Else,
    Return,

    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Misc
    Illegal,
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub offset: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, offset: usize) -> Self {
        Self {
            token_type,
            lexeme,
            offset,
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    source: String,
    char: Option<char>,
    current_pos: usize,

    /// offset of current token lexeme
    start: usize,

    /// Marks if lexer finished consuming source characters.
    /// Terminates the Iterator.
    finished: bool,
}

impl Lexer {
    // TODO: Can we call this .as_bytes() only once?

    pub fn new(source: String) -> Self {
        let mut lexer = Self {
            source,
            char: None,
            current_pos: 0,
            start: 0,
            finished: false,
        };

        lexer
    }

    /// Reads the next char from source.
    fn read_char(&mut self) {
        self.char = self
            .source
            .as_bytes()
            .get(self.current_pos)
            .map(|c| (*c).into());

        self.current_pos += 1;
    }

    /// Returns the next character to be consumed.  
    fn peek_char(&self) -> Option<char> {
        self.source
            .as_bytes()
            .get(self.current_pos)
            .map(|c| (*c).into())
    }

    /// Picks next token type conditionally based on next character.
    fn token_if(
        &mut self,
        next: char,
        token_type1: TokenType,
        token_type2: TokenType,
        prev: char,
    ) -> Token {
        match self.peek_char() {
            Some(c) if c == next => {
                self.read_char();
                Token::new(token_type1, prev.to_string() + &c.to_string(), self.start)
            }
            Some(_) | None => self.token(token_type2),
        }
    }

    /// Builds a token of given type from current character.
    fn token(&self, token_type: TokenType) -> Token {
        if let Some(c) = self.char {
            return Token::new(token_type, c.to_string(), self.start);
        }

        Token::new(token_type, "".to_string(), self.start)
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        self.read_char();

        while let Some(c) = self.char {
            if c == ' ' || c == '\n' {
                self.read_char();
            } else {
                break;
            }
        }

        self.start = self.current_pos - 1;

        let token = if let Some(c) = self.char {
            match c {
                '+' => self.token(TokenType::Plus),
                '-' => self.token(TokenType::Minus),
                '*' => self.token(TokenType::Asterisk),
                '/' => self.token(TokenType::Slash),
                '<' => self.token(TokenType::LesserThan),
                '>' => self.token(TokenType::GreaterThan),
                '!' => self.token_if('=', TokenType::NotEq, TokenType::Bang, c),
                '=' => self.token_if('=', TokenType::Eq, TokenType::Assign, c),
                _ => self.token(TokenType::Illegal),
            }
        } else {
            println!("Finished processing all tokens");
            self.finished = true;
            Token::new(TokenType::Eof, String::new(), self.start)
        };

        Some(token)
    }
}
