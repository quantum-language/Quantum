use crate::vm::opcode::{
    Const,
    Local,
    Opcode,
};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;

pub struct VM {
    pub(crate) stack: Vec<Const>,
    pub(crate) regs: Vec<Const>,
    pub(crate) pc: usize,
    pub(crate) code: Vec<Opcode>,
    pub(crate) funcs: HashMap<usize, usize>,
}

impl VM {
    pub fn new(code: Vec<Opcode>, funcs: HashMap<usize, usize>) -> Self {
        Self {
            stack: Vec::new(),
            regs: Vec::new(),
            pc: 0,
            code,
            funcs,
        }
    }

    pub fn run(&self) {
        loop {
            let op = &self.code[self.pc];
            match op {
                Opcode::Move { from, to } => {
                    let val = self.get(from);
                    self.set(to, val.clone());
                }
                Opcode::Const { to, value } => {
                    self.set(to, value.clone());
                }
                Opcode::Add { left, right, to } => {
                    let left = self.get(left);
                    let right = self.get(right);
                    let val = match (left, right) {
                        (Const::Int(l), Const::Int(r)) => Const::Int(l + r),
                        (Const::Float(l), Const::Float(r)) => Const::Float(l + r),
                        (Const::Float(l), Const::Int(r)) => Const::Float(l + r as f64),
                        (Const::Int(l), Const::Float(r)) => Const::Float(l as f64 + r),
                        _ => panic!("Cannot add {:?} and {:?}", left, right),
                    };
                    self.set(to, val);
                }
                Opcode::Sub { left, right, to } => {
                    let left = self.get(left);
                    let right = self.get(right);
                    let val = match (left, right) {
                        (Const::Int(l), Const::Int(r)) => Const::Int(l - r),
                        (Const::Float(l), Const::Float(r)) => Const::Float(l - r),
                        (Const::Float(l), Const::Int(r)) => Const::Float(l - r as f64),
                        (Const::Int(l), Const::Float(r)) => Const::Float(l as f64 - r),
                        _ => panic!("Cannot subtract {:?} and {:?}", left, right),
                    };
                    self.set(to, val);
                }
                Opcode::Mul { left, right, to } => {
                    let left = self.get(left);
                    let right = self.get(right);
                    let val = match (left, right) {
                        (Const::Int(l), Const::Int(r)) => Const::Int(l * r),
                        (Const::Float(l), Const::Float(r)) => Const::Float(l * r),
                        (Const::Float(l), Const::Int(r)) => Const::Float(l * r as f64),
                        (Const::Int(l), Const::Float(r)) => Const::Float(l as f64 * r),
                        _ => panic!("Cannot multiply {:?} and {:?}", left, right),
                    };
                    self.set(to, val);
                }
                Opcode::Div { left, right, to } => {
                    let left = self.get(left);
                    let right = self.get(right);
                    let val = match (left, right) {
                        (Const::Int(l), Const::Int(r)) => Const::Int(l / r),
                        (Const::Float(l), Const::Float(r)) => Const::Float(l / r),
                        (Const::Float(l), Const::Int(r)) => Const::Float(l / r as f64),
                        (Const::Int(l), Const::Float(r)) => Const::Float(l as f64 / r),
                        _ => panic!("Cannot divide {:?} and {:?}", left, right),
                    };
                    self.set(to, val);
                }
                Opcode::Mod { left, right, to } => {
                    let left = self.get(left);
                    let right = self.get(right);
                    let val = match (left, right) {
                        (Const::Int(l), Const::Int(r)) => Const::Int(l % r),
                        (Const::Float(l), Const::Float(r)) => Const::Float(l % r),
                        (Const::Float(l), Const::Int(r)) => Const::Float(l % r as f64),
                        (Const::Int(l), Const::Float(r)) => Const::Float(l as f64 % r),
                        _ => panic!("Cannot mod {:?} and {:?}", left, right),
                    };
                    self.set(to, val);
                }
                Opcode::Call { func, args, to } => {
                    let func = self.funcs.get(func).unwrap();
                    self.pc = *func;
                    continue;
                }
                Opcode::Ret { from } => {
                    return;
                }
            }
            self.pc += 1;
        }
    }

    fn get(&self, local: &Local) -> Const {
        match local {
            Local::Reg(reg) => self.regs[*reg].clone(),
            Local::Stack(stack) => self.stack[*stack].clone(),
        }
    }

    fn set(&mut self, local: &Local, value: Const) {
        match local {
            Local::Reg(reg) => self.regs[*reg] = value,
            Local::Stack(stack) => self.stack[*stack] = value,
        }
    }
}

impl Debug for VM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VM {{")?;
        write!(f, " stack: {:?},", self.stack)?;
        write!(f, " regs: {:?},", self.regs)?;
        write!(f, " pc: {:?},", self.pc)?;
        write!(f, " code: {:?},", self.code)?;
        write!(f, " funcs: {:?},", self.funcs)?;
        write!(f, " }}")
    }
}