use crate::ytype::*;
use crate::ast::*;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct Expr {
    pub name: String,
    pub asts: Ast,
}

fn vec_find(vec: &Vec<Type>, name: &String) -> bool {
    for typ in vec {
        if let Type::TypeArgs(arg_name) = typ {
            if name == arg_name {
                return true;
            }
        }
    }
    false
}

fn parse_type(ast: &Ast, t_scope: TypeScope) -> TypeScope {
    let mut types_vec: Vec<Constructor> = vec![];
    let mut args_vec: Vec<Type> = vec![];
    let mut var_name = "".to_string();

    if let Ast::Type(vec) = ast {
        for (i, ast) in vec.into_iter().enumerate() {
            if i == 0 {
                if let Ast::Var(var) = ast {
                    var_name = var.name.clone();
                }
            } else if i == 1 {
                if let Ast::Expr(vec_args) = ast {
                    for arg in vec_args {
                        if let Ast::Var(var) = &arg {
                            args_vec.push(Type::TypeArgs(var.name.clone()));
                        }
                    }
                }
            } else {
                if let Ast::Expr(vec_cons) = ast {
                    let mut cons_name = "".to_string();
                    let mut tmp: Vec<Type> = vec![];
                    for (i, con) in vec_cons.into_iter().enumerate() {
                        if i == 0 {
                            if let Ast::Var(var) = con {
                                cons_name = var.name.clone();
                            }
                        } else {
                            if let Ast::Var(var) = con {
                                if var.name != var_name && !vec_find(&args_vec, &var.name) {
                                    panic!("Type arguments cannot find: {}", var.name);
                                } else {
                                    tmp.push(Type::TypeArgs(var.name.clone()));
                                }
                            }
                        }
                    }
                    types_vec.push(Constructor{name: cons_name, args: tmp});
                }
            }
        }
    }

    let new_type = TypeContext{typ: Type::CustomType(var_name), args: args_vec, constructors: types_vec};
    let mut new_types = t_scope.alive_type.clone();
    new_types.push(new_type);

    TypeScope{alive_type: new_types, gamma: HashMap::new()}
}

fn parse_expr_ordinary(ast: &Ast) -> Expr {
    let mut var_name = "".to_string();

    if let Ast::Expr(vec) = ast {
        if let Ast::Var(var) = &vec[1] {
            if vec[0] == Ast::Keyword(Keyword::Generic) {
                var_name = var.name.clone() + "@Generic";
            } else {
                var_name = var.name.clone();
            }
        }
    }

    Expr{name: var_name, asts: ast.clone()}
}

pub fn scope_parser(asts: &Vec<Ast>, scopes: &mut HashMap<String, Scope>) {
    let mut stack = VecDeque::new();
    stack.push_front("");
    let mut now_scope = "";

    for ast in asts {
        match ast {
            Ast::Expr(vec) => {
                if vec[0] == Ast::Keyword(Keyword::Scope) {
                    if let Ast::LitString(scope) = &vec[1] {
                        now_scope = scope.as_str();
                        match scopes.get(&now_scope.to_string()) {
                            Some(_) => panic!("Multiple definition of scope: {}", now_scope),
                            None    => {
                                scopes.insert(now_scope.to_string(), Scope {
                                    name: now_scope.to_string(),
                                    parent: Some(stack[0].to_string()),
                                    children: vec![],
                                    t_scope: TypeScope{alive_type: vec![], gamma: HashMap::new()},
                                    context: HashMap::new(),
                                });
                                stack.push_front(now_scope);
                            }
                        }
                    }
                } else if vec[0] == Ast::Keyword(Keyword::End) {
                    now_scope = stack[1];
                    let stat = scopes.entry(now_scope.to_string()).or_insert(Scope::new());
                    stat.children.push(stack[0].to_string());
                    stack.pop_front();
                } else if vec[0] == Ast::Keyword(Keyword::Let) || vec[0] == Ast::Keyword(Keyword::Generic) { // If not main, only need let statements
                    if now_scope != "" {
                        let stat = scopes.entry(now_scope.to_string()).or_insert(Scope::new()); 
                        let parsed_expr = parse_expr_ordinary(ast);
                        stat.context.insert(parsed_expr.name, parsed_expr.asts);
                    }
                }
            },
            Ast::Type(_) => {
                if now_scope != "" {
                    match scopes.get(&now_scope.to_string()) {
                        Some(scope) => {
                            let parsed_types = parse_type(ast, scope.t_scope.clone());
                            let stat = scopes.entry(now_scope.to_string()).or_insert(Scope::new());
                            stat.t_scope = parsed_types;
                        },
                        None        => panic!("Unresolved Scope: {}", now_scope)
                    }
                }
            },
            _              => panic!("Unresolved Ast: {:?}", ast)
        }
    }
}
