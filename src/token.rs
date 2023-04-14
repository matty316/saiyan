#[derive(PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    Illegal, EOF,

    Ident, Int, Assign, Plus, Minus, Bang, Star, Slash, LT, GT,

    Comma, Semicolon, LParen, RParen, LBrace, RBrace, Function, Let,
    True, False, If, Else, Return
}
 
