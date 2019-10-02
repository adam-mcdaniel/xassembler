use crate::target::*;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Literal {
    String(String),
    Number(String),
    ForeignFunction(String),
}

impl<T: Target> Compile<T> for Literal {
    fn compile(self) -> Result<String, Error> {
        match self {
            Self::String(s) => Ok(T::push(T::string(T::quote(s)))),
            Self::Number(n) => Ok(T::push(T::number(n))),
            Self::ForeignFunction(f) => Ok(T::push(T::foreign_func(f))),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct FnCall(pub Box<Value>, pub Vec<Value>);

impl<T: Target> Compile<T> for FnCall {
    fn compile(self) -> Result<String, Error> {
        let FnCall(function, arguments) = self;
        let compiled_args = arguments
            .iter()
            .rev()
            .map(|arg| {
                let value = arg.clone();
                Compile::<T>::compile(value).unwrap()
            })
            .map(T::copy)
            .collect::<String>();

        if let Value::Name(Name::DotName(head, idents)) = (*function).clone() {
            let actual_idents = idents[..idents.len() - 1].to_vec();
            let Identifier(method_name) = &idents[idents.len() - 1];
            Ok(compiled_args
                + &Compile::<T>::compile(Name::DotName(head, actual_idents))?
                + &T::method_call(T::string(T::quote(method_name))))
        } else {
            Ok(compiled_args + &T::call(Compile::<T>::compile((*function).clone())?))
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Identifier(pub String);

impl<T: Target> Compile<T> for Identifier {
    fn compile(self) -> Result<String, Error> {
        let Identifier(name) = self;
        Ok(T::push(T::string(T::quote(name))))
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Function(pub Vec<Identifier>, pub Suite);

impl<T: Target> Compile<T> for Function {
    fn compile(self) -> Result<String, Error> {
        let Function(parameters, body) = self;

        let stores = parameters
            .iter()
            .map(|s| T::store(Compile::<T>::compile((*s).clone()).unwrap()))
            .collect::<String>();

        Ok(T::push(T::func(stores + &Compile::<T>::compile(body)?)))
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct FunctionDef(pub Name, pub Function);

impl<T: Target> Compile<T> for FunctionDef {
    fn compile(self) -> Result<String, Error> {
        let FunctionDef(name, function) = self;

        Ok(Compile::<T>::compile(Expr::Assignment(
            name,
            Value::Function(function),
        ))?)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Value {
    Name(Name),
    Literal(Literal),
    FnCall(FnCall),
    Function(Function),
}

impl<T: Target> Compile<T> for Value {
    fn compile(self) -> Result<String, Error> {
        match self {
            Self::Name(name) => match name {
                Name::Name(n) => Compile::<T>::compile(n).and_then(|n| Ok(T::load(n))),
                otherwise => Compile::<T>::compile(otherwise),
            },
            Self::Literal(l) => Compile::<T>::compile(l),
            Self::FnCall(f) => Compile::<T>::compile(f),
            Self::Function(f) => Compile::<T>::compile(f),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Name {
    Name(Identifier),
    IndexName(Box<Value>, Vec<Value>),
    DotName(Box<Value>, Vec<Identifier>),
}

impl<T: Target> Compile<T> for Name {
    fn compile(self) -> Result<String, Error> {
        match self {
            Self::Name(n) => Compile::<T>::compile(n),
            Self::DotName(head, tail) => Ok(T::dotname((*head).clone(), tail)),
            Self::IndexName(head, tail) => Ok(T::indexname((*head).clone(), tail)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Expr {
    Assignment(Name, Value),
    ForLoop {
        counter: Identifier,
        element: Identifier,
        list: Value,
        body: Suite,
    },
    WhileLoop(Value, Suite),
    IfThenElse(Value, Suite, Suite),
    FunctionDef(FunctionDef),
    StructDef(StructDef),
    Value(Value),
}

impl<T: Target> Compile<T> for Expr {
    fn compile(self) -> Result<String, Error> {
        match self {
            Self::Assignment(name, value) => match name {
                Name::Name(n) => Ok(T::store(
                    T::copy(Compile::<T>::compile(value)?) + &Compile::<T>::compile(n)?,
                )),
                otherwise => Ok(T::assign(
                    T::copy(Compile::<T>::compile(value)?) + &Compile::<T>::compile(otherwise)?,
                )),
            },
            Self::WhileLoop(condition, body) => Ok(T::while_loop(
                Compile::<T>::compile(condition)?,
                Compile::<T>::compile(body)?,
            )),
            Self::ForLoop {
                counter,
                element,
                list,
                body,
            } => Ok(T::for_loop(
                Compile::<T>::compile(counter)?,
                Compile::<T>::compile(element)?,
                Compile::<T>::compile(list)?,
                Compile::<T>::compile(body)?,
            )),
            Self::IfThenElse(condition, then_body, else_body) => Ok(T::if_then_else(
                Compile::<T>::compile(condition)?,
                Compile::<T>::compile(then_body)?,
                Compile::<T>::compile(else_body)?,
            )),
            Self::FunctionDef(function_def) => Ok(Compile::<T>::compile(function_def)?),
            Self::StructDef(struct_def) => Ok(Compile::<T>::compile(struct_def)?),
            Self::Value(value) => Ok(Compile::<T>::compile(value)?),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Suite(pub Vec<Expr>);

impl<T: Target> Compile<T> for Suite {
    fn compile(self) -> Result<String, Error> {
        let Suite(exprs) = self;
        Ok(exprs
            .iter()
            .map(|c| Compile::<T>::compile(c.clone()).unwrap())
            .collect::<String>())
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct StructDef(pub Name, pub Vec<FunctionDef>);

impl<T: Target> Compile<T> for StructDef {
    fn compile(self) -> Result<String, Error> {
        let StructDef(name, function_defs) = self;
        let mut exprs = vec![Expr::Assignment(
            Name::Name(Identifier("self".to_string())),
            Value::FnCall(FnCall(
                Box::new(Value::Name(Name::Name(Identifier("dict".to_string())))),
                vec![],
            )),
        )];

        let self_function_defs = function_defs
            .iter()
            .map(|f| {
                let FunctionDef(name, fun) = f;
                let result_name;

                match name {
                    Name::DotName(_, other_names) => result_name = other_names.clone(),
                    Name::Name(ident) => {
                        result_name = vec![ident.clone()];
                    }
                    _ => {
                        // DO SOMETHING BAD HERE
                        result_name = vec![Identifier("BAD".to_string())];
                    }
                }

                FunctionDef(
                    Name::DotName(
                        Box::new(Value::Name(Name::Name(Identifier("self".to_string())))),
                        result_name,
                    ),
                    fun.clone(),
                )
            })
            .map(|f| Expr::FunctionDef(f.clone()))
            .collect::<Vec<Expr>>();

        exprs.extend(self_function_defs);
        exprs.push(Expr::Value(Value::Name(Name::Name(Identifier(
            "self".to_string(),
        )))));

        let body = Suite(exprs);
        let constructor: Function = Function(vec![], body);
        Ok(Compile::<T>::compile(Expr::Assignment(
            name,
            Value::Function(constructor),
        ))?)
    }
}
