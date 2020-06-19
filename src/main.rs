extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod parser;
mod ytype;
mod type_checker;
mod pack_resol;

use std::fs;
use parser::parse;
use pack_resol::*;

fn main() {
    let unparsed_file = fs::read_to_string("./test/yui.yui").expect("cannot read file");

    let asts = parse(unparsed_file);

    let imported_asts = package_import(&asts, "main".to_string(), 0, true);

    for ast in imported_asts {
        scope_validation(&ast.content);
        println!("{}", ast.name);
    }
}
