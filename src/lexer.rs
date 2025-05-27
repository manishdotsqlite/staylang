use crate::{library::nth_char, token::{Token, TokenType}};

pub struct Lexer {
        pub source: String,
        pub cur_char: char,
        pub cur_pos: i32,
}

impl Lexer {
    pub fn init(source: String) -> Self {
        Lexer { source: source, cur_char: '0', cur_pos: -1 }
    }

    pub fn nextChar(&mut self) {
        self.cur_pos = self.cur_pos + 1;
        if self.cur_pos >= self.source.len() as i32 {
                self.cur_char = '\0'; 
        } else {
                self.cur_char = nth_char(&self.source, self.cur_pos);
        }
    }

    pub fn peek(&self) -> char {
        if self.cur_pos + 1 >= self.source.len() as i32{
                return '\0'
        } 
        nth_char(&self.source, self.cur_pos + 1)
    }


    pub fn get_token(&mut self) -> Token {
        self.skip_whitespace();
        self.skip_comment();
        let token: Token;
        if self.cur_char == '\0' {
            token = Token::new(self.cur_char.to_string(), TokenType::EOF);
        } else if self.cur_char == '+' {
            token = Token::new(self.cur_char.to_string(), TokenType::PLUS);
        } else if self.cur_char == '-' {
            token = Token::new(self.cur_char.to_string(), TokenType::MINUS);
        } else if self.cur_char == '*' {
            token = Token::new(self.cur_char.to_string(), TokenType::ASTERISK);
        } else if self.cur_char == '/' {
            token = Token::new(self.cur_char.to_string(), TokenType::SLASH);
        } else if self.cur_char == '\n' {
            token = Token::new(self.cur_char.to_string(), TokenType::NEWLINE);
        } else if self.cur_char == '=' {
                if self.peek() == '=' {
                        let last_char = self.cur_char;
                        self.nextChar();
                        token = Token::new(last_char.to_string() + &self.cur_char.to_string(), TokenType::EQEQ);
                } else {
                        token = Token::new(self.cur_char.to_string(), TokenType::EQ);
                }
        }  else if self.cur_char == '>' {
                if self.peek() == '=' {
                        let last_char = self.cur_char;
                        self.nextChar();
                        token = Token::new(last_char.to_string() + &self.cur_char.to_string(), TokenType::GTEQ);
                } else {
                        token = Token::new(self.cur_char.to_string(), TokenType::GT);
                }
        } else if self.cur_char == '<' {
                if self.peek() == '=' {
                        let last_char = self.cur_char;
                        self.nextChar();
                        token = Token::new(last_char.to_string() + &self.cur_char.to_string(), TokenType::LTEQ);
                } else {
                        token = Token::new(self.cur_char.to_string(), TokenType::LT);
                }
        } else if self.cur_char == '!' {
                if self.peek() == '=' {
                        let last_char = self.cur_char;
                        self.nextChar();
                        token = Token::new(last_char.to_string() + &self.cur_char.to_string(), TokenType::NOTEQ);
                } else {
                        panic!("Unexpected character: {}", self.cur_char);
                }
        } else if self.cur_char  == '"' {
                self.nextChar();
                let start_pos = self.cur_pos;
                while self.cur_char != '"' {
                        if self.cur_char == '\r' || self.cur_char == '\n' || self.cur_char == '\t' || self.cur_char == '%' {
                                panic!("Illegal character in string: {}", self.cur_char);
                        }
                        self.nextChar();
                }
                let toktext = &self.source[start_pos as usize..self.cur_pos as usize];
                token = Token::new(toktext.to_string(), TokenType::STRING);
        } else if self.cur_char.is_alphabetic() {
                let start_pos = self.cur_pos;
                while self.peek().is_alphanumeric() {
                        self.nextChar();
                }

                let toktext = &self.source[start_pos as usize..(self.cur_pos+1) as usize];
                match self.check_if_keyword(toktext) {
                        Ok(token_type) => {
                                token = Token::new(toktext.to_string(), token_type);
                        },
                        Err(_) => {
                                token = Token::new(toktext.to_string(), TokenType::IDENT);
                        },
                };

        } else if self.cur_char.is_numeric(){
                let start_pos = self.cur_pos;
                while self.peek().is_numeric() {
                        self.nextChar();
                }
                if self.peek() == '.' {
                        self.nextChar();

                        if !self.peek().is_numeric() {
                                panic!("Illegal character in number: {}", self.peek());
                        }
                        while self.peek().is_numeric() {
                                self.nextChar();
                        }
                }

                let toktext = &self.source[start_pos as usize..(self.cur_pos + 1_) as usize];
                token = Token::new(toktext.to_string(), TokenType::NUMBER);
        } else {
                panic!("Unexpected character: {}", self.cur_char);
        }
        self.nextChar();
        token
    }

    pub fn skip_whitespace(&mut self) {
        while self.cur_char.is_whitespace() || self.cur_char == '\t' || self.cur_char == '\r' {
            self.nextChar();
        }
    }

    pub fn skip_comment(&mut self) {
        if self.cur_char == '#' {
            while self.cur_char != '\n' && self.cur_char != '\0' {
                self.nextChar();
            }
        }
    }

    pub fn check_if_keyword(&self, tokentext: &str) -> Result<TokenType, ()> {
        match tokentext.to_uppercase().as_str() {
            "LABEL" => Ok(TokenType::LABEL),
            "GOTO" => Ok(TokenType::GOTO),
            "PRINT" => Ok(TokenType::PRINT),
            "INPUT" => Ok(TokenType::INPUT),
            "LET" => Ok(TokenType::LET),
            "IF" => Ok(TokenType::IF),
            "THEN" => Ok(TokenType::THEN),
            "ENDIF" => Ok(TokenType::ENDIF),
            "WHILE" => Ok(TokenType::WHILE),
            "REPEAT" => Ok(TokenType::REPEAT),
            "ENDWHILE" => Ok(TokenType::ENDWHILE),
            _ => Err(())
        }
    }

}

