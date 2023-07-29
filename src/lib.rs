use std::io;

mod lexer;

pub fn start_prompt() {
    loop {
        let mut source = String::new();

        io::stdin()
            .read_line(&mut source)
            .expect("Failed to read in the source.");

        let lexer = lexer::Lexer::new(source);

        println!("{:?}", lexer.into_iter().collect::<Vec<_>>());
    }
}
