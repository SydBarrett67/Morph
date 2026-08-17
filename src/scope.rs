use crate::{symbol::{ Symbol, SymbolId }, typechecker::Value};

use std::{collections::HashMap};

#[derive(Debug, Clone)]
pub struct ASTScope {
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<usize>
}
impl ASTScope {
    pub fn new(parent: Option<usize>) -> Self {
        Self {
            symbols: HashMap::new(),
            parent: parent,
        }
    }

    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        if self.symbols.contains_key(name) {
            Some(self.symbols.get(name)?)
        }
        else {
            None
        }
    }
}