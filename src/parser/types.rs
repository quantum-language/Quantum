#[derive(Debug, Clone)]
pub enum Access {
    Public,
    Private,
    Protected,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Box<Value>>),
    Map(Vec<(Box<Value>, Box<Value>)>),
    Null,
    Undefined,
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    String,
    Char,
    Bool,
    Null,
    Undefined,
    Any,
    Void,
    Array(Box<Type>),
    Map(Box<Type>, Box<Type>),
}