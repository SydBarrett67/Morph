use crate::typechecker::{Type, Value};

pub struct SymbolId(pub usize);

#[derive(Debug, Clone)]
pub enum Symbol {
    Variable {
        name: String,
        ty: Type,
    }
}