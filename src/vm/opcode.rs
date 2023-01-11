#[derive(Debug, Clone)]
pub(crate) enum Opcode {
    Move { from: Local, to: Local },
    Const { to: Local, value: Const },
    Add { left: Local, right: Local, to: Local },
    Sub { left: Local, right: Local, to: Local },
    Mul { left: Local, right: Local, to: Local },
    Div { left: Local, right: Local, to: Local },
    Mod { left: Local, right: Local, to: Local },
    Neg { from: Local, to: Local },
    Not { from: Local, to: Local },
    And { left: Local, right: Local, to: Local },
    Or { left: Local, right: Local, to: Local },
    Xor { left: Local, right: Local, to: Local },
    Shl { left: Local, right: Local, to: Local },
    Shr { left: Local, right: Local, to: Local },
    Cmp { left: Local, right: Local, to: Local },
    Jmp { to: usize },
    JmpIf { cond: Local, to: usize },
    JmpIfNot { cond: Local, to: usize },
    Call { func: usize, args: Vec<Local>, to: Local },
    Ret { from: Local },
    Print { from: Local },
    Halt,
}

#[derive(Debug, Clone)]
pub(crate) enum Const {
    Int(i64),
    Float(f64),
    Str(String),
    Addr(usize),
}

#[derive(Debug, Clone)]
pub(crate) enum Local {
    Reg(usize),
    Stack(usize),
}