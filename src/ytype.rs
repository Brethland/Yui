use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::ast::Ast;

#[derive(Debug)]
pub enum BaseType {
    Int,
    Str,
    Any,
    Unit,
}

#[derive(Debug)]
pub enum Type {
    BaseType(BaseType),
    CustomType(String),
    ArrowType {
        left:  Box<Type>,
        right: Box<Type>,
    },
}

#[derive(Debug)]
pub struct Constructor {
    name: String,
    args: Vec<Type>,
}

#[derive(Debug)]
pub struct TypeContext {
    typ: Type,
    constructors: Vec<Constructor>,
}

#[derive(Debug)]
pub struct TypeScope {
    alive_type: Vec<TypeContext>,
    gamma: HashMap<String, Type>,
}

pub struct Scope {
    t_scope: TypeScope,
    context: HashMap<String, Ast>,
}

lazy_static! {
    static ref GL_SCOPE: Scope = Scope {
        t_scope: TypeScope {
            alive_type: vec![TypeContext{typ: Type::BaseType(BaseType::Any), constructors: vec![]},
                             TypeContext{typ: Type::BaseType(BaseType::Int), constructors: vec![]},
                             TypeContext{typ: Type::BaseType(BaseType::Str), constructors: vec![]},
                             TypeContext{typ: Type::BaseType(BaseType::Unit), constructors: vec![]}],
            gamma: HashMap::new(),
        },
        context: HashMap::new(),
    };
}