use crate::interpreter::{ Value, Function };
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RuntimeScope {
    pub values: HashMap<String, Value>,
    pub functions: HashMap<String, Function>,
    pub scopes: Vec<RuntimeScope>,
}

impl RuntimeScope {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            functions: HashMap::new(),
            scopes: Vec::new()
        }
    }

    // Get variable
    pub fn get_var(&self, name: String, depth: usize) -> Option<&Value> {
        for i in (0..=depth).rev() {
            if let Some(value) = self.scopes.get(i).unwrap().values.get(&name) {
                return Some(value);
            }
        }
        None
    }
    // Get function
    pub fn get_func(&self, name: String, depth: usize) -> Option<&Function> {
        for i in (0..=depth).rev() {
            if let Some(value) = self.scopes.get(i).unwrap().functions.get(&name) {
                return Some(value);
            }
        }
        None
    }
    // Add var
    pub fn push_var(&mut self, name: String, value: Value, depth: usize) {
        self.scopes.get_mut(depth).unwrap().values.insert(name, value);
    }
    // Add func
    pub fn push_func(&mut self, func: Function, depth: usize) {
        self.scopes.get_mut(depth).unwrap().functions.insert(func.name.clone(), func);
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