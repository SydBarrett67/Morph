use std::collections::HashMap;

use crate::ast::{ Expression, Node, Operator };

pub struct Interpreter {
    ast: Node,
    pub variables: HashMap<String, i32>,
}

impl Interpreter {
    pub fn new(ast: Node) -> Self {
        Self {
            ast: ast,
            variables: HashMap::new(),
        }
    }

    pub fn eval_expr(statement: Expression) {

    }

    pub fn interpret(&mut self) {
        match self.ast {
            Node::Root(ref nodes) => {
                // Main loop
                for statement in nodes {
                    match statement {
                        Node::Let(
                            Expression::Identifier(var_name),
                            Expression::Number(var_value),
                        ) => {
                            self.variables.insert(
                                var_name.clone(),
                                *var_value
                            );
                        }

                        _ => {}
                    }

                }

            }
            _ => {}
        }
    }
}