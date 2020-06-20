extern crate pest;
#[macro_use]
extern crate pest_derive;
mod ast;
mod parser;
mod ytype;
mod scope_resol;
mod pack_resol;

use std::fs;
use std::env;
use std::collections::HashMap;

use parser::parse;
use pack_resol::*;
use ytype::*;
use scope_resol::scope_parser;

fn main() -> std::io::Result<()> {
    let mut path = env::current_dir()?;

    path.push("test");
    path.push("yui.yui");
    let pre_path = String::from(path.as_path().to_str().unwrap());
    path.pop();

    let unparsed_file = fs::read_to_string(pre_path)?;
    let asts = match parse(unparsed_file) {
        Ok(asts) => asts,
        _        => panic!("Cannot parse file correctly")
    };

    let imported_asts = package_import(&asts, "main".to_string(), 0, true, path.as_path().to_str().unwrap());
    let mut scopes = HashMap::new();
    scopes.insert("".to_string(), Scope {
        name: "".to_string(),
        parent: Option::None,
        children: vec![],
        t_scope: TypeScope {
            alive_type: vec![TypeContext{typ: Type::BaseType(BaseType::Any), constructors: vec![]},
                             TypeContext{typ: Type::BaseType(BaseType::Int), constructors: vec![]},
                             TypeContext{typ: Type::BaseType(BaseType::Str), constructors: vec![]},
                             TypeContext{typ: Type::BaseType(BaseType::Unit), constructors: vec![]}],
            gamma: HashMap::new(),
        },
        context: HashMap::new(),
    });

    for ast in imported_asts {
        scope_validation(&ast.content);
        scope_parser(&ast.content, &mut scopes, ast.name);
    }

    Ok(())
}
