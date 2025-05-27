use crate::library::nth_char;

pub struct Lexer {
        source: String,
        cur_char: char,
        cur_pos: i32,
}

impl Lexer {
    pub fn init(source: String) -> Self {
        Lexer { source: source, cur_char: ' ', cur_pos: -1 }
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

}