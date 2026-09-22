use crate::interpreter::{ Value };

#[derive(Debug, Clone)]
pub struct RuntimeScope {
    env: Vec<Value>,
}

impl RuntimeScope {
    pub fn new() -> Self {
        Self {
            env: Vec::new()
        }
    }
}