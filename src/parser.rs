use pest::Parser;
use pest::iterators::*;
use crate::ast::*;

#[derive(Parser)]
#[grammar = "yui-file.pest"]
pub struct YuiFile;

fn parse_keywords(pair: pest::iterators::Pair<Rule>) -> Ast {
    let keyword = match pair.as_str() {
        "let"     => Keyword::Let,
        "match"   => Keyword::Match,
        "default" => Keyword::MDefault,
        "import"  => Keyword::Import,
        "scope"   => Keyword::Scope,
        "end"     => Keyword::End,
        "open"    => Keyword::Open,
        "generic" => Keyword::Generic,
        _         => panic!("unexpected keywords: {}", pair.as_str())
    };

    Ast::Keyword(keyword)
}

fn parse_operator(pair: pest::iterators::Pair<Rule>) -> Ast {
    let op = match pair.as_str() {
        "+"  => Op::Add,
        "-"  => Op::Minus,
        "*"  => Op::Mult,
        "/"  => Op::Div,
        "^"  => Op::Pow,
        "<"  => Op::Lt,
        "<=" => Op::Le,
        "="  => Op::Equ,
        "!=" => Op::Neq,
        ">=" => Op::Ge,
        ">"  => Op::Gt,
        "&"  => Op::And,
        "|"  => Op::Or,
        "!"  => Op::Not,
        _    => panic!("unexpected operator: {}", pair.as_str())
    };

    Ast::Operator(op)
}

fn parse_t_expr(pair: pest::iterators::Pair<Rule>) -> Vec<String> {
    let pairs = pair.into_inner();
    let mut type_vec = vec![];

    for pair in pairs {
        if pair.as_rule() == Rule::symbol {
            type_vec.push(pair.as_str().to_string());
        }
    }

    type_vec
}

fn parse_var(pair: pest::iterators::Pair<Rule>) -> Ast {
    let pairs = pair.into_inner();

    let mut type_annotation = TExpr{type_name: vec![], type_generic: vec![]};
    let mut flag = false;
    let mut name = "".to_string();

    for pair in pairs {
        if pair.as_rule() == Rule::symbol {
            name = pair.as_str().to_string();
        } else if pair.as_rule() == Rule::t_expr {
            if flag == false {
                type_annotation.type_name = parse_t_expr(pair);
                flag = true;
            } else {
                type_annotation.type_generic = parse_t_expr(pair);
            }
        }
    }

    Ast::Var(Var{name: name, type_annotation: type_annotation})
}

fn parse_name(pair: pest::iterators::Pair<Rule>) -> Ast {
    let pair = pair.into_inner().collect::<Vec<Pair<Rule>>>();
    match pair[0].as_rule() {
        Rule::keywords => parse_keywords(pair[0].clone()),
        Rule::operator => parse_operator(pair[0].clone()),
        Rule::var      => parse_var(pair[0].clone()),
        _              => panic!("unexpected handle: {:?}", pair)
    }
}

fn parse_lit(pair: pest::iterators::Pair<Rule>) -> Ast {
    let pair = pair.into_inner().collect::<Vec<Pair<Rule>>>();
    match pair[0].as_rule() {
        Rule::integer => Ast::LitInteger(pair[0].as_str().parse().unwrap()),
        Rule::string  => Ast::LitString(pair[0].as_str().to_string()),
        _             => panic!("unexpected literal: {:?}", pair)
    }
}

fn parse_expr(pair: pest::iterators::Pair<Rule>) -> Ast {
    let pairs = pair.into_inner();

    let mut expr_vec: Vec<Ast> = vec![];

    for pair in pairs {
        match pair.as_rule() {
            Rule::name => expr_vec.push(parse_name(pair)),
            Rule::lit  => expr_vec.push(parse_lit(pair)),
            Rule::expr => expr_vec.push(parse_expr(pair)),
            _          => {}
        }
    }

    Ast::Expr(expr_vec)
}

fn parse_type(pair: pest::iterators::Pair<Rule>) -> Ast {
    let pairs = pair.into_inner();

    let mut type_vec: Vec<Ast> = vec![];

    for pair in pairs {
        if pair.as_rule() == Rule::symbol {
            type_vec.push(Ast::Var(Var{name: pair.as_str().to_string(), type_annotation: TExpr{type_name: vec!["Any".to_string()], type_generic: vec![]}}));
        }
        if pair.as_rule() == Rule::expr {
            type_vec.push(parse_expr(pair));
        }
    }

    Ast::Type(type_vec)
}

pub fn parse(s: String) -> Vec<Ast>{
    let pairs = YuiFile::parse(Rule::file, &s).expect("boom!").next().unwrap().into_inner().filter(|token| token.as_rule() != Rule::skipped).collect::<Vec<Pair<Rule>>>();

    let mut result = vec![];

    for pair in pairs {
        match pair.as_rule() {
            Rule::expr   => result.push(parse_expr(pair)),
            Rule::type_c => result.push(parse_type(pair)),
            _            => {}
        }
    }

    result
}