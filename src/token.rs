extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

macro_rules! token {
    ($name:ident -> $($t:ty),+) => (
        #[derive(Debug, PartialEq, Clone)]
        pub struct $name($(pub $t, )+);
    );
    ($name:ident { $($i:ident -> $($t:ty),+);+ } ) => (
        #[derive(Debug, PartialEq, Clone)]
        pub enum $name {
            $($i($($t, )+), )+
        }
    );
    ($name:ident { $($i:ident);+} ) => (
        #[derive(Debug, PartialEq, Clone)]
        pub enum $name {
            $($i, )+
        }
    );
}

token!(Body -> Vec<Expr>);

token!(Expr {
    Value -> Value;
    Assignment -> Assignment;
    FunctionDef -> FunctionDef;
    ClassDef -> ClassDef;
    WhileLoop -> WhileLoop
});

token!(ForeignFunction -> Box<Value>);

token!(ClassDef -> Value, Body);
token!(FunctionDef -> Value, Function);

token!(Function -> Vec<Value>, Body);

token!(Call {
    Method -> Box<Value>, Vec<Value>;
    Function -> Box<Value>, Vec<Value>
});

token!(DotName -> Box<Value>, Vec<Value>);

token!(IndexName -> Vec<Value>);

token!(Assignment {
    Name -> Box<Value>, Box<Value>;
    DotName -> Box<Value>, Box<Value>;
    IndexName -> Box<Value>, Box<Value>
});

token!(WhileLoop -> Box<Value>, Body);

token!(Value {
    IndexName -> IndexName;
    DotName -> DotName;
    Call -> Call;
    Name -> String;
    Number -> Number;
    String -> String;
    Bool -> Bool;
    Group -> Group;
    Function -> Function;
    ForeignFunction -> ForeignFunction
});

token!(Math {
    Multiply -> Box<Value>, Box<Value>;
    Divide -> Box<Value>, Box<Value>;
    Subtract -> Box<Value>, Box<Value>;
    Modulus -> Box<Value>, Box<Value>;
    Greater -> Box<Value>, Box<Value>;
    Less -> Box<Value>, Box<Value>;
    Equal -> Box<Value>, Box<Value>;
    NotEqual -> Box<Value>, Box<Value>;
    LessEqual -> Box<Value>, Box<Value>;
    GreaterEqual -> Box<Value>, Box<Value>
});

token!(Bool {
    True;
    False
});

token!(Number -> String);

token!(Group -> Box<Expr>);
