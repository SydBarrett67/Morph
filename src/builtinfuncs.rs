use crate::{interpreter::Function, scope::{self, RuntimeScope}};
/*
    This file contains all the builtin function (not in stdlib)
*/

pub fn print(msg: String) {
    println!("{}", msg);
}






// This function loads all builtins in a scope
pub fn __loadbuiltins(scope: RuntimeScope) -> RuntimeScope {
    
    scope
}