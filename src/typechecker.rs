#[derive(Debug, Clone)]
pub enum Type {
    INT,
    BOOL,
    STRING
}
pub enum Value {
    Int(i32),
    Bool(bool),
    String(String)
}

pub struct TypeChecker {

}

impl TypeChecker {
    pub fn new() -> Self {
        Self {

        }
    }


}