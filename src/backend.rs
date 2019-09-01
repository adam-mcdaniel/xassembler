use crate::{Identifier, Value};

use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Error {
    InvalidFunctionName,
}

pub trait Compile {
    fn compile(self) -> Result<String, Error>;
}

const MACHINE_NAME: &str = "xasm";

pub fn quote(name: impl ToString) -> String {
    format!("\"{}\"", name.to_string())
}

pub fn push(value: impl ToString) -> String {
    format!("{}.push({});", MACHINE_NAME, value.to_string())
}

pub fn load(identifier: impl ToString) -> String {
    format!(
        "{name}{MACHINE}.load();",
        MACHINE = MACHINE_NAME,
        name = identifier.to_string()
    )
}

pub fn number(number: impl ToString) -> String {
    format!("Value::number({})", number.to_string())
}

pub fn string(string: impl ToString) -> String {
    format!("Value::string({})", string.to_string())
}

pub fn store(value: impl ToString) -> String {
    format!("{}{}.store();", value.to_string(), MACHINE_NAME)
}

pub fn copy(value: impl ToString) -> String {
    format!("{}{}.copy();", value.to_string(), MACHINE_NAME)
}

pub fn func(body: impl ToString) -> String {
    format!(
        "Value::function(|{MACHINE}: &mut Machine| {{{func}}}, &{MACHINE})",
        func = body.to_string(),
        MACHINE = MACHINE_NAME
    )
}

pub fn foreign_func(name: impl ToString) -> String {
    format!(
        "Value::function({name}, &{MACHINE})",
        name = name.to_string(),
        MACHINE = MACHINE_NAME
    )
}

pub fn while_loop(condition: impl ToString, body: impl ToString) -> String {
    format!(
        "{body}{condition}{MACHINE}.while_loop();",
        MACHINE = MACHINE_NAME,
        condition = push(func(condition)),
        body = push(func(body))
    )
}

pub fn if_then_else(
    condition: impl ToString,
    then_fn: impl ToString,
    else_fn: impl ToString,
) -> String {
    format!(
        "{else_fn}{then_fn}{condition}{MACHINE}.if_then_else();",
        MACHINE = MACHINE_NAME,
        condition = push(func(condition)),
        then_fn = push(func(then_fn)),
        else_fn = push(func(else_fn))
    )
}

pub fn call(func: impl ToString) -> String {
    format!(
        "{func}{MACHINE}.call();",
        MACHINE = MACHINE_NAME,
        func = func.to_string()
    )
}

pub fn method_call(method_name: impl ToString) -> String {
    format!(
        "{method_name}{MACHINE}.method_call();",
        method_name = push(method_name.to_string()),
        MACHINE = MACHINE_NAME
    )
}

pub fn assign(pointer_value: impl ToString) -> String {
    format!(
        "{pointer_value}{MACHINE}.assign();",
        pointer_value = pointer_value.to_string(),
        MACHINE = MACHINE_NAME
    )
}

pub fn dotname(head: Value, tail: Vec<Identifier>) -> String {
    let mut result = head.compile().unwrap();
    for ident in tail {
        let Identifier(name) = ident;
        result += &(push(string(quote(name))) + &format!("{}.index();", MACHINE_NAME));
    }
    result
}

pub fn indexname(head: Value, tail: Vec<Value>) -> String {
    let mut result = head.compile().unwrap();
    for value in tail {
        result += &(value.compile().unwrap() + &format!("{}.index();", MACHINE_NAME));
    }
    result
}
