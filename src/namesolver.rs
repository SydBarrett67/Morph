
use crate::scope::Scope;
use crate::ast::{ Expression, Operator, Statement };
use crate::symbol::{ Symbol, SymbolId };
use crate::typechecker::Type;

#[derive(Debug, Clone)]
pub struct NameError{ msg: String }

pub struct NameSolver {
    ast: Statement,
    glob_scope: Vec<Scope>,
    curr_scope: usize
}

impl NameSolver {
    pub fn new(ast: Statement) -> Self {
        Self {
            ast: ast,
            glob_scope: vec![
                Scope::new(None)
            ],
            curr_scope: 0,
        }
    }

    pub fn solve_statement(&mut self, statement: Statement) -> Result<(), NameError> {
        match statement {

            // Variable declaration => new entry in scope hashmap
            Statement::Let(
                Expression::Identifier(name),
                Expression::Identifier(ty),
                ..
            ) => {
                let inferred_type = match ty.as_str() {

                    "int" => Ok(Type::INT),
                    "string" => Ok(Type::STRING),
                    "bool" => Ok(Type::BOOL),

                    _ => {
                        Err(
                            NameError { msg: String::from("(TODO) TypeError") }
                        )
                    }
                };

                self.glob_scope
                    .get_mut(self.curr_scope)
                    .unwrap()
                    .symbols
                    .insert(
                        name.clone(),
                        Symbol::Variable {
                            name,
                            ty: inferred_type?,
                        },
                    );

                Ok(())
            }

            // New scope
            Statement::Scope(
                stmts
            ) => {
                self.glob_scope.push(
                    Scope::new(Some(self.curr_scope))
                );
                self.curr_scope += 1;


                // Solve every statement inside new scope
                for stmt in stmts {
                    self.solve_statement(stmt)?
                }

                self.curr_scope -= 1 ;

                Ok(())
            }

            _ => Ok(())
        }
    }

    pub fn build_scopes(&mut self) -> Result<Vec<Scope>, NameError> {

        let nodes = match &self.ast {
            Statement::Root(nodes) => nodes.clone(),

            _ => {
                return Err(NameError {
                    msg: String::from("NameError: could not resolve."),
                });
            }
        };

        for statement in nodes {
            self.solve_statement(statement)?;
        }

        Ok(self.glob_scope.clone())
    }
}
