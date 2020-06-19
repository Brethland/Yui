extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod parser;
mod ytype;
mod type_checker;
mod pack_resol;

use std::fs;
use std::env;
use parser::parse;
use pack_resol::*;

fn main() -> std::io::Result<()> {
    let mut path = env::current_dir()?;
    path.push("test");

    let pre_path = String::from(path.as_path().to_str().unwrap());

    let unparsed_file = fs::read_to_string(pre_path + "\\yui.yui")?; // To do : not on Windows?
    let asts = parse(unparsed_file);

    let imported_asts = package_import(&asts, "main".to_string(), 0, true, path.as_path().to_str().unwrap());

    for ast in imported_asts {
        scope_validation(&ast.content);
        println!("{}", ast.name);
    }

    Ok(())
}
