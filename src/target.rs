use crate::{Identifier, Value};

use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub const MACHINE_NAME: &str = "xasm";

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Error {
    InvalidFunctionName,
}

pub trait Compile<T: Target> {
    fn compile(self) -> Result<String, Error>;
}

pub trait Target {
    fn quote(name: impl ToString) -> String {
        format!("\"{}\"", name.to_string())
    }
    fn push(value: impl ToString) -> String;
    fn load(value: impl ToString) -> String;
    fn store(value: impl ToString) -> String;
    fn number(value: impl ToString) -> String;
    fn string(value: impl ToString) -> String;
    fn copy(value: impl ToString) -> String;
    fn block(body: impl ToString) -> String;
    fn func(body: impl ToString) -> String;
    fn foreign_func(value: impl ToString) -> String;
    fn while_loop(condition: impl ToString, body: impl ToString) -> String;
    fn for_loop(
        counter_identifier: impl ToString,
        element_identifier: impl ToString,
        list_value: impl ToString,
        body: impl ToString,
    ) -> String;
    fn if_then_else(
        condition: impl ToString,
        then_fn: impl ToString,
        else_fn: impl ToString,
    ) -> String;
    fn call(func: impl ToString) -> String;
    fn method_call(method_name: impl ToString) -> String;
    fn assign(pointer_value: impl ToString) -> String;
    fn dotname(head: Value, tail: Vec<Identifier>) -> String;
    fn indexname(head: Value, tail: Vec<Value>) -> String;
}
