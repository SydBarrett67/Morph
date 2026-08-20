use std::collections::HashMap;

use crate::ast::{ Statement, Symbol };

#[derive(Debug, Clone)]
pub struct NameError{ msg: String }

pub struct NameSolver {
    ast: Statement,
    symbol_table: HashMap<String, Symbol>
}

impl NameSolver {
    pub fn new(ast: Statement) -> Self {
        Self {
            ast: ast,
            symbol_table: HashMap::new(),
        }
    }

    // Traverse crude AST and returns typed AST
    pub fn analyze(&mut self) -> Result<Statement, NameError> {
        match &self.ast {

            // Enter scope
            Statement::Scope(stmts) => {

                for statement in stmts {

                    match statement {

                        // Let
                        Statement::Let(
                            name,
                            ty,
                            ..
                        ) => {
                            
                        }

                        _ => todo!()
                    }

                }

                todo!()
            }

            _ => todo!()
        }
    }
}
