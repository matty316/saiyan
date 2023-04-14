use crate::token::{Token, TokenType};

struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    #[must_use] fn new(input: String) -> Self {
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

    fn next_token(&mut self) -> Token {
        let tok: Token;
        match self.ch {
            b'=' => tok = Self::new_token(TokenType::Assign, self.ch),
            b';' => tok = Self::new_token(TokenType::Semicolon, self.ch),
            b'(' => tok = Self::new_token(TokenType::LParen, self.ch),
            b')' => tok = Self::new_token(TokenType::RParen, self.ch),
            b',' => tok = Self::new_token(TokenType::Comma, self.ch),
            b'+' => tok = Self::new_token(TokenType::Plus, self.ch),
            b'{' => tok = Self::new_token(TokenType::LBrace, self.ch),
            b'}' => tok = Self::new_token(TokenType::RBrace, self.ch),
            b'\0' => {
                let token = Token { token_type: TokenType::EOF, literal: String::new() };
                tok = token;
            }
            _ => unreachable!(), 
        }

        self.read_char();
        tok
    }
 
    fn new_token(token_type: TokenType, ch: u8) -> Token {
        let string = String::from_utf8(vec![ch]).unwrap();
        Token { token_type, literal: string }
    }
}
#[test]
fn test_next_token() {
    let input = "=+(){},;".to_string();
    let tests = vec![
        Token { token_type: TokenType::Assign, literal: "=".to_string() },
        Token { token_type: TokenType::Plus, literal: "+".to_string() },
        Token { token_type: TokenType::LParen, literal: "(".to_string() },
        Token { token_type: TokenType::RParen, literal: ")".to_string() },
        Token { token_type: TokenType::LBrace, literal: "{".to_string() },
        Token { token_type: TokenType::RBrace, literal: "}".to_string() },
        Token { token_type: TokenType::Comma, literal: ",".to_string() },
        Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
        Token { token_type: TokenType::EOF, literal: String::new() },
    ];

    let mut lexer = Lexer::new(input);

    for t in tests {
        let token = lexer.next_token();
        assert_eq!(t, token);
    }
}
