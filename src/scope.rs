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

    // Get variable
    pub fn get_var(&self, name: String, depth: usize) -> Option<&Value> {
        println!("depth: {}", depth);
        println!("scopes: {}", self.scopes.len());
        for i in (0..=depth).rev() {
            if let Some(value) = self.scopes.get(i).unwrap().values.get(&name) {
                return Some(value);
            }
        }
        None
    }
    // Add var
    pub fn push_var(&mut self, name: String, value: Value, depth: usize) {
        self.scopes.get_mut(depth).unwrap().values.insert(name, value);
    }
    // Set var
    pub fn set_var(&mut self, name: String, value: Value, depth: usize) -> bool {
        for i in (0..=depth).rev() {
            if let Some(scope) = self.scopes.get_mut(i) {
                if scope.values.contains_key(&name) {
                    scope.values.insert(name, value);
                    return true;
                }
            }
        }

        false
    }

    // Printout 
    pub fn print(&self) -> String {

        let mut result = String::new();
        for (depth, scope) in self.scopes.iter().enumerate() {

            result.push_str(
                &format!("Scope {}:\n", depth)
            );

            for (key, value) in &scope.values {
                result.push_str(
                    &format!("    {}: {:?}\n", key.as_str(), value)
                );
            }
        }
        result
    }

}