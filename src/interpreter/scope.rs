use crate::parser::types::*;
use crate::parser::statement::Statement;
use crate::parser::statement;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Scope {
    pub name: String,
    pub parent: Option<Box<Scope>>,
    pub variables: HashMap<String, Variable>,
    pub functions: HashMap<String, Function>,
    pub classes: HashMap<String, Class>,
    pub interfaces: HashMap<String, Interface>,
    pub types: HashMap<String, CustomType>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            name: String::from("global"),
            parent: None,
            variables: HashMap::new(),
            functions: HashMap::new(),
            classes: HashMap::new(),
            interfaces: HashMap::new(),
            types: HashMap::new(),
        }
    }

    pub fn new_child(&self, name: String) -> Scope {
        Scope {
            name,
            parent: Some(Box::new(self.clone())),
            variables: HashMap::new(),
            functions: HashMap::new(),
            classes: HashMap::new(),
            interfaces: HashMap::new(),
            types: HashMap::new(),
        }
    }

    pub fn get_variable(&self, name: String) -> Value {
        if let Some(variable) = self.variables.get(&name) {
            variable.value.clone()
        } else if let Some(parent) = &self.parent {
            parent.get_variable(name)
        } else {
            Value::Null
        }
    }

    pub fn declare_variable(&mut self, name: String, value: Value, type_: Type) {
        self.variables.insert(name.clone(), Variable {
            name,
            value,
            _type: type_,
            constant: false,
            access: Access::Public,
        });
    }

    pub fn declare_constant(&mut self, name: String, value: Value, type_: Type) {
        self.variables.insert(name.clone(), Variable {
            name,
            value,
            _type: type_,
            constant: true,
            access: Access::Public,
        });
    }

    pub fn declare_type(&mut self, name: String, types: Vec<Type>) {
        self.types.insert(name.clone(), CustomType {
            name,
            types,
            access: Access::Public,
        });
    }

    pub fn declare_access(&mut self, access: Access, statement: Statement) {
        match statement {
            Statement::VariableDeclaration(name, expression, type_) => {
                let value = self.get_variable(name.clone());
                self.variables.insert(name.clone(), Variable {
                    name,
                    value,
                    _type: type_,
                    constant: false,
                    access,
                });
            },
            Statement::ConstantDeclaration(name, expression, type_) => {
                let value = self.get_variable(name.clone());
                self.variables.insert(name.clone(), Variable {
                    name,
                    value,
                    _type: type_,
                    constant: true,
                    access,
                });
            },
            Statement::TypeDeclaration(name, types) => {
                self.types.insert(name.clone(), CustomType {
                    name,
                    types,
                    access,
                });
            },
            // Statement::FunctionDeclaration(name, parameters, body, return_type) => {
            //     self.functions.insert(name, Function {
            //         name,
            //         parameters,
            //         body,
            //         access,
            //     });
            // },
            _ => (),
        }
    }

    pub fn declare_export(&mut self, statement: Statement) {
        match statement {
            Statement::VariableDeclaration(name, expression, type_) => {
                let value = self.get_variable(name.clone());
                self.variables.insert(name.clone(), Variable {
                    name,
                    value,
                    _type: type_,
                    constant: false,
                    access: Access::Public,
                });
            },
            Statement::ConstantDeclaration(name, expression, type_) => {
                let value = self.get_variable(name.clone());
                self.variables.insert(name.clone(), Variable {
                    name,
                    value,
                    _type: type_,
                    constant: true,
                    access: Access::Public,
                });
            },
            Statement::TypeDeclaration(name, types) => {
                self.types.insert(name.clone(), CustomType {
                    name,
                    types,
                    access: Access::Public,
                });
            },
            // Statement::FunctionDeclaration(name, parameters, body, return_type) => {
            //     self.functions.insert(name, Function {
            //         name,
            //         parameters,
            //         body,
            //         access: Access::Public,
            //     });
            // },
            _ => (),
        }
    }

    pub fn declare_function(&mut self, name: String, parameters: Vec<Variable>, body: Vec<Statement>, return_type: Type) {
        self.functions.insert(name.clone(), Function {
            name,
            parameters,
            body,
            return_type,
            access: Access::Public,
        });
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: Value,
    pub _type: Type,
    pub constant: bool,
    pub access: Access,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Variable>,
    pub body: Vec<Statement>,
    pub return_type: Type,
    pub access: Access,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub parent: Option<String>,
    pub variables: HashMap<String, Variable>,
    pub functions: HashMap<String, Function>,
    pub access: Access,
}

#[derive(Debug, Clone)]
pub struct Interface {
    pub name: String,
    pub functions: HashMap<String, Function>,
}

#[derive(Debug, Clone)]
pub struct CustomType {
    pub name: String,
    pub types: Vec<Type>,
    pub access: Access,
}