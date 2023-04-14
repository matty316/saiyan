#[derive(PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Illegal, EOF,

    Ident, Int, Assign, Plus, Comma, Semicolon, LParen, RParen,
    LBrace, RBrace, Function, Let
}
