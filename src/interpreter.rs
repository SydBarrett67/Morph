use crate::ast::{ Expression, Statement, Operator };

use crate::scope::{ ASTScope };
use crate::typechecker::{Type, TypeChecker, Value};

use std::{collections::HashMap, fmt::{Error, format}};

#[derive(Debug)]
pub enum InterpreterError {
    RuntimeError(String)
}

pub struct Interpreter {
    ast: Statement,
    symbols: Vec<ASTScope>,
    pub values: Vec<HashMap<String, Value>>,
    depth: usize
}

impl Interpreter {
    // Constructor
    pub fn new(ast: Statement, symbols: Vec<ASTScope>) -> Self {
        Self {
            ast: ast,
            symbols: symbols,
            values: vec![HashMap::new()],
            depth: 0
        }
    }

    pub fn eval_expr(&self, statement: &Expression) -> Result<Value, InterpreterError> {
        match statement {

            // Literal (any type)
            Expression::Literal(value) => {
                Ok(Value::String(value.clone()))
            }

            // Idenfier, resolve
            Expression::Identifier(name) => {
                let scope = match self.values.get(self.depth) {
                    Some(scope) => scope,
                    None => {
                        return Err(
                            InterpreterError::RuntimeError(
                                String::from("InterpreterError: invalid scope.")
                            )
                        );
                    }
                };

                let value = match scope.get(name) {
                    Some(value) => value,
                    None => {
                        return Err(
                            InterpreterError::RuntimeError(
                                format!(
                                    "InterpreterError: variable '{}' not found.",
                                    name
                                )
                            )
                        );
                    }
                };

                Ok(value.clone())
            }

            // Recursive BinExpr
            Expression::BinaryExpression(op, lhv , rhv ) => {

                let lhs = self.eval_expr(lhv)?;
                let rhs = self.eval_expr(rhv)?;
                match (lhs, rhs) {
                    (Value::Int(lhs), Value::Int(rhs)) => {
                        match op {
                            Operator::Add => Ok(Value::Int(lhs + rhs)),
                            Operator::Sub => Ok(Value::Int(lhs - rhs)),
                            Operator::Mul => Ok(Value::Int(lhs * rhs)),
                            Operator::Div => Ok(Value::Int(lhs / rhs)),
                            _ => Err(
                                InterpreterError::RuntimeError(
                                    String::from("InterpreterError: invalid operator for int.")
                                )
                            )
                        }
                    }

                    (Value::String(lhs), Value::String(rhs)) => {
                        match op {
                            Operator::Add => Ok(Value::String(lhs + &rhs)),
                            _ => Err(
                                InterpreterError::RuntimeError(
                                    String::from("InterpreterError: invalid operator for string.")
                                )
                            )
                        }
                    }

                    _ => Err(
                        InterpreterError::RuntimeError(
                            String::from("InterpreterError: incompatible types.")
                        )
                    )
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
        match &self.ast {
            // Enter root AST node
            Statement::Root(statements) => {

                // Cicle through every statement
                for statement in statements {

                    // Pattern matching for statement types
                    match statement {

                        // Variable declaration
                        Statement::Let(
                            Expression::Identifier(name),
                            ..,
                            value
                        ) => {
                            let value = self.eval_expr(value)?;

                            self.values
                                .get_mut(self.depth)
                                .unwrap()
                                .insert(
                                    name.clone(),
                                    value,
                                );
                        }

                        _ => return Err(InterpreterError::RuntimeError(
                            String::from("InterpreterError: invalid statement.")
                        ))
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