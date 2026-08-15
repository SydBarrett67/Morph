use crate::types::Type;

use std::{collections::HashMap};

pub struct Scope {
    symbols: HashMap<String, Type>,
    parent: Box<Option<Scope>>
}

impl Scope {
    pub fn new(parent: Scope) -> Self {
        Self {
            symbols: HashMap::new(),
            parent: Box::new(Some(parent))
        }
    }
}