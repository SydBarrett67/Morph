use crate::typechecker::Type;

pub struct SymbolId(pub usize);

#[derive(Debug, Clone)]
pub enum Symbol {
    Variable {
        name: String,
        ty: Type
    }
}