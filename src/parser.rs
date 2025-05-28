use std::collections::HashSet;

use crate::{lexer::Lexer, token::{Token, TokenType}};

pub struct Parser {
      lexer: Lexer,
      cur_token: Token,
      peek_token: Token,  
      symbols: HashSet<String>,
      labels_declared: HashSet<String>,
      labels_gotoed: HashSet<String>
}


impl Parser {

        pub fn check_token(&self, token_type: TokenType) -> bool {
                self.cur_token.token_type == token_type
        }

        pub fn check_peek(&self, token_type: TokenType) -> bool {
                self.peek_token.token_type == token_type
        }

        pub fn match_token_type(&mut self, token_type: TokenType) {
                if !self.check_token(token_type) {
                        panic!("Expected token type: {:?}, found: {:?}", token_type, self.cur_token.token_type);
                }
                self.next_token();
        }

        pub fn next_token(&mut self) {
                self.cur_token = self.peek_token.clone();
                self.peek_token = self.lexer.get_token();
        }

        pub fn init(mut lexer: Lexer) -> Self {
                lexer.nextChar();
                let mut parser = Parser {
                        lexer,
                        cur_token: Token::new(String::new(), TokenType::EOF),
                        peek_token: Token::new(String::new(), TokenType::EOF),
                        symbols: HashSet::new(),
                        labels_declared: HashSet::new(),
                        labels_gotoed: HashSet::new(),
                };
                parser.next_token();
                parser.next_token();
                parser
        }



        pub fn program(&mut self) {
                println!("PROGRAM");

                while self.check_token(TokenType::NEWLINE) {
                        self.next_token();
                }

                while !self.check_token(TokenType::EOF) {
                        self.statement();
                }

                for label in self.labels_declared.iter() {

                }
        }


        pub fn statement(&mut self) {
                if self.check_token(TokenType::PRINT) {
                        println!("STATEMENT-PRINT");
                        self.next_token();

                        if self.check_token(TokenType::STRING) {
                                self.next_token();
                        } else {
                                self.expression();
                        }

                } else if self.check_token(TokenType::IF){
                        println!("STATEMENT-IF");
                        self.next_token();
                        self.comparison();

                        self.match_token_type(TokenType::THEN);
                        self.nl();

                        while !self.check_token(TokenType::ENDIF) {
                                self.statement();
                        }

                        self.match_token_type(TokenType::ENDIF);

                } else if self.check_token(TokenType::WHILE) {
                        println!("STATEMENT-WHILE");
                        self.next_token();
                        self.comparison();

                        self.match_token_type(TokenType::REPEAT);
                        self.nl();

                        while !self.check_token(TokenType::ENDWHILE) {
                                self.statement();
                        }
                        self.match_token_type(TokenType::ENDWHILE);
                        
                } else if self.check_token(TokenType::LABEL) {
                        println!("STATEMENT-LABEL");
                        self.next_token();

                        if self.labels_declared.contains(&self.cur_token.literal) {
                                panic!("Label already exists: {}", self.cur_token.literal);
                        }
                        self.labels_declared.insert(self.cur_token.literal.clone());
                        self.match_token_type(TokenType::IDENT);

                } else if self.check_token(TokenType::GOTO) {
                        println!("STATEMENT-GOTO");
                        self.next_token();
                        self.match_token_type(TokenType::IDENT);

                } else if self.check_token(TokenType::LET) {
                        println!("STATEMENT-LET");
                        self.next_token();
                        self.match_token_type(TokenType::IDENT);
                        self.match_token_type(TokenType::EQ);
                        self.expression();

                } else if self.check_token(TokenType::INPUT) {
                        println!("STATEMENT-INPUT");
                        self.next_token();
                        self.match_token_type(TokenType::IDENT);
                } else {
                        panic!("Invalid statement at: {}", self.cur_token.literal);
                }

                self.nl();
        }

        pub fn nl(&mut self) {
                println!("NEWLINE");

                self.match_token_type(TokenType::NEWLINE);
                while self.check_token(TokenType::NEWLINE) {
                        self.next_token();
                }
        }

        pub fn comparison(&mut self) {
                println!("COMPARISON");
                self.expression();

                if self.isComparisonOperator() {
                        self.next_token();
                        self.expression();
                } else {
                        panic!("Expected Operator at: {}", self.cur_token.literal);
                }

                while self.isComparisonOperator() {
                        self.next_token();
                        self.expression();
                }
        }

        fn isComparisonOperator(&self) -> bool {
                self.check_token(TokenType::GT) || self.check_token(TokenType::GTEQ) || self.check_token(TokenType::LT) || self.check_token(TokenType::LTEQ) ||  self.check_token(TokenType::EQEQ) || self.check_token(TokenType::NOTEQ)
        }

        pub fn expression(&mut self) {
                println!("EXPRESSION");

                self.term();
                while self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
                        self.next_token();
                        self.term();
                }
                
        }

        pub fn term(&mut self) {
                println!("TERM");

                self.unary();
                while self.check_token(TokenType::ASTERISK) || self.check_token(TokenType::SLASH) {
                        self.next_token();
                        self.unary();
                }

        }

        pub fn unary(&mut self) {
                println!("UNARY");

                if self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
                        self.next_token();
                }
                self.primary();
        }

        pub fn primary(&mut self) {
                println!("PRIMARY ( {:?} )", self.cur_token.literal);

                if self.check_token(TokenType::NUMBER) {
                        self.next_token();
                } else if self.check_token(TokenType::IDENT) {
                        self.next_token();
                } else {
                        panic!("Unexpected token at: {}", self.cur_token.literal);
                }
        }


} 
