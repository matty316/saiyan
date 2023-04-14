use crate::ast::*;
use crate::lexer::*;
use crate::token::*;

struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    #[must_use] fn new(mut lexer: Lexer) -> Self {
        let token = lexer.next_token();
        let mut p = Self {
            lexer,
            current_token: token.clone(),
            peek_token: token.clone(),
        };
        
        p.next_token();
        p.next_token();
        
        p
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&self) -> Option<Program> {
        None
    }
}

#[test]
fn test_let_statement() {
    let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
    ".to_string();

    let lexer = Lexer::new(input);
    let parser = Parser::new(lexer);

    let program = parser.parse_program();

    match program {
        Some(p) => {
            assert_eq!(p.statements.len(), 3);
        }
        None => assert!(false),
    }
}
