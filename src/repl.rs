use crate::lexer::Lexer;
use crate::token::TokenType;
use std::io::{self, Write};

pub fn start_repl() {
    loop {
        print!(">> ");
        io::stdout().flush().expect("failed to flush");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read the line");

        let mut lexer = Lexer::new(line);
        let mut token = lexer.next_token();

        while token.token_type != TokenType::Eof {
            println!("{:?}", token);
            token = lexer.next_token();
        }
    }
}
