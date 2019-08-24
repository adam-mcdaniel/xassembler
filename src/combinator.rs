extern crate honeycomb;
use honeycomb::{
    atoms::{eof, opt, rec, seq_no_ws, space, sym},
    language::{array, identifier, number, string},
    Parser,
};

use crate::*;

pub fn string_literal() -> Parser<Literal> {
    ((space() >> string() << space()) - Literal::String) % "a string literal"
}

pub fn number_literal() -> Parser<Literal> {
    ((space() >> number() << space()) - Literal::Number) % "a number literal"
}

pub fn literal() -> Parser<Value> {
    (string_literal() | number_literal()) - Value::Literal
}

pub fn ident() -> Parser<Identifier> {
    ((space() >> identifier() << space()) - Identifier) % "an identifier"
}

pub fn dot_ident(values: Parser<Value>) -> Parser<(Box<Value>, Vec<Identifier>)> {
    ((values & ((sym('.') >> rec(ident)) * (1..)))
        - |v: (Value, Vec<Identifier>)| (Box::new(v.0), v.1))
        % "a dotted name"
}

pub fn index_name(values: Parser<Value>) -> Parser<(Box<Value>, Vec<Value>)> {
    ((values & ((seq_no_ws("[") >> rec(value) << seq_no_ws("]")) * (1..)))
        - |(head, indices)| (Box::new(head), indices))
        % "a value followed by one or more indices"
}

pub fn name() -> Parser<Name> {
    ((dot_ident(group() | literal() | (ident() - Name::Name - Value::Name))
        - |d| Name::DotName(d.0, d.1))
        | (index_name(group() | literal() | (ident() - Name::Name - Value::Name))
            - |d| Name::IndexName(d.0, d.1))
        | ident() - Name::Name)
        % "a dotted name, an indexed value, or an identifier"
}

pub fn fncall() -> Parser<Value> {
    ((((name() - Value::Name) | rec(group)) & array("(", rec(value), ")"))
        - |call_data: (Value, Vec<Value>)| {
            Value::FnCall(FnCall(Box::new(call_data.0), call_data.1))
        })
        % "a value followed by comma arguments"
}

pub fn group() -> Parser<Value> {
    seq_no_ws("(") >> rec(value) << seq_no_ws(")")
}

pub fn flat_value() -> Parser<Value> {
    literal()
}

pub fn recursive_value() -> Parser<Value> {
    (function() - Value::Function) | rec(fncall) | (name() - Value::Name) | rec(group)
}

pub fn value() -> Parser<Value> {
    rec(recursive_value) | rec(flat_value)
}

pub fn function() -> Parser<Function> {
    (seq_no_ws("fn") >> (array("(", ident(), ")") & suite()))
        - |(params, suite)| Function(params, suite)
}

pub fn function_def() -> Parser<FunctionDef> {
    let body = array("(", ident(), ")") & rec(suite);
    ((seq_no_ws("fn") >> name() & body)
        - |(n, (params, suite))| FunctionDef(n, Function(params, suite)))
        % "a valid function definition"
}

pub fn struct_def() -> Parser<StructDef> {
    ((seq_no_ws("class") >> ((name() << seq_no_ws("{")) & (function_def() * (..)))
        << seq_no_ws("}"))
        - |(name, function_defs)| StructDef(name, function_defs))
        % "a valid class definition"
}

pub fn assignment() -> Parser<Expr> {
    ((name() & (seq_no_ws("=") >> value())) - |(n, v)| Expr::Assignment(n, v))
        % "a valid assignment"
}

pub fn while_loop() -> Parser<Expr> {
    (((seq_no_ws("while") >> value()) & rec(suite)) - |(n, v)| Expr::WhileLoop(n, v))
        % "a valid while loop"
}

pub fn if_then_else() -> Parser<Expr> {
    ((((seq_no_ws("if") >> value()) & rec(suite)) & opt(seq_no_ws("else") >> rec(suite)))
        - |((condition, then_body), else_body_opt)| {
            let else_body = match else_body_opt {
                Some(body) => body,
                None => Suite(Vec::new()),
            };

            Expr::IfThenElse(condition, then_body, else_body)
        })
        % "a valid if else statement"
}

pub fn expr() -> Parser<Expr> {
    (assignment() << opt(seq_no_ws(";"))) % "a valid assignment"
        | struct_def() - Expr::StructDef
        | function_def() - Expr::FunctionDef
        | while_loop()
        | if_then_else()
        | (value() - Expr::Value << opt(seq_no_ws(";"))) % "a value"
}

pub fn suite() -> Parser<Suite> {
    ((seq_no_ws("{") >> (expr() * (..)) << seq_no_ws("}")) - Suite)
        % "a curly brace enclosed list of expressions"
}

pub fn program() -> Parser<Suite> {
    ((expr() * (..)) - Suite) << eof()
}
