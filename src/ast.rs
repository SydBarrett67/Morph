#[derive(Debug)]
pub enum Statement {
    // Root Statement
    Root(Vec<Statement>),

    // Variable declaration
    Let(
        Expression,
        Expression
    ),
    // Variabile assignment
    Assign(
        Expression,
        Expression
    )
}
#[derive(Debug)]
pub enum Expression {
    // Identifier (variables etc.)
    Identifier(String),

    // Simple int
    Number(i32),

    // Binary expression
    BinaryExpression(
        Operator,
        Box<Expression>,
        Box<Expression>,
    ),
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}