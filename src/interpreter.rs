use std::{collections::HashMap, fmt::Error};

use crate::ast::{ Expression, Statement, Operator };

#[derive(Debug)]
pub enum InterpreterError {
    Error(String)
}

pub struct Interpreter {
    ast: Statement,
    pub variables: HashMap<String, i32>,
}

impl Interpreter {
    pub fn new(ast: Statement) -> Self {
        Self {
            ast: ast,
            variables: HashMap::new(),
        }
    }

    pub fn eval_expr(&self, statement: &Expression) -> Result<i32, InterpreterError> {
        match statement {

            Expression::Number(value) => {
                Ok(*value)
            }

            Expression::BinaryExpression(op, lhv , rhv ) => {
                match op {
                    Operator::Add => {
                        Ok(self.eval_expr(lhv)? + self.eval_expr(rhv)?)
                    }

                    _ => { Err(
                        InterpreterError::Error(
                            String::from("InterpreterError: Binary expression error.")
                        )
                    ) }
                }
            }


            _ => { Err(
                InterpreterError::Error(
                    String::from("InterpreterError: Binary expression error.")
                )
            ) }
        }
    }

    pub fn interpret(&mut self) -> Result<(), InterpreterError>{
        match self.ast {
            Statement::Root(ref nodes) => {
                // Main loop
                for statement in nodes {
                    println!("\n\nSTATEMENT: {:?}", statement);
                    match statement {
                        Statement::Let(
                            Expression::Identifier(var_name),
                            expr
                        ) => {
                            self.variables.insert(
                                String::from(var_name),
                                self.eval_expr(&expr)?
                            );
                        }

                        _ => { 
                            return Err(InterpreterError::Error(
                                String::from("InterpreterError: execution failed.")
                            ));
                        }
                    }
                }

                Ok(())
            }
            _ => { Err(InterpreterError::Error(
                String::from("InterpreterError: AST root node error.")   
            )) }
        }
    }
}