use std::collections::HashMap;
use crate::ast::{Statement, Expression, Type};

#[derive(Debug, Clone)]
pub struct NameError {
    pub msg: String,
}

pub struct SemanticAnalyzer {
    ast: Statement,
    scopes: Vec<HashMap<String, Type>>,
}

impl SemanticAnalyzer {
    pub fn new(ast: Statement) -> Self {
        Self {
            ast,
            scopes: vec![HashMap::new()],
        }
    }

    pub fn analyze(mut self) -> Result<Statement, NameError> {
        let ast = std::mem::replace(&mut self.ast, Statement::Scope(vec![]));
        self.analyze_statement(ast)
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare_variable(&mut self, name: String, ty: Type) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(name, ty);
        }
    }

    fn lookup(&self, name: &str) -> Option<&Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty);
            }
        }
        None
    }

    fn analyze_statement(&mut self, stmt: Statement) -> Result<Statement, NameError> {
        match stmt {
            Statement::Scope(stmts) => {
                self.enter_scope();
                let mut annotated_stmts = Vec::new();
                for s in stmts {
                    annotated_stmts.push(self.analyze_statement(s)?);
                }
                self.exit_scope();
                Ok(Statement::Scope(annotated_stmts))
            }

            Statement::Let(name, declared_ty, expr) => {
                let annotated_expr = self.analyze_expression(expr)?;

                let ty = declared_ty.clone();
                self.declare_variable(name.clone(), ty.clone());

                Ok(Statement::Let(name, declared_ty, annotated_expr))
            }

            Statement::Assign(target, expr) => {
                let annotated_expr = self.analyze_expression(expr)?;
                Ok(Statement::Assign(target, annotated_expr))
            }

            Statement::If(expr, scope) => {
                let annotated_expr = self.analyze_expression(expr)?;
                Ok(Statement::If(annotated_expr, scope))
            }

            Statement::While(expr, scope) => {
                let annotated_expr = self.analyze_expression(expr)?;
                Ok(Statement::While(annotated_expr, scope))
            }

            Statement::FuncDecl(
                name,
                declared_type,
                args,
                scope,
            ) => {

                let annotated_scope = self.analyze_statement(*scope)?;

                Ok(Statement::FuncDecl(name, declared_type, args, Box::new(annotated_scope)))
            }

            _ => Err(NameError { msg: String::from("Boh fra se sei arrivato qui è un po' crazy") })
        }
    }

    fn analyze_expression(&mut self, expr: Expression) -> Result<Expression, NameError> {
        match expr {
            Expression::Identifier { name, .. } => {
                if let Some(ty) = self.lookup(&name) {
                    Ok(Expression::Identifier {
                        name,
                        ty: Some(ty.clone()),
                    })
                } else {
                    Err(NameError {
                        msg: format!("Variabile non trovata: {}", name),
                    })
                }
            }

            Expression::Literal(lit) => Ok(Expression::Literal(lit)),
            Expression::Parameter(name, ty) => Ok(Expression::Parameter(name, ty)),

            Expression::BinaryExpression(op, left, right, _) => {
                let annotated_left = self.analyze_expression(*left)?;
                let annotated_right = self.analyze_expression(*right)?;

                Ok(Expression::BinaryExpression(
                    op,
                    Box::new(annotated_left),
                    Box::new(annotated_right),
                    Some(Type::INT),
                ))
            }
        }
    }
}