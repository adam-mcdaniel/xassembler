use crate::{compile::compile, rust::Rust, target::Compile, token::*};
use pest::*;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};

#[derive(Parser)]
#[grammar = "xasm.pest"]
pub struct Xasm;

fn identifier(pair: Pair<Rule>) -> Identifier {
    Identifier(pair.as_span().as_str().trim().to_string())
}

fn dot_identifier(mut pairs: Pairs<Rule>) -> Name {
    Name::DotName(
        Box::new({
            let accessing = pairs.next().unwrap();
            match accessing.as_rule() {
                Rule::identifier => Value::Name(Name::Name(identifier(accessing))),
                Rule::group => value(accessing.into_inner().into_iter().next().unwrap()),
                _ => unreachable!(),
            }
        }),
        pairs
            .into_iter()
            .map(identifier)
            .collect::<Vec<Identifier>>(),
    )
}

fn index_identifier(mut pairs: Pairs<Rule>) -> Name {
    Name::IndexName(
        Box::new({
            let accessing = pairs.next().unwrap();
            match accessing.as_rule() {
                Rule::identifier => Value::Name(Name::Name(identifier(accessing))),
                Rule::group => value(accessing.into_inner().into_iter().next().unwrap()),
                _ => unreachable!(),
            }
        }),
        pairs.into_iter().map(value).collect::<Vec<Value>>(),
    )
}

fn name(pair: Pair<Rule>) -> Name {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::dot_identifier => dot_identifier(pair.into_inner()),
        Rule::index_identifier => index_identifier(pair.into_inner()),
        Rule::identifier => Name::Name(identifier(pair)),
        Rule::name => name(pair),
        _ => unreachable!(),
    }
}

fn fncall(pair: Pair<Rule>) -> FnCall {
    let mut pairs = pair.clone().into_inner();
    let call_operand = pairs.next().unwrap();
    FnCall(
        Box::new(match call_operand.as_rule() {
            Rule::name => Value::Name(name(call_operand)),
            Rule::group => value(call_operand),
            Rule::fncall => Value::FnCall(fncall(call_operand)),
            _ => unreachable!(),
        }),
        pairs.into_iter().map(value).collect(),
    )
}

fn literal(pair: Pair<Rule>) -> Literal {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::string_literal => {
            let result = pair.as_str().to_string();
            Literal::String(String::from(&result[1..result.len() - 1]))
        }
        Rule::number_literal => Literal::Number(pair.as_str().to_string()),
        Rule::foreign_function_literal => Literal::ForeignFunction(pair.as_str()[1..].to_string()),
        _ => unreachable!(),
    }
}

fn value(pair: Pair<Rule>) -> Value {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::function => Value::Function(function(pair)),
        Rule::fncall => Value::FnCall(fncall(pair)),
        Rule::name => Value::Name(name(pair)),
        Rule::literal => Value::Literal(literal(pair)),
        Rule::group => value(pair),
        Rule::value => value(pair),
        _ => unreachable!(),
    }
}

fn assignment(mut pairs: Pairs<Rule>) -> Expr {
    Expr::Assignment(name(pairs.next().unwrap()), value(pairs.next().unwrap()))
}

fn args(pair: Pair<Rule>) -> Vec<Identifier> {
    pair.into_inner().into_iter().map(identifier).collect()
}

fn suite(pair: Pair<Rule>) -> Suite {
    Suite(pair.into_inner().into_iter().map(expr).collect())
}

fn function(pair: Pair<Rule>) -> Function {
    let mut pairs = pair.into_inner();
    Function(args(pairs.next().unwrap()), suite(pairs.next().unwrap()))
}

fn function_def(pair: Pair<Rule>) -> FunctionDef {
    let mut pairs = pair.into_inner();
    FunctionDef(
        name(pairs.next().unwrap()),
        Function(args(pairs.next().unwrap()), suite(pairs.next().unwrap())),
    )
}

fn class_def(mut pairs: Pairs<Rule>) -> Expr {
    Expr::StructDef(StructDef(
        name(pairs.next().unwrap()),
        pairs
            .into_iter()
            .map(function_def)
            .collect::<Vec<FunctionDef>>(),
    ))
}

fn while_loop(mut pairs: Pairs<Rule>) -> Expr {
    Expr::WhileLoop(value(pairs.next().unwrap()), suite(pairs.next().unwrap()))
}

fn for_loop(mut pairs: Pairs<Rule>) -> Expr {
    Expr::ForLoop {
        counter: identifier(pairs.next().unwrap()),
        element: identifier(pairs.next().unwrap()),
        list: value(pairs.next().unwrap()),
        body: suite(pairs.next().unwrap()),
    }
}

fn if_then_else(mut pairs: Pairs<Rule>) -> Expr {
    Expr::IfThenElse(
        value(pairs.next().unwrap()),
        suite(pairs.next().unwrap()),
        pairs
            .next()
            .and_then(|s| Some(suite(s)))
            .or_else(|| Some(Suite(vec![])))
            .unwrap(),
    )
}

fn expr(pair: Pair<Rule>) -> Expr {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::assignment => assignment(pair.into_inner()),
        Rule::class_def => class_def(pair.into_inner()),
        Rule::function_def => Expr::FunctionDef(function_def(pair)),
        Rule::while_loop => while_loop(pair.into_inner()),
        Rule::for_loop => for_loop(pair.into_inner()),
        Rule::if_then_else => if_then_else(pair.into_inner()),
        Rule::value => Expr::Value(value(pair)),
        Rule::expr => expr(pair),
        _ => unreachable!(),
    }

    // Ok(Expr(vec![]))
}

pub fn parse(input: &str) -> Result<Suite, String> {
    let pairs = match Xasm::parse(Rule::program, input) {
        Ok(parsed) => Ok(parsed),
        Err(e) => Err(format!("{}", e)),
    }?;

    let mut result = vec![];
    // println!("pairs {:#?}", pairs);
    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => {
                let token = expr(pair.clone());
                let compiled = Compile::<Rust>::compile(token.clone()).unwrap();
                // println!("{:#?}\n\t=>\n{}", token, compiled);
                result.push(token);
            }
            Rule::COMMENT => {}
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }

    Ok(Suite(result))
}
