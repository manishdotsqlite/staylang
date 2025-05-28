use std::collections::HashSet;

use crate::{emitter::Emitter, lexer::Lexer, token::{Token, TokenType}};

pub struct Parserr {
      lexer: Lexer,
      cur_token: Token,
      peek_token: Token,  
      symbols: HashSet<String>,
      labels_declared: HashSet<String>,
      labels_gotoed: HashSet<String>,
      emitter: Emitter
}


impl Parserr {

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

        pub fn init(mut lexer: Lexer, emitter: Emitter) -> Self {
                lexer.nextChar();
                let mut parser = Parserr {
                        lexer,
                        cur_token: Token::new(String::new(), TokenType::EOF),
                        peek_token: Token::new(String::new(), TokenType::EOF),
                        symbols: HashSet::new(),
                        labels_declared: HashSet::new(),
                        labels_gotoed: HashSet::new(),
                        emitter: emitter
                };
                parser.next_token();
                parser.next_token();
                parser
        }

        pub fn return_emitter(&self) -> Emitter {
                self.emitter.clone()
        }


        pub fn program(&mut self) {
                self.emitter.header_line("#include <stdio.h>");
                self.emitter.header_line("int main(void) {");

                while self.check_token(TokenType::NEWLINE) {
                        self.next_token();
                }

                while !self.check_token(TokenType::EOF) {
                        self.statement();
                }

                self.emitter.emit_line("return 0;");
                self.emitter.emit_line("}");

                for label in self.labels_gotoed.iter() {
                        if !self.labels_declared.contains(label) {
                                panic!("Attempting to GOTO to undeclared label: {}", label);
                        }
                }
        }


        pub fn statement(&mut self) {
                if self.check_token(TokenType::PRINT) {
                        self.next_token();

                        if self.check_token(TokenType::STRING) {
                                self.emitter.emit_line(&format!("printf(\"{}\\n\");", self.cur_token.literal));
                                self.next_token();
                        } else {
                                self.emitter.emit(&format!("printf(\"%.2f\\n\", (float)("));
                                self.expression();
                                self.emitter.emit_line("));");
                        }

                } else if self.check_token(TokenType::IF){
                        self.next_token();
                        self.emitter.emit_line("if(");
                        self.comparison();

                        self.match_token_type(TokenType::THEN);
                        self.nl();
                        self.emitter.emit_line("){");

                        while !self.check_token(TokenType::ENDIF) {
                                self.statement();
                        }

                        self.match_token_type(TokenType::ENDIF);
                        self.emitter.emit_line("}");

                } else if self.check_token(TokenType::WHILE) {
                        self.next_token();
                        self.emitter.emit("while(");
                        self.comparison();

                        self.match_token_type(TokenType::REPEAT);
                        self.nl();
                        self.emitter.emit_line("){");

                        while !self.check_token(TokenType::ENDWHILE) {
                                self.statement();
                        }
                        self.match_token_type(TokenType::ENDWHILE);
                        self.emitter.emit_line("}");
                        
                } else if self.check_token(TokenType::LABEL) {
                        self.next_token();

                        if self.labels_declared.contains(&self.cur_token.literal) {
                                panic!("Label already exists: {}", self.cur_token.literal);
                        }
                        self.labels_declared.insert(self.cur_token.literal.clone());
                        self.emitter.emit_line(&(self.cur_token.literal.clone() + ":"));
                        self.match_token_type(TokenType::IDENT);

                } else if self.check_token(TokenType::GOTO) {
                        self.next_token();
                        self.labels_gotoed.insert(self.cur_token.literal.clone());
                        self.emitter.emit_line(&format!("goto {};", self.cur_token.literal));
                        self.match_token_type(TokenType::IDENT);

                } else if self.check_token(TokenType::LET) {
                        self.next_token();

                        if !self.symbols.contains(&self.cur_token.literal) {
                                self.symbols.insert(self.cur_token.literal.clone());
                                self.emitter.header_line(&format!("float {};", self.cur_token.literal));
                        }

                        self.emitter.emit(&(self.cur_token.literal.clone() + " = "));
                        self.match_token_type(TokenType::IDENT);
                        self.match_token_type(TokenType::EQ);
                        self.expression();
                        self.emitter.emit_line(";");

                } else if self.check_token(TokenType::INPUT) {
                        self.next_token();

                        if !self.symbols.contains(&self.cur_token.literal) {
                                self.symbols.insert(self.cur_token.literal.clone());
                                self.emitter.header_line(&format!("float {} ;", self.cur_token.literal));
                        }

                        self.emitter.emit_line(&format!("if(0 == scanf(\"%f\", &{})) {{", self.cur_token.literal));
                        self.emitter.emit_line(&format!("{} = 0;", self.cur_token.literal));
                        self.emitter.emit("scanf(\"%");
                        self.emitter.emit_line("*s\");");
                        self.emitter.emit_line("}");
                        self.match_token_type(TokenType::IDENT);
                } else {
                        panic!("Invalid statement at: {}", self.cur_token.literal);
                }

                if !self.check_token(TokenType::EOF) && !self.check_token(TokenType::ENDIF) && !self.check_token(TokenType::ENDWHILE) {
                        self.nl();
                }
        }

        pub fn nl(&mut self) {
                // println!("NEWLINE");

                self.match_token_type(TokenType::NEWLINE);
                // println!("WHAT");
                while self.check_token(TokenType::NEWLINE) {
                        self.next_token();
                }
                
        }

        pub fn comparison(&mut self) {
                // println!("COMPARISON");
                self.expression();

                if self.isComparisonOperator() {
                        self.emitter.emit(&self.cur_token.literal);
                        self.next_token();
                        self.expression();
                } else {
                        panic!("Expected Operator at: {}", self.cur_token.literal);
                }

                while self.isComparisonOperator() {
                        self.emitter.emit(&self.cur_token.literal);
                        self.next_token();
                        self.expression();
                }
        }

        fn isComparisonOperator(&self) -> bool {
                self.check_token(TokenType::GT) || self.check_token(TokenType::GTEQ) || self.check_token(TokenType::LT) || self.check_token(TokenType::LTEQ) ||  self.check_token(TokenType::EQEQ) || self.check_token(TokenType::NOTEQ)
        }

        pub fn expression(&mut self) {
                // println!("EXPRESSION");

                self.term();
                while self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
                        self.emitter.emit(&self.cur_token.literal);
                        self.next_token();
                        self.term();
                }
                
        }

        pub fn term(&mut self) {
                // println!("TERM");

                self.unary();
                while self.check_token(TokenType::ASTERISK) || self.check_token(TokenType::SLASH) {
                        self.emitter.emit(&self.cur_token.literal);
                        self.next_token();
                        self.unary();
                }

        }

        pub fn unary(&mut self) {
                // println!("UNARY");

                if self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
                        self.emitter.emit(&self.cur_token.literal);
                        self.next_token();
                }
                self.primary();
        }

        pub fn primary(&mut self) {
                // println!("PRIMARY ( {:?} )", self.cur_token.literal);

                if self.check_token(TokenType::NUMBER) {
                        self.emitter.emit(&self.cur_token.literal);
                        self.next_token();
                } else if self.check_token(TokenType::IDENT) {
                        if !self.symbols.contains(&self.cur_token.literal) {
                                panic!("Referencing variable before assignment: {}", self.cur_token.literal);
                        }
                        self.emitter.emit(&self.cur_token.literal);
                        self.next_token();
                } else {
                        panic!("Unexpected token at: {}", self.cur_token.literal);
                }
        }


} 
