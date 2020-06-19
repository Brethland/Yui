#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct TExpr {
    pub type_name: Vec<String>,
    pub type_generic: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Var {
    pub name: String,
    pub type_annotation: TExpr,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Keyword {
    Let,
    Match,
    MDefault,
    Import,
    Scope,
    End,
    Open,
    Generic,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Op {
    Add,
    Minus,
    Mult,
    Div,
    Pow,
    Lt,
    Le,
    Equ,
    Neq,
    Ge,
    Gt,
    And,
    Or,
    Not,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Ast {
    Expr(Vec<Ast>),
    LitInteger(i64),
    LitString(String),
    Keyword(Keyword),
    Operator(Op),
    Var(Var),
    Type(Vec<Ast>),
}