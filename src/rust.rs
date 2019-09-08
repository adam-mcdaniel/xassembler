
use crate::{Target, Compile, MACHINE_NAME, Value, Identifier};

use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub struct Rust;

impl Target for Rust {
    fn push(value: impl ToString) -> String {
        format!("{}.push({});", MACHINE_NAME, value.to_string())
    }

    fn load(identifier: impl ToString) -> String {
        format!(
            "{name}{MACHINE}.load();",
            MACHINE = MACHINE_NAME,
            name = identifier.to_string()
        )
    }

    fn number(number: impl ToString) -> String {
        format!("Value::number({})", number.to_string())
    }

    fn string(string: impl ToString) -> String {
        format!("Value::string({})", string.to_string())
    }

    fn store(value: impl ToString) -> String {
        format!("{}{}.store();", value.to_string(), MACHINE_NAME)
    }

    fn copy(value: impl ToString) -> String {
        format!("{}{}.copy();", value.to_string(), MACHINE_NAME)
    }

    fn func(body: impl ToString) -> String {
        format!(
            "Value::function(|{MACHINE}: &mut Machine| {{{func}}}, &{MACHINE})",
            func = body.to_string(),
            MACHINE = MACHINE_NAME
        )
    }

    fn foreign_func(name: impl ToString) -> String {
        format!(
            "Value::function({name}, &{MACHINE})",
            name = name.to_string(),
            MACHINE = MACHINE_NAME
        )
    }

    fn while_loop(condition: impl ToString, body: impl ToString) -> String {
        format!(
            "{body}{condition}{MACHINE}.while_loop();",
            MACHINE = MACHINE_NAME,
            condition = Self::push(Self::func(condition)),
            body = Self::push(Self::func(body))
        )
    }

    fn if_then_else(
        condition: impl ToString,
        then_fn: impl ToString,
        else_fn: impl ToString,
    ) -> String {
        format!(
            "{else_fn}{then_fn}{condition}{MACHINE}.if_then_else();",
            MACHINE = MACHINE_NAME,
            condition = Self::push(Self::func(condition)),
            then_fn = Self::push(Self::func(then_fn)),
            else_fn = Self::push(Self::func(else_fn))
        )
    }

    fn call(func: impl ToString) -> String {
        format!(
            "{func}{MACHINE}.call();",
            MACHINE = MACHINE_NAME,
            func = func.to_string()
        )
    }

    fn method_call(method_name: impl ToString) -> String {
        format!(
            "{method_name}{MACHINE}.method_call();",
            method_name = Self::push(method_name.to_string()),
            MACHINE = MACHINE_NAME
        )
    }

    fn assign(pointer_value: impl ToString) -> String {
        format!(
            "{pointer_value}{MACHINE}.assign();",
            pointer_value = pointer_value.to_string(),
            MACHINE = MACHINE_NAME
        )
    }

    fn dotname(head: Value, tail: Vec<Identifier>) -> String {
        let mut result = Compile::<Self>::compile(head).unwrap();
        for ident in tail {
            let Identifier(name) = ident;
            result += &(Self::push(Self::string(Self::quote(name))) + &format!("{}.index();", MACHINE_NAME));
        }
        result
    }

    fn indexname(head: Value, tail: Vec<Value>) -> String {
        let mut result = Compile::<Self>::compile(head).unwrap();
        for value in tail {
            result += &(Compile::<Self>::compile(value).unwrap() + &format!("{}.index();", MACHINE_NAME));
        }
        result
    }

}