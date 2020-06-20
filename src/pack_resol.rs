use crate::ast::*;
use crate::parser::parse;

use std::fs;
use std::cmp::Ordering;
use std::path::PathBuf;
use std::collections::VecDeque;

#[derive(PartialEq, Eq)]
pub struct Packages {
    pub name: String,
    pub content: Vec<Ast>,
}

impl Ord for Packages {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Packages {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read_file(s: String, pre_path: &str) -> Vec<Packages> {
    let mut path = s.clone();

    path = path.trim_end_matches("\"").to_string();
    path = path.trim_start_matches("\"").to_string();
    let p = path.clone();
    path.push_str(".yui");
    
    let mut package_path = PathBuf::from(pre_path.to_string());
    package_path.push(path);
    let unparsed_file = fs::read_to_string(package_path.as_path().to_str().unwrap()).expect("cannot read file: {}");
    package_path.pop();
    
    let asts = match parse(unparsed_file) {
        Ok(asts) => asts,
        _        => panic!("Cannot parse file correctly")
    };

    let pos = p.as_str().rfind("/");
    match pos {
        Some(c) => package_path.push(&p[..c]),
        _       => {},
    }

    package_import(&asts, p, 0, true, package_path.to_str().unwrap())
}

pub fn package_import(asts: &Vec<Ast>, now_scope: String, depth: u64, total: bool, pre_path: &str) -> Vec<Packages> {
    let mut result = vec![];

    for (i, ast) in asts.iter().enumerate() {
        match ast {
            Ast::Expr(vec) => {
                if vec[0] == Ast::Keyword(Keyword::Import) {
                    if vec.len() != 2 {
                        panic!("Mismatch of import argument number: {}", vec.len());
                    }
                    if depth != 0 {
                        panic!("Cannot import packages in scope");
                    }
                    if let Ast::LitString(path) = &vec[1] {
                        if *path == "main".to_string() {
                            panic!("Package cannot be named main");
                        }
                        result.append(&mut read_file(path.clone(), pre_path));
                    }
                } else if vec[0] == Ast::Keyword(Keyword::Scope) {
                    if vec.len() != 2 {
                        panic!("Mismatch of scope argument number: {}", vec.len());
                    }
                    if let Ast::LitString(scope) = &vec[1] {
                        package_import(&asts[i+1..].to_vec(), (*scope).clone(), depth + 1, false, pre_path);
                    }
                }
            },
            _ => {},
        };
    }

    if total == true { // For Optimization
        let cleaned_asts = asts.clone().into_iter().filter(|x| match x {
            Ast::Expr(vec) => {
                if vec[0] == Ast::Keyword(Keyword::Import) {
                    false
                }
                else {
                    true
                }
            },
            _ => true,
        }).collect();
        
        result.push(Packages{name: now_scope, content: cleaned_asts});
        result.sort_by(|a, b| a.cmp(b));
        result.dedup();
    }
    result
}

pub fn scope_validation(asts: &Vec<Ast>) {
    let mut stack = VecDeque::new();

    for ast in asts {
        match ast {
            Ast::Expr(vec) => {
                if vec[0] == Ast::Keyword(Keyword::Scope) {
                    if let Ast::LitString(scope) = &vec[1] {
                        stack.push_front(scope);
                    }
                } else if vec[0] == Ast::Keyword(Keyword::End) {
                    if vec.len() != 2 {
                        panic!("Mismatch of end argument number: {}", vec.len());
                    }
                    if let Ast::LitString(scope) = &vec[1] {
                        if stack[0] != scope {
                            panic!("Scope {} cannot close before subscope {}", scope, stack[0]);
                        }
                        stack.pop_front();
                    }
                }
            },
            _ => {},
        }
    }

    if stack.len() > 0 {
        panic!("Some scope not enclose");
    }
}