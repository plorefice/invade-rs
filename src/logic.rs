use pest::Parser;

#[derive(Parser)]
#[grammar = "../res/isa.pest"]
pub struct LogicParser;
