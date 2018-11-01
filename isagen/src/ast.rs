use std::fmt;

#[derive(Debug)]
pub enum Statement {
    Assignment(Operand, Box<Expression>),
    Expression(Box<Expression>),
    Conditional(Box<Expression>, Box<Statement>),
    Special,
    Empty,
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Assignment(op, expr) => write!(f, "{} <- {}", op, expr),
            Statement::Expression(expr) => write!(f, "{}", expr),
            Statement::Conditional(expr, stmt) => write!(f, "if {}, {}", expr, stmt),
            Statement::Special => write!(f, "special"),
            Statement::Empty => write!(f, ""),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Operand(Operand),
    Binary(Operand, BinaryOp, Box<Expression>),
    Unary(UnaryOp, Operand),
    Nonary(NonaryOp),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Operand(op) => write!(f, "{}", op),
            Expression::Binary(op, bop, expr) => write!(f, "{} {} {}", op, bop, expr),
            Expression::Unary(uop, op) => write!(f, "{}{}", uop, op),
            Expression::Nonary(nop) => write!(f, "{}", nop),
        }
    }
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    And,
    Or,
    Xor,
    Swap,
    Rlc,
    Rrc,
    Ral,
    Rar,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::And => write!(f, "&"),
            BinaryOp::Or => write!(f, "|"),
            BinaryOp::Xor => write!(f, "^"),
            BinaryOp::Swap => write!(f, "<->"),
            BinaryOp::Rlc => write!(f, "rlc"),
            BinaryOp::Rrc => write!(f, "rrc"),
            BinaryOp::Ral => write!(f, "ral"),
            BinaryOp::Rar => write!(f, "rar"),
        }
    }
}

#[derive(Debug)]
pub enum UnaryOp {
    Not,
    Call,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOp::Not => write!(f, "!"),
            UnaryOp::Call => write!(f, "CALL "),
        }
    }
}

#[derive(Debug)]
pub enum NonaryOp {
    Return,
}

impl fmt::Display for NonaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RET")
    }
}

#[derive(Debug)]
pub enum Operand {
    Reg8(Register8),
    Reg16(Register16),
    SpecialRegister(SpecialRegister),
    Memory(MemoryLocation),
    Flag(Flag),
    Immediate,
    Address,
    Location(u16),
    Constant(u16),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::Reg8(r) => write!(f, "{}", r),
            Operand::Reg16(r) => write!(f, "{}", r),
            Operand::SpecialRegister(r) => write!(f, "{}", r),
            Operand::Memory(m) => write!(f, "({})", m),
            Operand::Flag(fl) => write!(f, "{}", fl),
            Operand::Immediate => write!(f, "imm"),
            Operand::Address => write!(f, "adr"),
            Operand::Location(l) => write!(f, "${}", l),
            Operand::Constant(c) => write!(f, "{}", c),
        }
    }
}

#[derive(Debug)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    L,
    H,
}

impl fmt::Display for Register8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum Register16 {
    BC,
    DE,
    HL,
}

impl fmt::Display for Register16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum SpecialRegister {
    SP,
    PC,
    PCl,
    PCh,
    Flags,
}

impl fmt::Display for SpecialRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpecialRegister::SP | SpecialRegister::PC => write!(f, "{:?}", self),
            SpecialRegister::PCl => write!(f, "PC.lo"),
            SpecialRegister::PCh => write!(f, "PC.hi"),
            SpecialRegister::Flags => write!(f, "flags"),
        }
    }
}

#[derive(Debug)]
pub enum Flag {
    Z,
    S,
    P,
    CY,
    AC,
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct MemoryLocation(pub Box<Expression>);

impl fmt::Display for MemoryLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
