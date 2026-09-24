#[derive(Debug, Clone)]
pub enum Statement {
    // Scope
    Scope(Vec<Statement>),

    /*
    
        VARIABLE DECLARATION AND HANDLING

    */
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
    ),

    /*
    
        FLOW CONTROL

    */
    If(
        Expression,
        Box<Statement>
    ),
    While(
        Expression,
        Box<Statement>
    ),

    /*
    
        FUNCTIONS

    */
    // Function declaration
    FuncDecl(
        String,                 // Func name
        Type,           // Return type (optional)
        Option<Vec<Expression>> // Arguments (optional)
    )
}
#[derive(Debug, Clone)]
pub enum Type {
    INT,
    BOOL,
    STRING,


    // VOID type
    VOID
}


#[derive(Debug, Clone)]
pub enum Expression {
    // Identifier (variables etc.)
    Identifier {
        name: String,
        ty: Option<Type>,
    },

    // Function parameter
    Parameter(
        String,
        Type
    ),

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
    // Classical operator
    Add,
    Sub,
    Mul,
    Div,

    // Boolean operator
    Or,
    And,

    // Comparators
    Less,
    More,
    EqComp
}