#[derive(Debug)]
#[derive(Clone)]
pub enum Token {
    Identifier(String),
    Number(i32),
    Keyword(Keyword),
    Operand(Operand),
    Semicolon,
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