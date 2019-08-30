use crate::backend::*;


use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::{String, ToString};


#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Literal {
    String(String),
    Number(String),
}

impl Compile for Literal {
    fn compile(self) -> Result<String, Error> {
        match self {
            Self::String(s) => Ok(push(string(quote(s)))),
            Self::Number(n) => Ok(push(number(n))),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct FnCall(pub Box<Value>, pub Vec<Value>);

impl Compile for FnCall {
    fn compile(self) -> Result<String, Error> {
        let FnCall(function, arguments) = self;
        let compiled_args = arguments
            .iter()
            .rev()
            .map(|arg| (arg.clone()).compile().unwrap())
            .map(copy)
            .collect::<String>();

        if let Value::Name(Name::DotName(head, idents)) = (*function).clone() {
            let actual_idents = idents[..idents.len() - 1].to_vec();
            let Identifier(method_name) = &idents[idents.len() - 1];
            Ok(compiled_args
                + &Name::DotName(head, actual_idents).compile()?
                + &method_call(string(quote(method_name))))
        } else {
            Ok(compiled_args + &call(function.compile()?))
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Identifier(pub String);

impl Compile for Identifier {
    fn compile(self) -> Result<String, Error> {
        let Identifier(name) = self;
        Ok(push(string(quote(name))))
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Function(pub Vec<Identifier>, pub Suite);

impl Compile for Function {
    fn compile(self) -> Result<String, Error> {
        let Function(parameters, body) = self;

        let stores = parameters
            .iter()
            .map(|s| store((*s).clone().compile().unwrap()))
            .collect::<String>();

        Ok(push(func(stores + &body.compile()?)))
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct FunctionDef(pub Name, pub Function);

impl Compile for FunctionDef {
    fn compile(self) -> Result<String, Error> {
        let FunctionDef(name, function) = self;

        Ok(Expr::Assignment(name, Value::Function(function)).compile()?)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Value {
    Name(Name),
    Literal(Literal),
    FnCall(FnCall),
    Function(Function),
}

impl Compile for Value {
    fn compile(self) -> Result<String, Error> {
        match self {
            Self::Name(name) => match name {
                Name::Name(n) => n.compile().and_then(|n| Ok(load(n))),
                otherwise => otherwise.compile(),
            },
            Self::Literal(l) => l.compile(),
            Self::FnCall(f) => f.compile(),
            Self::Function(f) => f.compile(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Name {
    Name(Identifier),
    DotName(Box<Value>, Vec<Identifier>),
    IndexName(Box<Value>, Vec<Value>),
}

impl Compile for Name {
    fn compile(self) -> Result<String, Error> {
        match self {
            Self::Name(n) => n.compile(),
            Self::DotName(head, tail) => Ok(dotname((*head).clone(), tail)),
            Self::IndexName(head, tail) => Ok(indexname((*head).clone(), tail)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Expr {
    Assignment(Name, Value),
    WhileLoop(Value, Suite),
    IfThenElse(Value, Suite, Suite),
    FunctionDef(FunctionDef),
    StructDef(StructDef),
    Value(Value),
}

impl Compile for Expr {
    fn compile(self) -> Result<String, Error> {
        match self {
            Self::Assignment(name, value) => match name {
                Name::Name(n) => Ok(store(copy(value.compile()?) + &n.compile()?)),
                otherwise => Ok(assign(copy(value.compile()?) + &otherwise.compile()?)),
            },
            Self::WhileLoop(condition, body) => {
                Ok(while_loop(condition.compile()?, body.compile()?))
            }
            Self::IfThenElse(condition, then_body, else_body) => Ok(if_then_else(
                condition.compile()?,
                then_body.compile()?,
                else_body.compile()?,
            )),
            Self::FunctionDef(function_def) => Ok(function_def.compile()?),
            Self::StructDef(struct_def) => Ok(struct_def.compile()?),
            Self::Value(value) => Ok(value.compile()?),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Suite(pub Vec<Expr>);

impl Compile for Suite {
    fn compile(self) -> Result<String, Error> {
        let Suite(exprs) = self;
        Ok(exprs
            .iter()
            .map(|c| c.clone().compile().unwrap())
            .collect::<String>())
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct StructDef(pub Name, pub Vec<FunctionDef>);

impl Compile for StructDef {
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
        Ok(Expr::Assignment(name, Value::Function(constructor)).compile()?)
    }
}
