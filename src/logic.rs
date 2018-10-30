use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "../res/isa.pest"]
struct LogicParser;

#[derive(Debug)]
pub struct Logic(Vec<Statement>);

/// Parse a `str` containing a valid instruction logic and returns it.
pub fn parse_str(s: &str) -> Result<Logic, Error<Rule>> {
    let mut logic = LogicParser::parse(Rule::logic, s)?
        .next()
        .unwrap()
        .into_inner();
    let mut stmts = Vec::new();

    // A logic is nothing more than a sequence of statements
    loop {
        let pair = logic.next().unwrap();

        match pair.as_rule() {
            Rule::stmt => stmts.push(parse_stmt(pair)),
            Rule::EOI => break,
            _ => unreachable!(),
        }
    }

    println!("{:#?}", stmts);

    Ok(Logic(stmts))
}

#[derive(Debug)]
enum Statement {
    Assignment(Operand, Expression),
    Condition(Expression, Box<Statement>),
    Expression(Expression),
    Empty,
}

/// Parse a Pair as a logic statement.
/// TODO: handle errors.
fn parse_stmt(pair: Pair<Rule>) -> Statement {
    let pair = pair.into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::empty => Statement::Empty,
        Rule::assignment => {
            let mut inner = pair.into_inner();
            let op = inner.next().unwrap();
            let exp = inner.next().unwrap();

            Statement::Assignment(parse_operand(op), parse_expression(exp))
        }
        Rule::condition => unimplemented!(),
        Rule::expression => unimplemented!(),
        _ => unreachable!(),
    }
}

#[derive(Debug)]
enum Operand {
    Reg16(String),
    Reg8(String),
    Flags,
    Memory(u16),
    Immediate(u16),
    Number(u16),
}

/// Parse a Pair as an operand.
/// TODO: handle errors.
fn parse_operand(pair: Pair<Rule>) -> Operand {
    let pair = pair.into_inner().next().unwrap();

    match pair.as_rule() {
        //Rule::reg16 => Operand::Reg16(pair.into_inner().next().unwrap().as_str().to_string()),
        Rule::reg8 => Operand::Reg8(pair.into_inner().next().unwrap().as_str().to_string()),
        _ => unreachable!(),
    }
}

#[derive(Debug)]
enum Expression {
    Nonary(NonaryOp),
    Unary(UnaryOp),
    Binary(BinaryOp),
}

fn parse_expression(pair: Pair<Rule>) -> Expression {
    unreachable!()
}

#[derive(Debug)]
enum NonaryOp {
    Return,
}

fn parse_nonary_op(pair: Pair<Rule>) -> Expression {
    unreachable!()
}

#[derive(Debug)]
enum UnaryOp {
    Not,
    Call,
}

fn parse_unary_op(pair: Pair<Rule>) -> Expression {
    unreachable!()
}

#[derive(Debug)]
enum BinaryOp {
    Add,
    Sub,
    And,
    Or,
    Xor,
    Swap,
}

fn parse_binary_op(pair: Pair<Rule>) -> Expression {
    unreachable!()
}
