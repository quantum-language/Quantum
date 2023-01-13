mod vm;

use crate::vm::vm::VM;
use crate::vm::opcode::{Opcode, Const, Local};
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file: File = File::open("examples/Token.quantum").unwrap();
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    let mut code = Vec::new();
    let mut funcs = HashMap::new();
    let mut regs: Vec<Const> = Vec::new();
    let mut stack: Vec<Const> = Vec::new();

    let add = code.len();
    funcs.insert(0, add);
    code.push(Opcode::Const {
        to: Local::Reg(0),
        value: Const::Int(3),
    });
    code.push(Opcode::Const {
        to: Local::Reg(1),
        value: Const::Int(4),
    });
    code.push(Opcode::Add {
        left: Local::Reg(0),
        right: Local::Reg(1),
        to: Local::Reg(2),
    });
    code.push(Opcode::Halt);

    let mut vm = VM::new(code, funcs);
    vm.run();
    println!("{:#?}", vm);
}
