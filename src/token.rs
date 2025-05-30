
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
        EOF = -1,
        NEWLINE = 0,
	NUMBER = 1,
	IDENT = 2,
	STRING = 3,
	
        // KEYWORDS
	LABEL = 101,
	GOTO = 102,
	PRINT = 103,
	INPUT = 104,
	LET = 105,
	IF = 106,
	THEN = 107,
	ENDIF = 108,
	WHILE = 109,
	REPEAT = 110,
	ENDWHILE = 111,
	
        // OPERATORS
	EQ = 201,  
	PLUS = 202,
	MINUS = 203,
	ASTERISK = 204,
	SLASH = 205,
	EQEQ = 206,
	NOTEQ = 207,
	LT = 208,
	LTEQ = 209,
	GT = 210,
	GTEQ = 211,

}


#[derive(Debug, Clone)]
pub struct Token {
        pub literal: String,
        pub token_type: TokenType,
}

impl Token {
    pub fn new(literal: String, token_type: TokenType) -> Self {
        Token { literal, token_type }
    }
}