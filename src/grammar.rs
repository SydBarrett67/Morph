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
    // Variable names, 
    Identifier(String),
    Literal(String),
    Keyword(Keyword),
    Operand(Operand),

    // Single character tokens
    Semicolon, Colon,
    // ()
    OpenParen, CloseParen,
    // {}
    OpenBrace, CloseBrace
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Keyword {
    LET,
    // Conditional
    IF, ELSE,
    // Loops
    WHILE,
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