use crate::parser::expression::Expression;
use crate::parser::types::*;

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    VariableDeclaration(String, Expression, Type),
    ConstantDeclaration(String, Expression, Type),
    TypeDeclaration(String, Vec<Type>),
    AccessDeclaration(Access, Box<Statement>),
    ExportDeclaration(Box<Statement>),
    FunctionDeclaration(String, Vec<Variable>, Vec<Statement>, Type),
    Return(Expression),
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub type_: Type,
}