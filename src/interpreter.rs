use crate::ast::{ Expression, Statement, Operator };

use std::{collections::HashMap, fmt::{Error, format}};

#[derive(Debug)]
pub enum InterpreterError {
    RuntimeError(String)
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

            // Simple number
            Expression::Number(value) => {
                Ok(*value)
            }


            // Recursive BinExpr
            Expression::BinaryExpression(op, lhv , rhv ) => {
                match op {
                    Operator::Add => {
                        Ok(self.eval_expr(lhv)? + self.eval_expr(rhv)?)
                    }

                    Operator::Sub => {
                        Ok(self.eval_expr(lhv)? - self.eval_expr(rhv)?)
                    }

                    Operator::Mul => {
                        Ok(self.eval_expr(lhv)? * self.eval_expr(rhv)?)
                    }

                    Operator::Div => {
                        Ok(self.eval_expr(lhv)? / self.eval_expr(rhv)?)
                    }

                    _ => { Err(
                        InterpreterError::RuntimeError(
                            String::from("InterpreterError: invalid binary operator.")
                        )
                    ) }
                }
            }


            _ => { Err(
                InterpreterError::RuntimeError(
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

                        Statement::Assign(
                            Expression::Identifier(var_name),
                            expr
                        ) => {
                            if self.variables.contains_key(var_name) {
                                self.variables.insert(
                                    String::from(var_name),
                                    self.eval_expr(expr)?
                                );
                            }
                            else {
                                return Err(InterpreterError::RuntimeError(
                                    String::from(format!(
                                        "InterpreterError: {} was never declared.", var_name
                                    ))
                                ))
                            }
                        }

                        _ => { 
                            return Err(InterpreterError::RuntimeError(
                                String::from("InterpreterError: execution failed.")
                            ));
                        }
                    }
                }

                Ok(())
            }
            _ => { Err(InterpreterError::RuntimeError(
                String::from("InterpreterError: AST root node RuntimeError.")   
            )) }
        }
    }
}