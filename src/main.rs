use lexer::Lexer;
use parser::Parser;

mod lexer;
mod library;
mod token;
mod parser;

fn main() {
        let source = String::from("LET foo = bar * 3 + 2\nIF foo > 0 THEN\nPRINT \"yes!\"\nENDIF");
        let mut lexer = Lexer::init(source);
        let mut parser = Parser::init(lexer);
        parser.program();
}                      