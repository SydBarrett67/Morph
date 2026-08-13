#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Number(i32),
    Keyword(Keyword),
    Operand(Operand),
    Semicolon,
}

#[derive(Debug)]
pub enum Keyword {
    LET,
}

#[derive(Debug)]
pub enum Operand {
    EQUALS,
    PLUS,
}