use crate::tokens::token::TokenType;

#[derive(Debug, Clone)]
pub enum Expression {
    Variable(String),
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Box<Expression>>),
    Map(Vec<(Box<Expression>, Box<Expression>)>),
    Null,
    EndOfFile,

    BinaryOperation(Box<Expression>, TokenType, Box<Expression>),
    UnaryOperation(TokenType, Box<Expression>),
    FunctionCall(String, Vec<Box<Expression>>),
    ArrayAccess(Box<Expression>, Box<Expression>),
    MapAccess(Box<Expression>, Box<Expression>),
    Assignment(Box<Expression>, Box<Expression>),
    Increment(Box<Expression>),
    Decrement(Box<Expression>),
}

