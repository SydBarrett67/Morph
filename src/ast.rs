pub enum Node {
    // Root node
    Root(Vec<Node>),

    // Variable declaration
    Let(
        Identifier(String),
        Expression
    ),

    Expression
}

pub enum Expression {
    // Simple int
    Number(i32),

    // Binary expression
    BinaryExpression(Operator, Expression, Expression),
}

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}