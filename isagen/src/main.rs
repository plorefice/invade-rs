extern crate clap;
extern crate rustfmt_nightly as rustfmt;
extern crate serde_json;
extern crate sha2;

mod ast;
mod logic;

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use clap::Arg;
use rustfmt::{Config, EmitMode, Input, Session, Verbosity};
use serde_json::Value;
use sha2::{Digest, Sha256};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(VERSION)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("ISA")
                .help("ISA description file in JSON format")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Output file to generate")
                .index(2),
        )
        .get_matches();

    // Load ISA description
    let isa_path = matches.value_of("ISA").unwrap();
    let mut isa_file = File::open(isa_path).expect("could not open ISA file");
    let isa: Value = serde_json::from_reader(&mut isa_file).unwrap();

    // Choose output sink
    let mut out = match matches.value_of("OUTPUT") {
        Some(path) => Box::new(File::create(path).unwrap()) as Box<Write>,
        None => Box::new(io::stdout()) as Box<Write>,
    };

    // Parse ISA and generate code
    let mut code = vec![
        format!(
            "// Auto-generated from \"{}\" using isagen v{}",
            isa_path, VERSION,
        )
        .as_str(),
        &hash_file(Path::new(isa_path)).unwrap(),
        "use cpu::CPU;",
        "use isa::Instruction;",
        "",
        "#[allow(non_snake_case, unused_variables, unused_parens, unused_mut, unused_assignments)]",
        "impl CPU {",
    ]
    .join("\n")
    .to_string();

    // Generate mega-match for opcode decoding
    code.push_str(&gen_execute());

    // Parse opcode logic and generate functions
    let parser = logic::LogicParser::new();
    code.push_str(
        &isa.as_array()
            .unwrap()
            .iter()
            .map(|opdesc| {
                let opcode =
                    u8::from_str_radix(&opdesc["opcode"].as_str().unwrap()[2..], 16).unwrap();
                let logic = parser.parse(opdesc["logic"].as_str().unwrap()).unwrap();

                gen_opcode_fn(opcode, &logic).unwrap()
            })
            .collect::<Vec<_>>()
            .join("\n"),
    );

    code.push_str("}\n");

    // Run rustfmt on the code
    let mut cfg = Config::default();
    cfg.set().emit_mode(EmitMode::Stdout);
    cfg.set().verbose(Verbosity::Quiet);

    let mut fmt = Session::new(cfg, Some(&mut out));
    fmt.format(Input::Text(code)).unwrap();
}

fn hash_file(path: &Path) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut sha_256 = Sha256::new();
    io::copy(&mut f, &mut sha_256)?;

    let mut hash_str = "// sha256: ".to_owned();
    for byte in sha_256.result() {
        hash_str.push_str(&format!("{:x}", byte));
    }
    Ok(hash_str)
}

fn gen_execute() -> String {
    let mut code = vec![
        "pub fn execute(&mut self, instr: &Instruction) -> u16 {".to_string(),
        "match instr.opcode {".to_string(),
    ];
    code.push(
        (0..256)
            .map(|opc| format!("0x{:02X} => self.opc_0x{:02X}(instr),", opc, opc))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    code.push("_ => unreachable!(),\n}\n}\n\n".to_string());
    code.join("\n")
}

fn gen_opcode_fn<T>(opcode: u8, logic: &Vec<T>) -> Result<String, io::Error>
where
    T: AsRef<ast::Statement>,
{
    // Function signature
    let mut s = vec![
        format!(
            "pub fn opc_0x{:02X}(&mut self, instr: &Instruction) -> u16 {{\n",
            opcode
        )
        .as_str(),
        "let mut pc = self.pc;\n",
    ]
    .join("\n");

    // Generate code for each statement
    s.push_str(
        &logic
            .into_iter()
            .map(StmtBuilder::build)
            .collect::<Vec<_>>()
            .join("\n"),
    );

    s.push_str("pc\n}\n");
    Ok(s)
}

struct StmtBuilder {
    description: String,
    var_id: u8,
    code: Vec<String>,
}

impl StmtBuilder {
    fn build<T>(stmt: T) -> String
    where
        T: AsRef<ast::Statement>,
    {
        use ast::Operand::*;
        use ast::Statement::*;

        let mut builder = StmtBuilder {
            description: format!("// Statement: \"{}\"", stmt.as_ref()),
            var_id: 0,
            code: vec!["{".to_string()],
        };

        match stmt.as_ref() {
            Assignment(op, expr) => {
                let var = builder.eval_expression(expr);

                match op {
                    Reg8(r) => builder.store_reg8(var, r),
                    Reg16(r) => builder.store_reg16(var, r),
                    Memory(loc) => builder.store_mem(var, loc),
                    SpecialRegister(sr) => builder.store_special_reg(var, sr),
                    Flag(f) => builder.store_flag(var, f),
                    _ => unreachable!(),
                }
            }
            Expression(expr) => {
                builder.eval_expression(expr);
            }
            Conditional(cond, stmt) => {
                let cond = builder.eval_expression(cond);
                builder.code.push(format!("if __{} != 0 {{", cond));
                builder.code.push(StmtBuilder::build(stmt));
                builder.code.push("}".to_string());
            }
            Special => builder.code.push("// TODO: implement me".to_string()),
            Empty => (),
        }

        builder.code.push("}".to_string());

        format!("{}\n{}", builder.description, builder.code.join("\n"))
    }

    fn eval_expression<T>(&mut self, expr: T) -> u8
    where
        T: AsRef<ast::Expression>,
    {
        use ast::Expression::*;

        let var = self.var_id;
        self.var_id = self.var_id + 1;

        let eval = match expr.as_ref() {
            Operand(op) => self.eval_operand(op),
            Binary(op, binop, expr) => {
                let expr_val = self.eval_expression(expr);
                self.eval_binary(op, binop, expr_val)
            }
            Unary(unop, op) => self.eval_unary(op, unop),
            Nonary(nop) => self.eval_nonary(nop),
        };

        self.code.push(format!("let __{} = {};", var, eval));
        var
    }

    fn eval_operand(&mut self, op: &ast::Operand) -> String {
        use ast::Operand::*;
        use ast::SpecialRegister::*;

        match op {
            Immediate | Address => String::from("instr.data.unwrap()"),
            Reg8(r) => format!("self.regs.{}() as u16", r),
            Reg16(r) => format!("self.regs.{}()", r),
            SpecialRegister(sr) => String::from(match sr {
                SP => "self.sp",
                PC => "pc",
                PCh => "(pc >> 8)",
                PCl => "(pc & 0xFF)",
                Flags => "(self.flags.get() as u16)",
            }),
            Flag(fl) => format!("self.flags.{} as u16", fl),
            Constant(c) => format!("{}", c),
            Memory(loc) => {
                let var = self.eval_expression(&loc.0);
                format!("self.mem.load_u8(__{}) as u16", var)
            }
            Location(l) => format!("{}", l),
        }
    }

    //
    // Operation generators
    //

    fn eval_binary(&mut self, lhs: &ast::Operand, op: &ast::BinaryOp, rhs: u8) -> String {
        use ast::BinaryOp::*;

        match op {
            Add | Sub | And | Or | Xor => format!("{} {} __{}", self.eval_operand(lhs), op, rhs),
            _ => format!("/* TODO: implement me! */ 0"),
        }
    }

    fn eval_unary(&mut self, rhs: &ast::Operand, op: &ast::UnaryOp) -> String {
        use ast::UnaryOp::*;

        match op {
            Not => format!("!{}", self.eval_operand(rhs)),
            Call => format!(
                "self.opc_0xCD(&Instruction {{ opcode: 0xCD, data: Some({}) }})",
                self.eval_operand(rhs)
            ),
        }
    }

    fn eval_nonary(&mut self, nop: &ast::NonaryOp) -> String {
        use ast::NonaryOp::*;

        match nop {
            Return => String::from("self.opc_0xC9(instr)"),
        }
    }

    //
    // "Store" generators
    //

    fn store_reg8(&mut self, var: u8, r: &ast::Register8) {
        self.code
            .push(format!("self.regs.set_{}(__{} as u8);", r, var))
    }

    fn store_reg16(&mut self, var: u8, r: &ast::Register16) {
        self.code
            .push(format!("self.regs.set_{}(__{} as u16);", r, var));
    }

    fn store_special_reg(&mut self, var: u8, r: &ast::SpecialRegister) {
        use ast::SpecialRegister::*;

        self.code.push(match r {
            PC => format!("pc = __{} as u16;", var),
            SP => format!("self.sp = __{} as u16;", var),
            PCh => format!("pc = (pc & 0x00FF) | ((__{} as u16) << 8);", var),
            PCl => format!("pc = (pc & 0xFF00) | ((__{} as u16) & 0xFF);", var),
            Flags => format!("self.flags.store(__{} as u8);", var),
        })
    }

    fn store_flag(&mut self, var: u8, f: &ast::Flag) {
        self.code
            .push(format!("self.flags.{} = (__{} as u16) != 0;", f, var))
    }

    fn store_mem(&mut self, var: u8, loc: &ast::MemoryLocation) {
        let addr = self.eval_expression(&loc.0);

        self.code
            .push(format!("self.mem.store_u8(__{}, __{} as u8);", addr, var))
    }
}
