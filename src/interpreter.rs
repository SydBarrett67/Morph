use crate::ast::{ Expression, Statement, Operator, Literal };

use crate::scope::RuntimeScope;

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
                let scope: RuntimeScope = match self.env.getScope(self.depth) {
                    Some(scope) => scope.clone(),
                    None => {
                        return Err(
                            InterpreterError::RuntimeError(
                                String::from("InterpreterError: invalid scope.")
                            )
                        );
                    }
                };
                
                let value = match scope.getVar(name.to_string()) {
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
                    (Value::Int(lhs), Value::Int(rhs)) => {
                        match op {
                            Operator::Add => Ok(Value::Int(lhs + rhs)),
                            Operator::Sub => Ok(Value::Int(lhs - rhs)),
                            Operator::Mul => Ok(Value::Int(lhs * rhs)),
                            Operator::Div => Ok(Value::Int(lhs / rhs)),
                            
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

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), InterpreterError>{
        // Push global scope
        self.env.scopes.push(RuntimeScope::new());

        match &self.ast {
            // Enter scopes
            Statement::Scope(statements) => {

                // Cicle through every statement
                for statement in statements {

                    // Pattern matching for statement types
                    match statement {

                        // Enter nested scope
                        Statement::Scope(
                            stmnts: Vec<Statement>
                        ) => {
                            self.interpret();
                        }

                        // Variable declaration
                        Statement::Let(
                            name,
                            ty,
                            value
                        ) => {
                            let value = self.eval_expr(value)?;

                            self.env.pushVar(name.to_string(), value, self.depth);
                        }

                        // Variable assignment
                        Statement::Assign(
                            name,
                            expr
                        ) => {
                            let value = self.eval_expr(expr)?;

                            self.env.pushVar(name.to_string(), value, self.depth);
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



    // Printout
    pub fn printEnv(&self) -> String {
        self.env.print()
    }
}