use crate::parser::statement::Statement;
use crate::parser::expression::Expression;
use crate::interpreter::scope::*;
use crate::parser::types::{Value, Access};

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub scope: Scope,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            scope: Scope::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            self.execute(statement);
        }
    }

    fn execute(&mut self, statement: Statement) {
        match statement {
            Statement::Expression(expression) => {
                self.evaluate(expression);
            },
            Statement::VariableDeclaration(name, expression, type_) => {
                let value = self.evaluate(expression);
                self.scope.declare_variable(name, value, type_);
            },
            Statement::ConstantDeclaration(name, expression, type_) => {
                let value = self.evaluate(expression);
                self.scope.declare_constant(name, value, type_);
            },
            Statement::TypeDeclaration(name, types) => {
                self.scope.declare_type(name, types);
            },
            Statement::AccessDeclaration(access, statement) => {
                self.scope.declare_access(access, *statement);
            },
            Statement::ExportDeclaration(statement) => {
                self.scope.declare_export(*statement);
            },
            Statement::FunctionDeclaration(name, variables, statements, type_) => {
                let mut parameters = Vec::new();
                for variable in variables {
                    parameters.push(Variable {
                        name: variable.name,
                        value: Value::Undefined,
                        _type: variable.type_,
                        constant: false,
                        access: Access::Public,
                    });
                }
                self.scope.declare_function(name, parameters, statements, type_);
            },
            Statement::Return(expression) => {
                self.evaluate(expression);
            },
            _ => (),
        }
    }

    fn evaluate(&mut self, expression: Expression) -> Value {
        match expression {
            Expression::Variable(name) => {
                self.scope.get_variable(name)
            },
            Expression::Number(number) => {
                Value::Number(number)
            },
            Expression::String(string) => {
                Value::String(string)
            },
            Expression::Boolean(boolean) => {
                Value::Boolean(boolean)
            },
            Expression::Array(expressions) => {
                let mut values = Vec::new();
                for expression in expressions {
                    values.push(Box::new(self.evaluate(*expression)));
                }
                Value::Array(values)
            },
            Expression::Map(expressions) => {
                let mut values = Vec::new();
                for (key, value) in expressions {
                    values.push((Box::new(self.evaluate(*key)), Box::new(self.evaluate(*value))));
                }
                Value::Map(values)
            },
            Expression::Null => {
                Value::Null
            },
            Expression::EndOfFile => {
                Value::Undefined
            },
            _ => todo!()
        }
    }

    pub fn get_scope(&self) -> Scope {
        self.scope.clone()
    }
}