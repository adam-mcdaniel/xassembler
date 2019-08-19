extern crate pom;
use pom::parser::*;
use pom::Parser;

extern crate alloc;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::str;
use core::str::FromStr;

use crate::*;

const ALPHA: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALPHANUMERIC: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn space() -> Parser<u8, ()> {
	one_of(b" \t\r\n").repeat(0..).discard()
}

pub fn number() -> Parser<u8, Number> {
	let integer = one_of(b"123456789") - one_of(b"0123456789").repeat(0..) | sym(b'0');
	let frac = sym(b'.') + one_of(b"0123456789").repeat(1..);
	let exp = one_of(b"eE") + one_of(b"+-").opt() + one_of(b"0123456789").repeat(1..);
	let number = space().discard() * sym(b'-').opt() + integer + frac.opt() + exp.opt() - space().discard();
	number.collect().convert(str::from_utf8).map(|s| Number(s.trim().to_string()))
}


pub fn string() -> Parser<u8, String> {
    let special_char = sym(b'\\') | sym(b'/') | sym(b'"')
    | sym(b'b').map(|_|b'\x08') | sym(b'f').map(|_|b'\x0C')
    | sym(b'n').map(|_|b'\n') | sym(b'r').map(|_|b'\r') | sym(b't').map(|_|b'\t');
    let escape_sequence = sym(b'\\') * special_char;
    let string = space().discard() * sym(b'"') * (none_of(b"\\\"") | escape_sequence).repeat(0..) - sym(b'"') - space().discard();
    string.convert(String::from_utf8).map(|s| s.trim().to_string())
}


pub fn body() -> Parser<u8, Body> {
    (call(expr)).repeat(0..).map(|v| Body(v))
}


pub fn expr() -> Parser<u8, Expr> {
    call(whileloop).map(|w| Expr::WhileLoop(w))
        | (call(assignment) - sym(b';')).map(|a| Expr::Assignment(a))
        | (call(value) - sym(b';')).map(|v| Expr::Value(v))
}


// match an exact string, ignoring spaces before and after
pub fn exact_token(to_match: &'static str) -> Parser<u8, String> {
    (call(space) * seq(to_match.as_bytes()) - call(space))
        .map(|v| String::from_utf8(v.into()).unwrap())
}

pub fn value() -> Parser<u8, Value> {
	call(boolean).map(|b| Value::Bool(b))
    	| call(function).map(|f| Value::Function(f))
        | call(fncall).map(|c| Value::Call(c))
        | call(dotname).map(|d| Value::DotName(d))
        | call(name).map(|n| Value::Name(n))
        | call(number).map(|n| Value::Number(n))
        | call(string).map(|s| Value::String(s))
        // | group().map(|g| Value::Group(g))
        // | indexname().map(|i| Value::IndexName(i))
        // | foreign_function().map(|f| Value::ForeignFunction(f))
}


pub fn arguments() -> Parser<u8, Vec<Value>> {
    exact_token("(") * (call(value) - call(space) - sym(b',').opt() - call(space)).repeat(0..) - exact_token(")")
}

// hey.you.out.there.in.the.cold
pub fn fncall() -> Parser<u8, Call> {
    let method_call = call(dotname) + call(arguments);
    // let regular_call = 
	// call(value)
	//  + call(arguments);

	method_call.map(|v: (DotName, Vec<Value>)| Call::Method(v.0, v.1))
    	// | regular_call.map(|v: (Value, Vec<Value>)| Call::Function(Box::new(v.0), v.1))
}

// hey.you.out.there.in.the.cold
pub fn dotname() -> Parser<u8, DotName> {
    (
        (
			(
				// call(fncall).map(|c| Value::Call(c))
					// | call(indexname).map(|i| Value::IndexName(i))
				// call(indexname).map(|i| Value::IndexName(i))
				// 	| call(group).map(|g| Value::Group(g))
				// 	| call(name).map(|n| Value::Name(n))
				call(group).map(|g| Value::Group(g))
					// | call(indexname).map(|i| Value::IndexName(i))
					| call(name).map(|n| Value::Name(n))
			) - sym(b'.')
        ) + list(call(name), sym(b'.'))
    ).map(|v: (Value, Vec<String>)| {
        DotName(Box::new(v.0), v.1)
    })
}

// hey["test"][1][another_obj]
pub fn indexname() -> Parser<u8, IndexName> {
    let predicate = exact_token("[") * call(value) - exact_token("]");
    
	let subject = call(group).map(|g| Value::Group(g))
			| call(dotname).map(|d| Value::DotName(d))
			| call(name).map(|n| Value::Name(n))
			;

    (subject + predicate.repeat(1..)).map(|v: (Value, Vec<Value>)| {
        let mut combined = Vec::new();
        combined.push(v.0);
        combined.extend(v.1);
        IndexName(combined)
    })
}

// fn(a, b, asdf)
pub fn function() -> Parser<u8, Function> {
    let arguments = exact_token("(") * list(call(name), sym(b',')) - exact_token(")");
    let fnbody = exact_token("{") * call(body) - exact_token("}");
    (exact_token("fn") * arguments + fnbody)
        .map(|args: (Vec<String>, Body)| {
            Function(args.0, args.1)
        })
}


// @function
// @ function
pub fn foreign_function() -> Parser<u8, ForeignFunction> {
    exact_token("@") * call(name).map(|s| ForeignFunction(s))
}

// true
// false
pub fn boolean() -> Parser<u8, Bool> {
    exact_token("true").map(|_| Bool::True)
        | exact_token("false").map(|_| Bool::False)
}

// ( VALUE )
pub fn group() -> Parser<u8, Group> {
    ((exact_token("(") * call(expr)) - exact_token(")"))
        .map(|e| Group(Box::new(e)))
}

// nasdfasdfeh
// as7sa6dfasdf
// asdfasdoiu
pub fn name() -> Parser<u8, String> {
    (call(space) * one_of(ALPHA) + one_of(ALPHANUMERIC).repeat(1..31))
        .map(|v: (u8, Vec<u8>)| {
            let mut result = Vec::new();
            result.push(v.0);
            result.extend(v.1);
            String::from_utf8(result).unwrap()
        }) - call(space)
}


// NAME = VALUE
// INDEXNAME = VALUE
// DOTNAME = VALUE
pub fn assignment() -> Parser<u8, Assignment> {
    ((call(name) - exact_token("=")) + call(value))
        .map(|value: (String, Value)|
                Assignment::Name(value.0, Box::new(value.1)))

    | ((call(indexname) - exact_token("=")) + call(value))
        .map(|value: (IndexName, Value)|
                Assignment::IndexName(value.0, Box::new(value.1)))

    | ((call(dotname) - exact_token("=")) + call(value))
        .map(|value: (DotName, Value)|
                Assignment::DotName(value.0, Box::new(value.1)))
}


// while VALUE { BODY }
pub fn whileloop() -> Parser<u8, WhileLoop> {
    let prefix = exact_token("while") * call(value) - exact_token("{");
    let suffix = call(body) - exact_token("}");
    (prefix + suffix)
        .map(|v: (Value, Body)| {
                WhileLoop(Box::new(v.0), v.1)
            })
}