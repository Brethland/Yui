use crate::ytype::*;
use crate::ast::*;
use std::collections::{HashMap, VecDeque};

fn parse_type(ast: &Ast, t_scope: TypeScope) -> TypeScope {
    TypeScope{alive_type: vec![], gamma: HashMap::new()}
} 

fn parse_expr_ordinary(ast: &Ast) -> Ast {
    Ast::LitInteger(114)
}

pub fn scope_parser(asts: &Vec<Ast>, scopes: &mut HashMap<String, Scope>, package_name: String) {
    let mut stack = VecDeque::new();
    stack.push_front("");
    let mut now_scope = "";

    for (i, ast) in asts.into_iter().enumerate() {
        match ast {
            Ast::Expr(vec) => {
                if vec[0] == Ast::Keyword(Keyword::Scope) {
                    if let Ast::LitString(scope) = &vec[1] {
                        now_scope = scope.as_str();
                        match scopes.get(&now_scope.to_string()) {
                            Some(_) => panic!("Multiple definition of scope: {}", now_scope),
                            None    => {
                                scopes.insert(now_scope.to_string(), Scope{
                                    name: now_scope.to_string(),
                                    parent: Some(stack[0].to_string()),
                                    children: vec![],
                                    t_scope: scopes[&stack[0].to_string()].t_scope.clone(), // To do sort type declaration
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
                } else if package_name == "main".to_string() || vec[0] == Ast::Keyword(Keyword::Let) { // If not main, only need let statements
                    if now_scope != "" {
                        let stat = scopes.entry(now_scope.to_string()).or_insert(Scope::new()); 
                        stat.context.insert(now_scope.to_string() + "." + i.to_string().as_str(), parse_expr_ordinary(ast));
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