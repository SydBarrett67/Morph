#[derive(Debug, Clone)]
pub enum Statement {

    // Scope
    Scope(Vec<Statement>),

    // Variable declaration
    Let(
        Expression, // Variable name
        Type,       // Declared type
        Expression  // Variable value
    ),

    // Variabile assignment
    Assign(
        Expression,
        Expression
    )
}
#[derive(Debug, Clone)]
pub enum Type {
    INT,
    BOOL,
    STRING
}


#[derive(Debug, Clone)]
pub enum Expression {
    // Identifier (variables etc.)
    Identifier(String),

    // Literal (any)
    Literal(
        String
    ),

    // Binary expression
    BinaryExpression(
        Operator,
        Box<Expression>,
        Box<Expression>,
    ),
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}