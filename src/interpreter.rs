use std::thread::scope;

use crate::ast::{ Expression, Statement, Operator, Literal };

use crate::scope::{self, RuntimeScope};

#[derive(Debug)]
pub enum InterpreterError {
    RuntimeError(String)
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
    String(String)
}

pub struct Interpreter {
    ast: Statement,
    pub env: RuntimeScope,
    depth: usize
}

impl Interpreter {
    // Constructor
    pub fn new(ast: Statement) -> Self {
        Self {
            ast: ast,
            env: RuntimeScope::new(),
            depth: 0
        }
    }

    pub fn eval_expr(&self, statement: &Expression) -> Result<Value, InterpreterError> {
        match statement {

            // Literal (any type)
            Expression::Literal(literal) => {
                match literal {
                    Literal::Int(number) => Ok(Value::Int(*number)),
                    Literal::String(string) => Ok(Value::String(string.to_string())),
                    Literal::Bool(bool_value) => Ok(Value::Bool(*bool_value)),
                }
            }

            // Idenfier, resolve
            Expression::Identifier{
                name,
                ..
            } => {
                
                let value = match self.env.get_var(name.to_string(), self.depth) {
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
            Expression::BinaryExpression(op, lhv, rhv, _ ) => {

                let lhs = self.eval_expr(lhv)?;
                let rhs = self.eval_expr(rhv)?;
                match (lhs, rhs) {
                    // Allowed operation on Int data type
                    (Value::Int(lhs), Value::Int(rhs)) => {
                        match op {
                            Operator::Add   => Ok(Value::Int(lhs + rhs)),
                            Operator::Sub   => Ok(Value::Int(lhs - rhs)),
                            Operator::Mul   => Ok(Value::Int(lhs * rhs)),
                            Operator::Div   => Ok(Value::Int(lhs / rhs)),

                            Operator::More  => Ok(Value::Bool(lhs > rhs)),
                            Operator::Less  => Ok(Value::Bool(lhs < rhs)),
                            
                            _ => Err(
                                InterpreterError::RuntimeError(
                                    String::from("InterpreterError: invalid operator for int.")
                                )
                            )
                        }
                    }
                    // Allowed operation on String data type
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
                    // Allowed operation on Bool data type
                    (Value::Bool(lhs), Value::Bool(rhs)) => {
                        match op {
                            Operator::Or    => Ok(Value::Bool(lhs || rhs)),
                            Operator::And   => Ok(Value::Bool(lhs && rhs)),
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

    pub fn interpret(&mut self) -> Result<(), InterpreterError> {

        // Push global scope
        self.env.scopes.push(RuntimeScope::new());

        match &self.ast {

            Statement::Scope(statements) => {

                let statements = statements.clone();

                self.interpret_scope(&statements)

            }

            _ => {
                Err(InterpreterError::RuntimeError(
                    String::from("InterpreterError: AST root node RuntimeError.")
                ))
            }

        }

    }

    fn interpret_scope(
        &mut self,
        statements: &Vec<Statement>
    ) -> Result<(), InterpreterError> {

        // Cicle through every statement
        for statement in statements {

            match statement {

                Statement::Let(
                    name,
                    ..,
                    value
                ) => {

                    let value = self.eval_expr(value)?;

                    self.env.push_var(
                        name.to_string(),
                        value,
                        self.depth
                    );

                }

                Statement::Assign(name, expr) => {
                    let value = self.eval_expr(expr)?;

                    if !self.env.set_var(
                        name.to_string(),
                        value,
                        self.depth
                    ) {
                        return Err(
                            InterpreterError::RuntimeError(
                                format!("InterpreterError: variable '{}' not found.", name)
                            )
                        );
                    }
                }

                Statement::Scope(statements) => {

                    self.env.scopes.push(RuntimeScope::new());
                    self.depth += 1;

                    self.interpret_scope(statements)?;

                    self.env.scopes.pop();
                    self.depth -= 1;

                }

                // If statement
                Statement::If(
                    expr,
                    scope
                ) => {
                    let value = self.eval_expr(expr)?;

                    if let Value::Bool(true) = value {
                        match scope.as_ref() {
                            Statement::Scope(statements) => {
                                self.env.scopes.push(RuntimeScope::new());
                                self.depth += 1;

                                self.interpret_scope(statements)?;

                                self.env.scopes.pop();
                                self.depth -= 1;
                            }

                            _ => {
                                return Err(
                                    InterpreterError::RuntimeError(
                                        String::from("InterpreterError: 'if' body must be a scope.")
                                    )
                                );
                            }
                        }
                    }
                }

                // While
                Statement::While(
                    expr,
                    scope
                ) => {
                    loop {
                        let condition = self.eval_expr(expr)?;

                        match condition {
                            Value::Bool(true) => {}

                            Value::Bool(false) => break,

                            _ => {
                                return Err(
                                    InterpreterError::RuntimeError(
                                        String::from(
                                            "InterpreterError: while condition must be bool."
                                        )
                                    )
                                );
                            }
                        }

                        match scope.as_ref() {
                            Statement::Scope(statements) => {
                                self.env.scopes.push(RuntimeScope::new());
                                self.depth += 1;

                                self.interpret_scope(statements)?;

                                self.env.scopes.pop();
                                self.depth -= 1;
                            }

                            _ => {
                                return Err(
                                    InterpreterError::RuntimeError(
                                        String::from(
                                            "InterpreterError: 'while' body must be a scope."
                                        )
                                    )
                                );
                            }
                        }
                    }
                }

                _ => return Err(InterpreterError::RuntimeError(
                    String::from("InterpreterError: invalid statement.")
                ))

            }

        }

        Ok(())

    }



    // Printout
    pub fn print_env(&self) -> String {
        self.env.print()
    }
}