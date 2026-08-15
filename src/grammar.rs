#[derive(Debug)]
#[derive(Clone)]

pub struct TokenInfo {
    pub token: Token,
    pub pos: Position
}
#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize
}

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Number(i32),
    Keyword(Keyword),
    Operand(Operand),

    // Single character tokens
    Semicolon,
    OpenParen,
    CloseParen,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Keyword {
    LET,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Operand {
    EQUALS,
    PLUS,
    MINUS,
    STAR,
    SLASH
}