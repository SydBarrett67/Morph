use crate::interpreter::{ Value };
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RuntimeScope {
    pub values: HashMap<String, Value>,
    pub scopes: Vec<RuntimeScope>,
}

impl RuntimeScope {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            scopes: Vec::new()
        }
    }

    // Get scope overload
    pub fn getScope(&self, depth: usize) -> Option<&RuntimeScope> {
        self.scopes.get(depth)
    }
    // Get variable overload
    pub fn getVar(&self, name: String) -> Option<&Value> {
        self.values.get(&name)
    }
    pub fn pushVar(&mut self, name: String, value: Value, depth: usize) {
        self.scopes.get_mut(depth).unwrap().values.insert(name, value);
    }

    // Printout 
    pub fn print(&self) -> String {
        let mut result = String::new();

        for scope in &self.scopes {
            for (key, value) in &scope.values {
                result.push_str(&format!("{}: {:?}\n", key.as_str(), value));
            }
        }

        result
    }

}