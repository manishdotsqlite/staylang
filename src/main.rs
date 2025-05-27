use lexer::Lexer;
use token::TokenType;

mod lexer;
mod library;
mod token;

fn main() {
        let source = String::from("PRINT \"How many fibonacci numbers do you want?\" \nINPUT nums \nLET a = 0 \nLET b = 1");
        let mut lexer = Lexer::init(source);
        lexer.nextChar();
        
        loop {
                let token = lexer.get_token();
                if token.token_type == TokenType::EOF {
                        break;
                }
                println!("Token: {:?}, Literal: {}", token.token_type, token.literal);
        }
}                      