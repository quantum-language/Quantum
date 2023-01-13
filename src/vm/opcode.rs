pub type Char = u16;

#[derive(Debug, Clone)]
pub enum Opcode {
    Load { to: Local, value: Const },
    Store { from: Local },
    Pop,
    Peek { to: Local },
    Peek2 { to: Local },
    Push { from: Local, value: Const },
    Dup,
    Dup2,
    Swap,
    Swap2,
    
    Move { from: Local, to: Local },
    
    Add { left: Local, right: Local, to: Local },
    Sub { left: Local, right: Local, to: Local },
    Mul { left: Local, right: Local, to: Local },
    Div { left: Local, right: Local, to: Local },
    Mod { left: Local, right: Local, to: Local },
    Neg { from: Local, to: Local },
    Inc { from: Local, to: Local },
    Dec { from: Local, to: Local },

    And { left: Local, right: Local, to: Local },
    Or { left: Local, right: Local, to: Local },
    Xor { left: Local, right: Local, to: Local },
    Not { from: Local, to: Local },
    Shl { left: Local, right: Local, to: Local },
    Shr { left: Local, right: Local, to: Local },
    Ushr { left: Local, right: Local, to: Local },

    Cmp { left: Local, right: Local, to: Local },

    Goto { to: usize },
    IfEq { to: usize, left: Local, right: Local },
    IfNe { to: usize, left: Local, right: Local },
    IfLe { to: usize, left: Local, right: Local },
    IfGe { to: usize, left: Local, right: Local },
    IfLt { to: usize, left: Local, right: Local },
    IfGt { to: usize, left: Local, right: Local },
    IfNull { to: usize, from: Local },
    IfNonNull { to: usize, from: Local },

    New { to: Local, class: usize },
    NewArray { to: Local, class: usize, size: Local },
    NewMultiArray { to: Local, array_type: usize, dimensions: Vec<Local> },
    ArrayLength { to: Local, from: Local },
    ArrayLoad { to: Local, from: Local, index: Local },
    ArrayStore { to: Local, from: Local, index: Local },

    Const { to: Local, value: Const },
    Call { func: usize, args: Vec<Local>, to: Local },
    Ret { from: Local },
    Halt,
}

#[derive(Debug, Clone)]
pub enum Const {
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Reference(usize),
    Byte(i8),
    Char(Char),
    Short(i16),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub enum Local {
    Reg(usize),
    Stack(usize),
    Const(Const),
    Addr(usize),
}