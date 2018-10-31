#[derive(Debug)]
pub enum Statement {
    Assignment(Operand, Box<Expression>),
    Expression(Box<Expression>),
    Conditional(Box<Expression>, Box<Statement>),
    Special,
    Empty,
}

#[derive(Debug)]
pub enum Expression {
    Operand(Operand),
    Binary(Operand, BinaryOp, Box<Expression>),
    Unary(UnaryOp, Operand),
    Nonary(NonaryOp),
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

#[derive(Debug)]
pub enum UnaryOp {
    Not,
    Call,
}

#[derive(Debug)]
pub enum NonaryOp {
    Return,
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

#[derive(Debug)]
pub enum Register16 {
    BC,
    DE,
    HL,
}

#[derive(Debug)]
pub enum SpecialRegister {
    SP,
    PC,
    PCl,
    PCh,
    Flags,
}

#[derive(Debug)]
pub enum Flag {
    Z,
    S,
    P,
    CY,
    AC,
}

#[derive(Debug)]
pub struct MemoryLocation(pub Box<Expression>);
