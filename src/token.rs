extern crate alloc;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::String;


#[derive(Debug, PartialEq)]
pub struct Body(Vec<Expr>);

#[derive(Debug, PartialEq)]
pub enum Expr {
    Value(Value),
    Assignment(Assignment),
    WhileLoop(WhileLoop)
}

#[derive(Debug, PartialEq)]
pub struct Comment(String);

#[derive(Debug, PartialEq)]
pub struct ForeignFunction(String);

#[derive(Debug, PartialEq)]
pub struct Function(Vec<String>, Box<Body>);

#[derive(Debug, PartialEq)]
pub enum Call {
    Method(DotName, Vec<Value>),
    Function(Box<Value>, Vec<Value>)
}

#[derive(Debug, PartialEq)]
pub struct DotName(Box<Value>, Vec<String>);

#[derive(Debug, PartialEq)]
pub struct IndexName(Vec<Value>);

#[derive(Debug, PartialEq)]
pub enum Assignment {
    Name(String, Box<Value>),
    DotName(String, DotName),
    IndexName(String, IndexName)
}

#[derive(Debug, PartialEq)]
pub struct WhileLoop(Box<Value>, Box<Body>);

#[derive(Debug, PartialEq)]
pub enum Value {
    IndexName(IndexName),
    DotName(DotName),
    Call(Call),
    Name(String),
    Number(Number),
    String(String),
    Bool(Bool),
    Group(Group),
    Function(Function),
    ForeignFunction(ForeignFunction),
}

#[derive(Debug, PartialEq)]
pub enum Math {
    Multiply(Box<Value>, Box<Value>),
    Divide(Box<Value>, Box<Value>),
    Add(Box<Value>, Box<Value>),
    Subtract(Box<Value>, Box<Value>),
    Modulus(Box<Value>, Box<Value>),
    Greater(Box<Value>, Box<Value>),
    Less(Box<Value>, Box<Value>),
    Equal(Box<Value>, Box<Value>),
    NotEqual(Box<Value>, Box<Value>),
    LessEqual(Box<Value>, Box<Value>),
    GreaterEqual(Box<Value>, Box<Value>)
}

#[derive(Debug, PartialEq)]
pub enum Bool {
    True, False
}

#[derive(Debug, PartialEq)]
pub struct Number(String);

#[derive(Debug, PartialEq)]
pub struct Group(Box<Expr>);

