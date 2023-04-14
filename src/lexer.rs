use crate::token::{Token, TokenType};
use std::collections::HashMap;

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    #[must_use] pub fn new(input: String) -> Self {
        Self {
            input: input.as_bytes().to_vec(),
            position: 0,
            read_position: 1,
            ch: input.as_bytes()[0],
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = b'\0';
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    } 

    pub fn next_token(&mut self) -> Token {
        let tok: Token;
        self.skip_whitespace();
        match self.ch {
            b'=' => {
                if self.peek() == b'=' {
                    self.read_char();
                    tok = Token { token_type: TokenType::EQ, literal: "==".to_string() };
                } else {
                    tok = Self::new_token(TokenType::Assign, self.ch);
                }
            }
            b'!' => {
                if self.peek() == b'=' {
                    self.read_char();
                    tok = Token { token_type: TokenType::NotEQ, literal: "!=".to_string() };
                } else {
                    tok = Self::new_token(TokenType::Bang, self.ch);
                }
            }
            b';' => tok = Self::new_token(TokenType::Semicolon, self.ch),
            b'(' => tok = Self::new_token(TokenType::LParen, self.ch),
            b')' => tok = Self::new_token(TokenType::RParen, self.ch),
            b',' => tok = Self::new_token(TokenType::Comma, self.ch),
            b'+' => tok = Self::new_token(TokenType::Plus, self.ch),
            b'-' => tok = Self::new_token(TokenType::Minus, self.ch),
            b'/' => tok = Self::new_token(TokenType::Slash, self.ch),
            b'*' => tok = Self::new_token(TokenType::Star, self.ch),
            b'<' => tok = Self::new_token(TokenType::LT, self.ch),
            b'>' => tok = Self::new_token(TokenType::GT, self.ch),
            b'{' => tok = Self::new_token(TokenType::LBrace, self.ch),
            b'}' => tok = Self::new_token(TokenType::RBrace, self.ch),
            b'\0' => {
                tok = Token { token_type: TokenType::EOF, literal: String::new() };
            }
            _ => {
                if self.is_letter() {
                    let literal = self.read_identifier();
                    let token = Token { token_type: lookup_ident(literal.clone()), literal: literal };
                    return token;
                } else if self.ch.is_ascii_digit() {
                    let token = Token { token_type: TokenType::Int, literal: self.read_number() };
                    return token;
                } else {
                    tok = Self::new_token(TokenType::Illegal, self.ch);
                }
            } 
        }

        self.read_char();
        tok
    }
 
    fn new_token(token_type: TokenType, ch: u8) -> Token {
        let string = String::from_utf8(vec![ch]).unwrap();
        Token { token_type, literal: string }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter() {
            self.read_char()
        }
        let bytes = self.input[position..self.position].to_vec();
        String::from_utf8(bytes).unwrap()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        let bytes = self.input[position..self.position].to_vec();
        String::from_utf8(bytes).unwrap()
    }

    fn is_letter(&self) -> bool {
        self.ch.is_ascii_alphabetic() || self.ch == b'_'
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }

    fn peek(&self) -> u8 {
        if self.read_position >= self.input.len() {
            b'\0'
        } else {
            self.input[self.read_position]
        }
    }
}

fn keywords() -> HashMap<String, TokenType> {
    HashMap::from([
        ("fn".to_string(), TokenType::Function),
        ("let".to_string(), TokenType::Let),
        ("true".to_string(), TokenType::True),
        ("false".to_string(), TokenType::False),
        ("if".to_string(), TokenType::If),
        ("else".to_string(), TokenType::Else),
        ("return".to_string(), TokenType::Return ),
    ])
}

fn lookup_ident(ident: String) -> TokenType {
    match keywords().get(&ident) {
        Some(&t) => t,
        None => TokenType::Ident,
    }
}

#[test]
fn test_next_token() {
    let input = "let five = 5;
    let ten = 10;

    let add = fn(x, y) {
        x + y;
    };

    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;

    if (5 < 10) {
        return true;
    } else {
        return false;
    }

    10 == 10;
    10 != 9;
    ".to_string();
    let tests = vec![
        Token { token_type: TokenType::Let, literal: "let".to_string() },
        Token { token_type: TokenType::Ident, literal: "five".to_string() },
        Token { token_type: TokenType::Assign, literal: "=".to_string() },
        Token { token_type: TokenType::Int, literal: "5".to_string() },
        Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
        Token { token_type: TokenType::Let, literal: "let".to_string() },
        Token { token_type: TokenType::Ident, literal: "ten".to_string() },
        Token { token_type: TokenType::Assign, literal: "=".to_string() },
        Token { token_type: TokenType::Int, literal: "10".to_string() },
        Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
        Token { token_type: TokenType::Let, literal: "let".to_string() },
        Token { token_type: TokenType::Ident, literal: "add".to_string() },
        Token { token_type: TokenType::Assign, literal: "=".to_string() },
        Token { token_type: TokenType::Function, literal: "fn".to_string() },
        Token { token_type: TokenType::LParen, literal: "(".to_string() },
        Token { token_type: TokenType::Ident, literal: "x".to_string() },
        Token { token_type: TokenType::Comma, literal: ",".to_string() },
        Token { token_type: TokenType::Ident, literal: "y".to_string() },
        Token { token_type: TokenType::RParen, literal: ")".to_string() },
        Token { token_type: TokenType::LBrace, literal: "{".to_string() },
        Token { token_type: TokenType::Ident, literal: "x".to_string() },
        Token { token_type: TokenType::Plus, literal: "+".to_string() },
        Token { token_type: TokenType::Ident, literal: "y".to_string() },
        Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
        Token { token_type: TokenType::RBrace, literal: "}".to_string() },
        Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
        Token { token_type: TokenType::Let, literal: "let".to_string() },
        Token { token_type: TokenType::Ident, literal: "result".to_string() },
        Token { token_type: TokenType::Assign, literal: "=".to_string() },
        Token { token_type: TokenType::Ident, literal: "add".to_string() },
        Token { token_type: TokenType::LParen, literal: "(".to_string() },
        Token { token_type: TokenType::Ident, literal: "five".to_string() },
        Token { token_type: TokenType::Comma, literal: ",".to_string() },
        Token { token_type: TokenType::Ident, literal: "ten".to_string() },
        Token { token_type: TokenType::RParen, literal: ")".to_string() },
        Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
        Token { token_type: TokenType::Bang, literal: "!".to_string() },
        Token { token_type: TokenType::Minus, literal: "-".to_string() },
        Token { token_type: TokenType::Slash, literal: "/".to_string() },
        Token { token_type: TokenType::Star, literal: "*".to_string() },
        Token { token_type: TokenType::Int, literal: "5".to_string() },
        Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
        Token { token_type: TokenType::Int, literal: "5".to_string() },
        Token { token_type: TokenType::LT, literal: "<".to_string() },
        Token { token_type: TokenType::Int, literal: "10".to_string() },
        Token { token_type: TokenType::GT, literal: ">".to_string() },
        Token { token_type: TokenType::Int, literal: "5".to_string() },
        Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
        Token { token_type: TokenType::If, literal: "if".to_string() },
        Token { token_type: TokenType::LParen, literal: "(".to_string() },
        Token { token_type: TokenType::Int, literal: "5".to_string() },
        Token { token_type: TokenType::LT, literal: "<".to_string() },
        Token { token_type: TokenType::Int, literal: "10".to_string() },
        Token { token_type: TokenType::RParen, literal: ")".to_string() },
        Token { token_type: TokenType::LBrace, literal: "{".to_string() },
        Token { token_type: TokenType::Return, literal: "return".to_string() },
        Token { token_type: TokenType::True, literal: "true".to_string() },
        Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
        Token { token_type: TokenType::RBrace, literal: "}".to_string() },
        Token { token_type: TokenType::Else, literal: "else".to_string() },
        Token { token_type: TokenType::LBrace, literal: "{".to_string() },
        Token { token_type: TokenType::Return, literal: "return".to_string() },
        Token { token_type: TokenType::False, literal: "false".to_string() },
        Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
        Token { token_type: TokenType::RBrace, literal: "}".to_string() },
        Token { token_type: TokenType::Int, literal: "10".to_string() },
        Token { token_type: TokenType::EQ, literal: "==".to_string() },
        Token { token_type: TokenType::Int, literal: "10".to_string() },
        Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
        Token { token_type: TokenType::Int, literal: "10".to_string() },
        Token { token_type: TokenType::NotEQ, literal: "!=".to_string() },
        Token { token_type: TokenType::Int, literal: "9".to_string() },
        Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
        Token { token_type: TokenType::EOF, literal: String::new() },
    ];

    let mut lexer = Lexer::new(input);

    for (i, t) in tests.iter().enumerate() {
        let token = lexer.next_token();
        assert_eq!(*t, token, "failed at index {}", i);
    }
}
