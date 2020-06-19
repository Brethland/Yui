use crate::ytype::*;
use crate::ast::Ast;

pub enum TypedAst {
    Expr(Vec<TypedAst>, Type),
    Name(Ast, Type),
}

pub fn static_checking(asts: Vec<Ast>) -> Vec<TypedAst>{
    vec![]
}