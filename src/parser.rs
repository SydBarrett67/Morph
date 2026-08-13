use crate::ast::Expression;
use crate::grammar::{Operand, Token, Keyword};
use crate::ast::Node;

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

    pub fn parse(&mut self) -> Result<Node, ParserError> {
        let mut root_node: Node = Node::Root(Vec::new());

        let mut statement: Node;
        while !self.tokens.is_empty() {
            let token = self.consume();

            match token {
                Token::Keyword(Keyword::LET) => {
                    match (
                        self.consume(),
                        self.consume(),
                        self.consume(),
                        self.consume()
                    )
                    {
                        (
                            Token::Identifier(name),
                            Token::Operand(Operand::EQUALS),
                            Token::Number(value),
                            Token::Semicolon
                        ) => {
                            statement =
                                Node::Let(
                                    Expression::Identifier(name),
                                    Expression::Number(value)
                                )
                        }

                        _ => {
                            return Err(ParserError::SyntaxError(
                                String::from("Unexpected use of 'let' Keyword.")
                            ));
                        }
                    }
                }


                _ => { 
                    return Err(ParserError::SyntaxError(
                        String::from("Invalid syntax.")
                    )); 
                }
            }

            if let Node::Root(nodes) = &mut root_node {
                nodes.push(
                    statement
                )
            }
        }
        Ok(root_node)
    }

}