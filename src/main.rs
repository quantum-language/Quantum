mod vm;
mod lexer;
mod tokens;
mod parser;
mod interpreter;

use crate::vm::vm::VM;
use crate::vm::opcode::{Opcode, Const, Local};
use crate::lexer::lexer::Lexer;
use crate::tokens::token::{Token, TokenType};
use crate::parser::parser::parse;
use crate::interpreter::interpreter::Interpreter;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file: File = File::open("examples/Main.quantum").unwrap();
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    let mut lexer = Lexer::new(contents);
    let tokens = lexer.scan_tokens();
    println!("{:#?}", tokens);

    let mut parser = parse(tokens);
    println!("{:#?}", parser);

    let mut interpreter = Interpreter::new();
    interpreter.interpret(parser);

    println!("{:#?}", interpreter.get_scope());


    // let mut code = Vec::new();
    // let mut funcs = HashMap::new();
    // let mut regs: Vec<Const> = Vec::new();
    // let mut stack: Vec<Const> = Vec::new();

    // let add = code.len();
    // funcs.insert(0, add);
    // code.push(Opcode::Const {
    //     to: Local::Reg(0),
    //     value: Const::Int(3),
    // });
    // code.push(Opcode::Const {
    //     to: Local::Reg(1),
    //     value: Const::Int(4),
    // });
    // code.push(Opcode::Add {
    //     left: Local::Reg(0),
    //     right: Local::Reg(1),
    //     to: Local::Reg(2),
    // });
    // code.push(Opcode::Halt);

    // let mut vm = VM::new(code, funcs);
    // vm.run();
    // println!("{:#?}", vm);
}
