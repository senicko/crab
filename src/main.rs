mod lexer;
mod repl;
mod token;

fn main() {
    println!("Hello! This is the Crab programming language!");
    repl::start_repl();
}
