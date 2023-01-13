use crate::vm::opcode::{
    Const,
    Local,
    Opcode,
};
use core::panic;
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

    pub fn run(&mut self) {
        loop {
            let op = self.code[self.pc].clone();
            match op {
                Opcode::Load { to, value } => {
                    match to.clone() {
                        Local::Reg(reg) => {
                            self.regs[reg] = value.clone();
                        }
                        Local::Stack(stack) => {
                            self.stack[stack] = value.clone();
                        }
                        _ => todo!(),
                    }
                }
                Opcode::Store { from } => {
                    match from.clone() {
                        Local::Reg(reg) => {
                            self.stack.push(self.regs[reg].clone());
                        }
                        Local::Stack(stack) => {
                            self.stack.push(self.stack[stack].clone());
                        }
                        _ => todo!(),
                    }
                }
                Opcode::Pop => {
                    self.stack.pop();
                }
                Opcode::Peek { to } => {
                    match to.clone() {
                        Local::Reg(reg) => {
                            self.regs[reg] = self.stack.last().unwrap().clone();
                        }
                        Local::Stack(stack) => {
                            self.stack[stack] = self.stack.last().unwrap().clone();
                        }
                        _ => todo!(),
                    }
                }
                Opcode::Peek2 { to } => {
                    match to.clone() {
                        Local ::Reg(reg) => {
                            self.regs[reg] = self.stack[self.stack.len() - 2].clone();
                        }
                        Local::Stack(stack) => {
                            self.stack[stack] = self.stack[self.stack.len() - 2].clone();
                        }
                        _ => todo!(),
                    }
                }
                Opcode::Push { from, value } => {
                    match from.clone() {
                        Local::Reg(reg) => {
                            self.regs[reg] = value.clone();
                        }
                        Local::Stack(stack) => {
                            self.stack[stack] = value.clone();
                        }
                        _ => todo!(),
                    }
                }
                Opcode::Dup => {
                    self.stack.push(self.stack.last().unwrap().clone());
                }
                Opcode::Dup2 => {
                    self.stack.push(self.stack[self.stack.len() - 2].clone());
                }
                Opcode::Swap => {
                    let last = self.stack.pop().unwrap();
                    let second_last = self.stack.pop().unwrap();
                    self.stack.push(last);
                    self.stack.push(second_last);
                }
                Opcode::Swap2 => {
                    let last = self.stack.pop().unwrap();
                    let second_last = self.stack.pop().unwrap();
                    let third_last = self.stack.pop().unwrap();
                    let fourth_last = self.stack.pop().unwrap();
                    self.stack.push(last);
                    self.stack.push(second_last);
                    self.stack.push(third_last);
                    self.stack.push(fourth_last);
                }
                Opcode::Move { from, to } => {
                    let value = self.get(&from);
                    self.set(&to, value.clone());
                }
                Opcode::Add { left, right, to } => {
                    let left = self.get(&left);
                    let right = self.get(&right);
                    let value = match (left, right) {
                        (Const::Int(left), Const::Int(right)) => Const::Int(left + right),
                        (Const::Float(left), Const::Float(right)) => Const::Float(left + right),
                        (Const::Long(left), Const::Long(right)) => Const::Long(left + right),
                        (Const::Double(left), Const::Double(right)) => Const::Double(left + right),

                        (Const::Int(left), Const::Float(right)) => Const::Float(left as f32 + right),
                        (Const::Int(left), Const::Long(right)) => Const::Long(left as i64 + right),
                        (Const::Int(left), Const::Double(right)) => Const::Double(left as f64 + right),

                        (Const::Float(left), Const::Int(right)) => Const::Float(left + right as f32),
                        (Const::Float(left), Const::Long(right)) => Const::Float(left + right as f32),
                        (Const::Float(left), Const::Double(right)) => Const::Double(left as f64 + right),

                        (Const::Long(left), Const::Int(right)) => Const::Long(left + right as i64),
                        (Const::Long(left), Const::Float(right)) => Const::Float(left as f32 + right),
                        (Const::Long(left), Const::Double(right)) => Const::Double(left as f64 + right),

                        (Const::Double(left), Const::Int(right)) => Const::Double(left + right as f64),
                        (Const::Double(left), Const::Float(right)) => Const::Double(left + right as f64),
                        (Const::Double(left), Const::Long(right)) => Const::Double(left + right as f64),

                        (Const::Byte(left), Const::Byte(right)) => Const::Byte(left + right),
                        (Const::Short(left), Const::Short(right)) => Const::Short(left + right),
                        (Const::Char(left), Const::Char(right)) => Const::Char(left + right),

                        _ => panic!("invalid types for add"),
                    };
                    self.set(&to, value);
                }
                Opcode::Sub { left, right, to } => todo!(),
                Opcode::Mul { left, right, to } => todo!(),
                Opcode::Div { left, right, to } => todo!(),
                Opcode::Mod { left, right, to } => todo!(),
                Opcode::Neg { from, to } => todo!(),
                Opcode::Inc { from, to } => todo!(),
                Opcode::Dec { from, to } => todo!(),
                Opcode::And { left, right, to } => todo!(),
                Opcode::Or { left, right, to } => todo!(),
                Opcode::Xor { left, right, to } => todo!(),
                Opcode::Not { from, to } => todo!(),
                Opcode::Shl { left, right, to } => todo!(),
                Opcode::Shr { left, right, to } => todo!(),
                Opcode::Ushr { left, right, to } => todo!(),
                Opcode::Cmp { left, right, to } => todo!(),
                Opcode::Goto { to } => todo!(),
                Opcode::IfEq { to, left, right } => todo!(),
                Opcode::IfNe { to, left, right } => todo!(),
                Opcode::IfLt { to, left, right } => todo!(),
                Opcode::IfLe { to, left, right } => todo!(),
                Opcode::IfGt { to, left, right } => todo!(),
                Opcode::IfGe { to, left, right } => todo!(),
                Opcode::IfNull { to, from } => todo!(),
                Opcode::IfNonNull { to, from } => todo!(),
                Opcode::New { to, class } => todo!(),
                Opcode::NewArray { to, class, size } => todo!(),
                Opcode::NewMultiArray { to, array_type, dimensions } => todo!(),
                Opcode::ArrayLength { to, from } => todo!(),
                Opcode::ArrayLoad { to, from, index } => todo!(),
                Opcode::ArrayStore { to, from, index } => todo!(),
                Opcode::Const { to, value } => {
                    self.set(&to, value.clone());
                }
                Opcode::Call { func, args, to } => todo!(),
                Opcode::Ret { from } => todo!(),
                Opcode::Halt => {
                    break;
                }
                _ => todo!("{:?}", op),
            }
            self.pc += 1;
        }
    }

    fn get(&mut self, local: &Local) -> Const {
        match local {
            Local::Reg(reg) => self.regs[*reg].clone(),
            Local::Stack(stack) => self.stack[*stack].clone(),
            Local::Const(constant) => constant.clone(),
            Local::Addr(address) => todo!("get address")
        }
    }

    fn set(&mut self, local: &Local, value: Const) {
        match local {
            Local::Reg(reg) => {
                if *reg >= self.regs.len() {
                    self.regs.resize(*reg + 1, Const::Int(0));
                }
                self.regs[*reg] = value;
            }
            Local::Stack(stack) => self.stack[*stack] = value,
            Local::Const(constant) => todo!("set constant"),
            Local::Addr(address) => todo!("set address")
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