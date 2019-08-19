extern crate pom;
pub use pom::parser::*;
pub use pom::Parser;

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::str;
use core::str::FromStr;

use crate::*;

const ALPHA: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALPHANUMERIC: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn space() -> Parser<u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

pub fn number() -> Parser<u8, Value> {
    let integer = one_of(b"123456789") - one_of(b"0123456789").repeat(0..) | sym(b'0');
    let frac = sym(b'.') + one_of(b"0123456789").repeat(1..);
    let exp = one_of(b"eE") + one_of(b"+-").opt() + one_of(b"0123456789").repeat(1..);
    let number =
        space().discard() * sym(b'-').opt() + integer + frac.opt() + exp.opt() - space().discard();
    number
        .collect()
        .convert(str::from_utf8)
        .map(|s| Value::Number(Number(s.trim().to_string())))
}

pub fn string() -> Parser<u8, Value> {
    let special_char = sym(b'\\')
        | sym(b'/')
        | sym(b'"')
        | sym(b'b').map(|_| b'\x08')
        | sym(b'f').map(|_| b'\x0C')
        | sym(b'n').map(|_| b'\n')
        | sym(b'r').map(|_| b'\r')
        | sym(b't').map(|_| b'\t');
    let escape_sequence = sym(b'\\') * special_char;
    let string = space().discard() * sym(b'"') * (none_of(b"\\\"") | escape_sequence).repeat(0..)
        - sym(b'"')
        - space().discard();
    string
        .convert(String::from_utf8)
        .map(|s| Value::String(s.trim().to_string()))
}

pub fn body() -> Parser<u8, Body> {
    (call(expr)).repeat(0..).map(|v| Body(v))
}

pub fn expr() -> Parser<u8, Expr> {
    class_def().map(|c| Expr::ClassDef(c))
        | call(function_def).map(|f| Expr::FunctionDef(f))
        | call(whileloop).map(|w| Expr::WhileLoop(w))
        | (call(assignment) - sym(b';').opt()).map(|a| Expr::Assignment(a))
        | (call(value) - sym(b';').opt()).map(|v| Expr::Value(v))
}

// match an exact string, ignoring spaces before and after
pub fn exact_token(to_match: &'static str) -> Parser<u8, String> {
    (call(space) * seq(to_match.as_bytes()) - call(space))
        .map(|v| String::from_utf8(v.into()).unwrap())
}

pub fn value() -> Parser<u8, Value> {
    call(function)
        | call(number)
        | call(string)
        | call(boolean)
        | call(fncall)
        | call(indexname)
        | call(dotname)
        | call(name)
        | call(group)
        | call(foreign_function)
}

pub fn arguments() -> Parser<u8, Vec<Value>> {
    exact_token("(") * (call(value) - call(space) - sym(b',').opt() - call(space)).repeat(0..)
        - exact_token(")")
}

// hey.you.out.there.in.the.cold
pub fn fncall() -> Parser<u8, Value> {
    let method_call = call(dotname) + call(arguments);
    let regular_call = (call(name) | call(group)) + call(arguments);

    method_call.map(|v: (Value, Vec<Value>)| Value::Call(Call::Method(Box::new(v.0), v.1)))
        | regular_call.map(|v: (Value, Vec<Value>)| Value::Call(Call::Function(Box::new(v.0), v.1)))
}

// hey.you.out.there.in.the.cold
pub fn dotname() -> Parser<u8, Value> {
    ((call(group) | call(name)) - sym(b'.') + list(call(name), sym(b'.')))
        .map(|v: (Value, Vec<Value>)| Value::DotName(DotName(Box::new(v.0), v.1)))
}

// hey["test"][1][another_obj]
pub fn indexname() -> Parser<u8, Value> {
    let predicate = exact_token("[") * call(value) - exact_token("]");

    let subject = call(group) | call(name);

    (subject + predicate.repeat(1..)).map(|v: (Value, Vec<Value>)| {
        let mut combined = Vec::new();
        combined.push(v.0);
        combined.extend(v.1);
        Value::IndexName(IndexName(combined))
    })
}

// struct Point {
//     fn new(self, x, y) {
//         self.x = x
//         self.y = y
//         self
//     }
//     fn println(self) {
//         print("<Point at x:")
//         print(self.x)
//         print(", ")
//         print(self.y)
//         println(">")
//     }
// }
pub fn class_def() -> Parser<u8, ClassDef> {
    let prefix = exact_token("struct") * name() - exact_token("{");
    let class_body = body() - exact_token("}");
    (prefix + class_body).map(|v: (Value, Body)| ClassDef(v.0, v.1))
}

// fn(a, b, asdf)
pub fn function() -> Parser<u8, Value> {
    let named_arguments = exact_token("(") * list(name(), sym(b',')) - exact_token(")");
    let fnbody = exact_token("{") * body() - exact_token("}");
    (exact_token("fn") * named_arguments + fnbody)
        .map(|args: (Vec<Value>, Body)| Value::Function(Function(args.0, args.1)))
}

// fn(a, b, asdf)
pub fn function_def() -> Parser<u8, FunctionDef> {
    let named_arguments = exact_token("(") * list(name(), sym(b',')) - exact_token(")");
    let fnbody = exact_token("{") * body() - exact_token("}");
    (exact_token("fn") * name() + named_arguments + fnbody).map(
        |args: ((Value, Vec<Value>), Body)| FunctionDef((args.0).0, Function((args.0).1, args.1)),
    )
}

// @function
// @ function
pub fn foreign_function() -> Parser<u8, Value> {
    exact_token("@") * name().map(|s| Value::ForeignFunction(ForeignFunction(Box::new(s))))
}

// true
// false
pub fn boolean() -> Parser<u8, Value> {
    exact_token("true").map(|_| Value::Bool(Bool::True))
        | exact_token("false").map(|_| Value::Bool(Bool::False))
}

// ( VALUE )
pub fn group() -> Parser<u8, Value> {
    ((exact_token("(") * call(expr)) - exact_token(")")).map(|e| Value::Group(Group(Box::new(e))))
}

// nasdfasdfeh
// as7sa6dfasdf
// asdfasdoiu
pub fn name() -> Parser<u8, Value> {
    (space() * one_of(ALPHANUMERIC).repeat(1..31) - space()).map(
        |v: Vec<u8>| {
            Value::Name(String::from_utf8(v).unwrap())
        },
    )
}

// NAME = VALUE
// INDEXNAME = VALUE
// DOTNAME = VALUE
pub fn assignment() -> Parser<u8, Assignment> {
    ((call(name) - exact_token("=")) + call(value))
        .map(|value: (Value, Value)| Assignment::Name(Box::new(value.0), Box::new(value.1)))
        | ((call(indexname) - exact_token("=")) + call(value)).map(|value: (Value, Value)| {
            Assignment::IndexName(Box::new(value.0), Box::new(value.1))
        })
        | ((call(dotname) - exact_token("=")) + call(value))
            .map(|value: (Value, Value)| Assignment::DotName(Box::new(value.0), Box::new(value.1)))
}

// while VALUE { BODY }
pub fn whileloop() -> Parser<u8, WhileLoop> {
    let prefix = exact_token("while") * call(value) - exact_token("{");
    let suffix = call(body) - exact_token("}");
    (prefix + suffix).map(|v: (Value, Body)| WhileLoop(Box::new(v.0), v.1))
}
