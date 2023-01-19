use crate::parser::statement::{Statement, Variable};
use crate::parser::expression::Expression;
use crate::parser::types::*;
use crate::tokens::token::{Token, TokenType};

pub fn parse(tokens: Vec<Token>) -> Vec<Statement> {
    let mut statements = Vec::new();
    let mut index = 0;
    while index < tokens.len() {
        statements.push(parse_statement(&tokens, &mut index));
        index += 1;
    }
    statements
}

fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Statement {
    match &tokens[*index].token_type {
        TokenType::Var => {
            *index += 1;
            let name = match &tokens[*index].token_type {
                TokenType::Identifier(name) => name.clone(),
                _ => todo!("Parse error"),
            };
            *index += 1;
            matcht(&tokens, index, TokenType::Colon);
            let type_ = parse_type(&tokens, index);
            matcht(&tokens, index, TokenType::Equal);
            let expression = parse_expression(&tokens, index);
            Statement::VariableDeclaration(name, expression, type_)
        }
        TokenType::Const => {
            *index += 1;
            let name = match &tokens[*index].token_type {
                TokenType::Identifier(name) => name.clone(),
                _ => todo!("Parse error"),
            };
            *index += 1;
            matcht(&tokens, index, TokenType::Colon);
            let type_ = parse_type(&tokens, index);
            matcht(&tokens, index, TokenType::Equal);
            let expression = parse_expression(&tokens, index);
            Statement::ConstantDeclaration(name, expression, type_)
        }
        TokenType::Pub | TokenType::Priv | TokenType::Prot => {
            let access = match &tokens[*index].token_type {
                TokenType::Pub => Access::Public,
                TokenType::Priv => Access::Private,
                TokenType::Prot => Access::Protected,
                _ => todo!("Parse error"),
            };
            *index += 1;
            let statement = parse_statement(&tokens, index);
            Statement::AccessDeclaration(access, Box::new(statement))
        }
        TokenType::Type => {
            *index += 1;
            let name = match &tokens[*index].token_type {
                TokenType::Identifier(name) => name.clone(),
                _ => todo!("Parse error"),
            };
            *index += 1;
            matcht(&tokens, index, TokenType::Equal);
            let mut types = Vec::new();
            while &tokens[*index].token_type != &TokenType::Semicolon {
                types.push(parse_type(&tokens, index));
                if &tokens[*index].token_type == &TokenType::Pipe {
                    *index += 1;
                }
            }
            Statement::TypeDeclaration(name, types)
        }
        TokenType::Export => {
            *index += 1;
            let statement = parse_statement(&tokens, index);
            Statement::ExportDeclaration(Box::new(statement))
        }
        TokenType::Func => {
            *index += 1;
            let name = match &tokens[*index].token_type {
                TokenType::Identifier(name) => name.clone(),
                _ => todo!("Parse error"),
            };
            *index += 1;
            matcht(&tokens, index, TokenType::LeftParen);
            let mut variables = Vec::new();
            while &tokens[*index].token_type != &TokenType::RightParen {
                let name = match &tokens[*index].token_type {
                    TokenType::Identifier(name) => name.clone(),
                    _ => todo!("Parse error"),
                };
                *index += 1;
                matcht(&tokens, index, TokenType::Colon);
                let type_ = parse_type(&tokens, index);
                variables.push(Variable { name, type_ });
                if &tokens[*index].token_type == &TokenType::Comma {
                    *index += 1;
                }
            }
            *index += 1;
            matcht(&tokens, index, TokenType::Colon);
            let return_type = parse_type(&tokens, index);
            matcht(&tokens, index, TokenType::LeftBrace);
            let mut statements = Vec::new();
            while &tokens[*index].token_type != &TokenType::RightBrace {
                statements.push(parse_statement(&tokens, index));
                *index += 1;
            }
            Statement::FunctionDeclaration(name, variables, statements, return_type)
        }
        TokenType::Return => {
            *index += 1;
            let expression = parse_expression(&tokens, index);
            Statement::Return(expression)
        }
        TokenType::EndOfFile => Statement::Expression(Expression::EndOfFile),
        _ => {
            let expression = parse_expression(&tokens, index);
            Statement::Expression(expression)
        }
    }
}

fn parse_type(tokens: &Vec<Token>, index: &mut usize) -> Type {
    match &tokens[*index].token_type {
        TokenType::Identifier(name) => {
            *index += 1;
            match name.as_str() {
                "Int" => Type::Int,
                "Float" => Type::Float,
                "String" => Type::String,
                "Bool" => Type::Bool,
                "Any" => Type::Any,
                "Void" => Type::Void,
                "Null" => Type::Null,
                "Array" => {
                    matcht(&tokens, index, TokenType::LeftBracket);
                    let type_ = parse_type(&tokens, index);
                    matcht(&tokens, index, TokenType::RightBracket);
                    Type::Array(Box::new(type_))
                }
                "Map" => {
                    matcht(&tokens, index, TokenType::LeftBracket);
                    let key_type = parse_type(&tokens, index);
                    matcht(&tokens, index, TokenType::Comma);
                    let value_type = parse_type(&tokens, index);
                    matcht(&tokens, index, TokenType::RightBracket);
                    Type::Map(Box::new(key_type), Box::new(value_type))
                }
                _ => todo!("Parse other types"),
            }
        }
        _ => todo!("Parse other types"),
    }
}

fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Expression {
    match &tokens[*index].token_type {
        TokenType::Identifier(name) => {
            *index += 1;
            Expression::Variable(name.clone())
        }
        TokenType::Number(number) => {
            *index += 1;
            Expression::Number(number.clone())
        }
        TokenType::String(string) => {
            *index += 1;
            Expression::String(string.clone())
        }
        TokenType::True => {
            *index += 1;
            Expression::Boolean(true)
        }
        TokenType::False => {
            *index += 1;
            Expression::Boolean(false)
        }
        TokenType::Null => {
            *index += 1;
            Expression::Null
        }
        TokenType::LeftParen => {
            *index += 1;
            let expression = parse_expression(&tokens, index);
            matcht(&tokens, index, TokenType::RightParen);
            expression
        }
        _ => todo!("unhandled expression type {:?}", tokens[*index].token_type),
    }
}

fn matcht(tokens: &Vec<Token>, index: &mut usize, token_type: TokenType) -> bool {
    if tokens[*index].token_type == token_type {
        *index += 1;
        true
    } else {
        panic!("Expected {:?} but got {:?}", token_type, tokens[*index].token_type);
    }
}