use std::collections::HashMap;
use crate::ast::Ast;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum BaseType {
    Int,
    Str,
    Any,
    Unit,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Type {
    BaseType(BaseType),
    CustomType(String),
    TypeArgs(String),
    ArrowType {
        left:  Box<Type>,
        right: Box<Type>,
    },
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Constructor {
    pub name: String,
    pub args: Vec<Type>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct TypeContext {
    pub typ: Type,
    pub args: Vec<Type>,
    pub constructors: Vec<Constructor>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypeScope {
    pub alive_type: Vec<TypeContext>,
    pub gamma: HashMap<String, Type>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Scope {
    pub name: String,
    pub parent: Option<String>,
    pub children: Vec<String>,
    pub t_scope: TypeScope,
    pub context: HashMap<String, Ast>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope{name: "".to_string(), 
            parent: Option::None,
            children: vec![], 
            t_scope: TypeScope{alive_type: vec![], gamma: HashMap::new()}, 
            context: HashMap::new()}
    }
}