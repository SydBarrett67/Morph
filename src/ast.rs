use std::string;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub ty: Type,
}


#[derive(Debug, Clone)]
pub enum Statement {
    // Scope
    Scope(Vec<Statement>),

    // Variable declaration
    Let(
        String,     // Variable name
        Type,       // Declared type
        Expression  // Variable value
    ),

    // Variabile assignment
    Assign(
        String,
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
    Identifier {
        name: String,
        symbol: Option<Symbol>,
        ty: Option<Type>,
    },

    // Literal (any)
    Literal(Literal),

    // Binary expression
    BinaryExpression(
        Operator,
        Box<Expression>,
        Box<Expression>,
        Option<Type>
    ),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i32),
    Bool(bool),
    String(String)
}


#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}