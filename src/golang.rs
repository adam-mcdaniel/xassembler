use crate::{Compile, Identifier, Target, Value, MACHINE_NAME};

use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub struct Golang;

impl Target for Golang {
    fn push(value: impl ToString) -> String {
        format!("{}.Push({})\n", MACHINE_NAME, value.to_string())
    }

    fn load(identifier: impl ToString) -> String {
        format!(
            "{name}{MACHINE}.Load()\n",
            MACHINE = MACHINE_NAME,
            name = identifier.to_string()
        )
    }

    fn number(number: impl ToString) -> String {
        format!("NewNumber({})", number.to_string())
    }

    fn string(string: impl ToString) -> String {
        format!("NewString({})", string.to_string())
    }

    fn store(value: impl ToString) -> String {
        format!("{}{}.Store()\n", value.to_string(), MACHINE_NAME)
    }

    fn copy(value: impl ToString) -> String {
        format!("{}{}.Copy()\n", value.to_string(), MACHINE_NAME)
    }

    fn func(body: impl ToString) -> String {
        format!(
            "NewFunction(func({MACHINE} *Machine) {{{func}}}, {MACHINE}.Duplicate())",
            func = body.to_string(),
            MACHINE = MACHINE_NAME
        )
    }

    fn foreign_func(name: impl ToString) -> String {
        format!(
            "NewFunction({name}, {MACHINE}.Duplicate())",
            name = name.to_string(),
            MACHINE = MACHINE_NAME
        )
    }

    fn for_loop(
        counter: impl ToString,
        element: impl ToString,
        list: impl ToString,
        body: impl ToString,
    ) -> String {
        format!(
            "{body}{list}{element}{counter}{MACHINE}.ForLoop()\n",
            MACHINE = MACHINE_NAME,
            counter = counter.to_string(),
            element = element.to_string(),
            list = list.to_string(),
            body = Self::push(Self::func(body))
        )
    }

    fn while_loop(condition: impl ToString, body: impl ToString) -> String {
        format!(
            "{body}{condition}{MACHINE}.WhileLoop()\n",
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
            "{else_fn}{then_fn}{condition}{MACHINE}.IfThenElse()\n",
            MACHINE = MACHINE_NAME,
            condition = Self::push(Self::func(condition)),
            then_fn = Self::push(Self::func(then_fn)),
            else_fn = Self::push(Self::func(else_fn))
        )
    }

    fn call(func: impl ToString) -> String {
        format!(
            "{func}{MACHINE}.Call()\n",
            MACHINE = MACHINE_NAME,
            func = func.to_string()
        )
    }

    fn method_call(method_name: impl ToString) -> String {
        format!(
            "{method_name}{MACHINE}.MethodCall()\n",
            method_name = Self::push(method_name.to_string()),
            MACHINE = MACHINE_NAME
        )
    }

    fn assign(pointer_value: impl ToString) -> String {
        format!(
            "{pointer_value}{MACHINE}.Assign()\n",
            pointer_value = pointer_value.to_string(),
            MACHINE = MACHINE_NAME
        )
    }

    fn dotname(head: Value, tail: Vec<Identifier>) -> String {
        let mut result = Compile::<Self>::compile(head).unwrap();
        for ident in tail {
            let Identifier(name) = ident;
            result += &(Self::push(Self::string(Self::quote(name)))
                + &format!("{}.Index()\n", MACHINE_NAME));
        }
        result
    }

    fn indexname(head: Value, tail: Vec<Value>) -> String {
        let mut result = Compile::<Self>::compile(head).unwrap();
        for value in tail {
            result += &(Compile::<Self>::compile(value).unwrap()
                + &format!("{}.Index()\n", MACHINE_NAME));
        }
        result
    }
}
