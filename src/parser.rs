use crate::ast::{Expression, Operator};
use crate::grammar::{ Operand, Token, Keyword };
use crate::ast::Statement;

#[derive(Debug)]
pub enum ParserError {
    SyntaxError(String),
}

pub struct Parser {
    tokens: Vec<Token>,
}
impl Parser {
    // Constructor
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
        }
    }

    pub fn peek(&self, look_ahead: usize) -> Option<&Token> {
        self.tokens.get(look_ahead)
    }

    pub fn consume(&mut self) -> Token {
        self.tokens.remove(0)
    }

    pub fn parse_factor(&mut self) -> Result<Expression, ParserError> {
        match self.peek(0) {

            Some(Token::Number(value)) => {
                self.consume();
                Ok(Expression::Number(*value))
            }

            _ => Err(
                ParserError::SyntaxError(
                    String::from("ParserError: expected factor.")
                )
            )
        }
    }

    pub fn parse_term(&mut self) -> Result<Expression, ParserError> {
        let lhv = self.parse_factor()?;
        match self.peek(0) {

            Some(Token::Operand(Operand::STAR)) => {
                self.consume();
                Ok(
                    Expression::BinaryExpression(
                        Operator::Mul,
                        Box::new(lhv),
                        Box::new(self.parse_factor()?)
                    )
                )
            }

            _ => { Err(
                ParserError::SyntaxError(
                    String::from("ParserError: expected term.")
                )
            )}
        }
    }

    pub fn parse_expr(&mut self) -> Result<Expression, ParserError> {
        let lhv = self.parse_term()?;
        match self.peek(0) {
            Some(Token::Semicolon) => {
                Ok(lhv)
            }

            Some(Token::Operand(Operand::PLUS)) => {
                self.consume();
                let rhv = self.parse_term()?;
                Ok(
                    Expression::BinaryExpression(
                        Operator::Add,
                        Box::new(lhv),
                        Box::new(rhv)
                    )
                )
            }

            _ => { Err(
                ParserError::SyntaxError(
                    String::from("ParserError: invalid expression.")
                )
            ) }
        }
    }

    pub fn parse(&mut self) -> Result<Statement, ParserError> {
        let mut root_node: Statement = Statement::Root(Vec::new());

        let mut statement: Statement;
        while !self.tokens.is_empty() {
            let token = self.consume();

            match token {
                Token::Keyword(Keyword::LET) => {
                    match (
                        self.consume(),
                        self.consume(),
                    )
                    {
                        (
                            Token::Identifier(name),
                            Token::Operand(Operand::EQUALS),
                        ) => {
                            let value = self.parse_expr()?;

                            statement =
                                Statement::Let(
                                    Expression::Identifier(name),
                                    value,
                                )
                        }

                        _ => {
                            return Err(ParserError::SyntaxError(
                                String::from("ParserError: unexpected use of 'let' Keyword.")
                            ));
                        }
                    }
                }

                _ => { 
                    return Err(ParserError::SyntaxError(
                        String::from("ParserError: invalid syntax.")
                    )); 
                }
            }

            if let Statement::Root(nodes) = &mut root_node {
                nodes.push(
                    statement
                )
            }
        }
        Ok(root_node)
    }

}