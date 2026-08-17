use crate::ast::{ Expression, Statement };
use crate::scope::ASTScope;

struct TypeError {
    msg: String
}

#[derive(Debug, Clone)]
pub enum Type {
    INT,
    BOOL,
    STRING
}
#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
    String(String)
}

pub struct TypeChecker {
    ast: Statement,
    scopes: Vec<ASTScope>,
    depth: usize,
}

impl TypeChecker {
    pub fn new(ast: Statement, scopes: Vec<ASTScope>) -> Self {
        Self {
            ast: ast,
            scopes: scopes,
            depth: 0
        }
    }

    pub fn check_expr(&self, expr: &Expression) -> Result<Type, TypeError> {
        match expr {



            _ => Err(
                TypeError { msg: String::from("TypeErro: type mismatch.") }
            )
        }
    }

    pub fn validate_AST(&self) -> Result<(), TypeError> {

        match &self.ast {
            Statement::Root(stmts) => {

                for statement in stmts {
                    match statement {
                        _ => todo!()
                    }
                }

                Ok(())
            }

            _ => todo!()
        }

    }
}