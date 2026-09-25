use crate::ast::Literal::{ self };
use crate::ast::{Expression, Operator, Parameter, Statement, Type };
use crate::grammar::{Keyword, Operand, Position, Token, TokenInfo};

#[derive(Debug)]
pub enum ParserError {
    SyntaxError(
        String,
        Position
    ),   
}

pub struct Parser {
    tokens: Vec<TokenInfo>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> Option<&TokenInfo> {
        self.tokens.get(self.position)
    }

    fn consume(&mut self) -> Result<TokenInfo, ParserError> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Ok(token)
        } else {
            Err(ParserError::SyntaxError(
                String::from("ParserError: unexpected EOF."),
                Position {
                    line: 0,
                    column: 0
                }
            ))
        }
    }

    pub fn parse_args(&mut self) -> Result<Vec<Parameter>, ParserError> {
        let mut arguments = Vec::new();

        match self.consume()? {
            TokenInfo {
                token: Token::OpenParen,
                ..
            } => {}

            token => {
                return Err(ParserError::SyntaxError(
                    String::from("ParserError: expected '('."),
                    token.pos
                ));
            }
        }

        loop {
            match self.peek() {
                Some(TokenInfo {
                    token: Token::CloseParen,
                    ..
                }) => {
                    self.consume()?;
                    break;
                }

                Some(TokenInfo {
                    token: Token::Identifier(name),
                    ..
                }) => {
                    let name = name.clone();
                    self.consume()?;

                    let ty = self.parse_type()?;

                    arguments.push(
                        Parameter {
                            name,
                            ty
                        }
                    );

                    match self.peek() {
                        Some(TokenInfo {
                            token: Token::Comma,
                            ..
                        }) => {
                            self.consume()?;
                        }

                        Some(TokenInfo {
                            token: Token::CloseParen,
                            ..
                        }) => {}

                        Some(token) => {
                            return Err(ParserError::SyntaxError(
                                String::from("ParserError: expected ',' or ')'."),
                                token.pos.clone()
                            ));
                        }

                        None => {
                            return Err(ParserError::SyntaxError(
                                String::from("ParserError: expected ')', found EOF."),
                                Position {
                                    line: 0,
                                    column: 0
                                }
                            ));
                        }
                    }
                }

                Some(token) => {
                    return Err(ParserError::SyntaxError(
                        String::from("ParserError: expected argument."),
                        token.pos.clone()
                    ));
                }

                None => {
                    return Err(ParserError::SyntaxError(
                        String::from("ParserError: expected ')', found EOF."),
                        Position {
                            line: 0,
                            column: 0
                        }
                    ));
                }
            }
        }

        Ok(arguments)
    }
    pub fn parse_type(&mut self) -> Result<Type, ParserError> {
        match self.peek() {
            Some(TokenInfo {
                token: Token::Colon,
                ..
            }) => {
                self.consume()?;
            }

            _ => {
                return Err(ParserError::SyntaxError(
                    String::from("ParserError: expected ':'."),
                    Position {
                        line: 0,
                        column: 0
                    }
                ));
            }
        }

        match self.consume()? {
            TokenInfo {
                token: Token::Identifier(ty),
                pos
            } => {
                match ty.as_str() {
                    "int" => Ok(Type::INT),
                    "string" => Ok(Type::STRING),
                    "bool" => Ok(Type::BOOL),

                    _ => Err(ParserError::SyntaxError(
                        String::from("ParserError: unknown type."),
                        pos
                    ))
                }
            }

            token => {
                Err(ParserError::SyntaxError(
                    String::from("ParserError: expected type."),
                    token.pos
                ))
            }
        }
    }

    pub fn parse_factor(&mut self) -> Result<Expression, ParserError> {
        match self.peek() {
            // Literal
            Some(TokenInfo {
                token: Token::Literal(value),
                ..
            }) => {
                let value = value.to_owned();
                self.consume()?;

                // Boolean
                let literal = if value == "true" {
                    Literal::Bool(true)
                } else if value == "false" {
                    Literal::Bool(false)
                } 
                // Int
                else if let Ok(num) = value.parse::<i32>() {
                    Literal::Int(num)
                } 
                // String
                else {
                    Literal::String(value)
                };

                Ok(Expression::Literal(literal))
            }

            // ()
            Some(TokenInfo {
                token: Token::OpenParen,
                ..
            }) => {
                self.consume()?;

                let expr = self.parse_expr()?;

                match self.peek() {
                    Some(TokenInfo {
                        token: Token::CloseParen,
                        ..
                    }) => {
                        self.consume()?;
                        Ok(expr)
                    }

                    Some(token) => Err(ParserError::SyntaxError(
                        String::from("expected ')'."),
                        token.pos.clone()
                    )),
                    None => Err(ParserError::SyntaxError(
                        String::from("ParserError: expected ')', found EOF."),
                        Position {
                            line: 0,
                            column: 0
                        }
                    )),
                }
            }

            // Identifier (var / func)
            Some(TokenInfo {
                token: Token::Identifier(name),
                ..
            }) => {
                let name = name.clone();
                self.consume()?;

                // If '(' => func
                match self.peek() {
                    Some(TokenInfo {
                        token: Token::OpenParen,
                        ..
                    }) => {
                        
                        let mut arguments: Vec<Expression>;
                        while let Some(token_info) = self.peek() {
                            arguments.push(self.parse_expr()?);
                        }

                        Ok(Expression::FuncCall( name, Box::new(arguments) ))
                    }

                    _ => Ok(Expression::Identifier { name: name, ty: None })
                }               
            }

            Some(token) => Err(ParserError::SyntaxError(
                String::from(
                    format!("unexpected token: {:?}", token.token)
                ),
                token.pos.clone()
            )),

            None => Err(ParserError::SyntaxError(
                String::from("ParserError: expected factor, found EOF."),
                Position {
                    line: 0,
                    column: 0,
                }
            )),
        }
    }
    pub fn parse_term(&mut self) -> Result<Expression, ParserError> {
        let mut lhs = self.parse_factor()?;

        while let Some(token_info) = self.peek() {
            let operator = match &token_info.token {
                Token::Operand(Operand::STAR)   => Operator::Mul,
                Token::Operand(Operand::SLASH)  => Operator::Div,

                Token::Operand(Operand::OR)     => Operator::Or,
                _ => break,
            };

            let operator = operator;
            self.consume()?;

            let rhs = self.parse_factor()?;

            lhs = Expression::BinaryExpression(
                operator,
                Box::new(lhs),
                Box::new(rhs),
                None
            );
        }

        Ok(lhs)
    }
    pub fn parse_expr(&mut self) -> Result<Expression, ParserError> {
        let mut lhs = self.parse_term()?;

        while let Some(token_info) = self.peek() {
            let operator = match &token_info.token {
                // Classical operands
                Token::Operand(Operand::PLUS)   => Operator::Add,
                Token::Operand(Operand::MINUS)  => Operator::Sub,

                // Boolean operands
                Token::Operand(Operand::AND)    => Operator::And,

                // Comparators
                Token::Operand(Operand::LESS)   => Operator::Less,
                Token::Operand(Operand::MORE)   => Operator::More,
                Token::Operand(Operand::EQCOMP) => Operator::EqComp,

                _ => break,
            };

            self.consume()?;

            let rhs = self.parse_term()?;

            lhs = Expression::BinaryExpression(
                operator,
                Box::new(lhs),
                Box::new(rhs),
                None
            );
        }

        Ok(lhs)
    }

    pub fn parse_stmt(&mut self) -> Result<Statement, ParserError> {
        let token = self.consume()?;

        let statement = match token.token {
            // Variable declaration
            Token::Keyword(Keyword::LET) => {
                let identifier = self.consume()?;

                let name = match identifier.token {
                    Token::Identifier(name) => name,

                    _ => {
                        return Err(ParserError::SyntaxError(
                            String::from("ParserError: expected identifier after 'let'."),
                            identifier.pos
                        ));
                    }
                };

                let declared_type = self.parse_type()?;

                let equals = self.consume()?;

                match equals.token {
                    Token::Operand(Operand::EQUALS) => {}

                    _ => {
                        return Err(ParserError::SyntaxError(
                            String::from("ParserError: expected '=' after type."),
                            equals.pos
                        ));
                    }
                }

                let value = self.parse_expr()?;

                match self.peek() {
                    Some(TokenInfo {
                        token: Token::Semicolon,
                        ..
                    }) => {
                        self.consume()?;
                    }

                    Some(token) => {
                        return Err(ParserError::SyntaxError(
                            String::from("ParserError: expected ';' at statement end."),
                            token.pos.clone()
                        ));
                    }

                    None => {
                        return Err(ParserError::SyntaxError(
                            String::from("ParserError: expected ';', found EOF."),
                            Position {
                                line: 0,
                                column: 0
                            }
                        ));
                    }
                }

                Statement::Let(
                    name,
                    declared_type,
                    value,
                )
            }

            // Assignment
            Token::Identifier(name) => {
                let equals = self.consume()?;

                match equals.token {
                    Token::Operand(Operand::EQUALS) => {}

                    _ => {
                        return Err(ParserError::SyntaxError(
                            String::from(
                                "ParserError: expected '=' after identifier.",
                            ),
                            Position {
                                line: 0,
                                column: 0,
                            }
                        ));
                    }
                }

                let value = self.parse_expr()?;

                match self.peek() {
                    Some(TokenInfo {
                        token: Token::Semicolon,
                        ..
                    }) => {
                        self.consume()?;
                    }

                    _ => {
                        return Err(ParserError::SyntaxError(
                            String::from(
                                "ParserError: expected ';' at statement end.",
                            ),
                            token.pos.clone()
                        ));
                    }
                }

                Statement::Assign(
                    name,
                    value,
                )
            }
            
            // Scope
            Token::OpenBrace => {
                let mut statements = Vec::new();
                while !matches!(
                    self.peek(),
                    Some(TokenInfo {
                        token: Token::CloseBrace,
                        ..
                    })
                ) {
                    statements.push(
                        self.parse_stmt()?
                    )
                }
                self.consume()?;
                Statement::Scope(statements)
            }

            // If
            Token::Keyword(Keyword::IF) => {
                let expr = self.parse_expr()?;

                let scope = self.parse_stmt()?;

                Statement::If(expr, Box::new(scope))
            }

            // While
            Token::Keyword(Keyword::WHILE) => {
                let expr = self.parse_expr()?;

                let scope = match self.parse_stmt()? {
                    Statement::Scope(statements) => Statement::Scope(statements),
                    _ => {
                        return Err(
                            ParserError::SyntaxError(
                                String::from("While body must be a scope"),
                                Position {
                                    line: 0,
                                    column: 0,
                                }
                            )
                        );
                    }
                };

                Statement::While(expr, Box::new(scope))
            }

            // Function declaration
            Token::Keyword(Keyword::FUNC) => {
                // Name
                let identifier = self.consume()?;
                let name = match identifier.token {
                    Token::Identifier(name) => name,
                    _ => {
                        return Err(ParserError::SyntaxError(
                            String::from(
                                "ParserError: expected identifier after 'func'.",
                            ),
                            Position {
                                line: 0,
                                column: 0,
                            }
                        ));
                    }
                };

                // Arguments
                let arguments = self.parse_args()?;

                // Type
                let declared_type = match self.peek() {
                    Some(TokenInfo {
                        token: Token::Colon,
                        ..
                    }) => self.parse_type()?,

                    Some(TokenInfo {
                        token: Token::OpenBrace,
                        ..
                    }) => Type::VOID,

                    Some(token) => {
                        return Err(ParserError::SyntaxError(
                            String::from("ParserError: expected ':' or '{'."),
                            token.pos.clone()
                        ));
                    }

                    None => {
                        return Err(ParserError::SyntaxError(
                            String::from("ParserError: expected return type or '{'."),
                            Position {
                                line: 0,
                                column: 0
                            }
                        ));
                    }
                };

                // Scope
                let scope = match self.parse_stmt()? {
                    Statement::Scope(statements) => Statement::Scope(statements),

                    _ => {
                        return Err(ParserError::SyntaxError(
                            String::from("Function body must be a scope."),
                            Position {
                                line: 0,
                                column: 0
                            }
                        ));
                    }
                };

                Statement::FuncDecl(
                    name,
                    declared_type,
                    arguments,
                    Box::new(scope)
                )
            }

            _ => {
                return Err(ParserError::SyntaxError(
                    String::from("ParserError: invalid statement."),
                    Position {
                        line: 0,
                        column: 0,
                    }
                ));
            }
        };

        Ok(statement)
    }

    pub fn parse(&mut self) -> Result<Statement, ParserError> {
        let mut nodes = Vec::new();


        while self.position < self.tokens.len() {
            
            let statement = self.parse_stmt()?;

            nodes.push(statement);
        }

        Ok(Statement::Scope(nodes))
    }

}