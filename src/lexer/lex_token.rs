#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType<'a> {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Colon,
    Semicolon,
    Slash,
    Backslash,
    Star,
    Mod,
    Hashtag,

    ListIndexer,
    MapIndexer,
    GridIndexer,
    ArrayIndexer,

    LogicalAnd,
    LogicalOr,
    LogicalXor,

    BinaryAnd,
    BinaryOr,
    BinaryXor,

    Bang,
    Hook,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Macro,
    RegionBegin,
    RegionEnd,

    Var,
    If,
    Else,
    Return,
    For,
    Repeat,
    While,
    Do,
    Until,
    Switch,
    Case,
    DefaultCase,
    Break,
    True,
    False,

    AndAlias,
    OrAlias,
    NotAlias,
    ModAlias,
    Div,

    Identifier(&'a str),
    String(&'a str),
    Number(&'a str),

    Comment(&'a str),
    MultilineComment(&'a str),
    EOF,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub line_number: u32,
    pub column_number: u32,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, line_number: u32, column_number: u32) -> Token {
        Token {
            token_type,
            line_number,
            column_number,
        }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn line_number(&self) -> &u32 {
        &self.line_number
    }
}

use std::fmt;
impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token {:#?} on {}:{}.",
            self.token_type, self.line_number, self.column_number
        )
    }
}
