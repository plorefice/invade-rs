#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Deserializer};

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

type ISA = HashMap<u8, OpcodeDescription>;

#[derive(Deserialize, Debug)]
struct OpcodeDescription {
    #[serde(deserialize_with = "from_hex")]
    opcode: u8,
    name: String,
    size: usize,
    operands: Vec<OperandType>,
    flags: String,
    notes: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
enum OperandType {
    Register(String),
    Immediate,
    Address,
}

#[derive(Debug)]
struct Instruction<'a> {
    desc: &'a OpcodeDescription,
    imm: Option<u16>,
}

fn from_hex<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let s: String = Deserialize::deserialize(deserializer)?;
    u8::from_str_radix(&s[2..], 16).map_err(D::Error::custom)
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let isa = load_isa(&args[1]).unwrap();
    let rom = load_rom(&args[2]).unwrap();

    disassemble(&rom, &isa).unwrap();
}

fn load_isa(fname: &str) -> Result<ISA, Box<Error>> {
    let descs: Vec<OpcodeDescription> = serde_json::from_reader(&mut File::open(fname)?)?;
    let mut isa = ISA::new();

    for d in descs {
        isa.insert(d.opcode, d);
    }

    Ok(isa)
}

fn load_rom(fname: &str) -> Result<Vec<u8>, io::Error> {
    File::open(fname)?.bytes().collect()
}

fn disassemble(rom: &[u8], isa: &ISA) -> Result<(), String> {
    let mut i = 0;

    while i < rom.len() {
        let opcode = rom[i];
        let ilen = isa[&opcode].size;
        let instr = decode(opcode, &rom[i..], isa);

        println!(
            "{:04x}  {:<8}  {}",
            i,
            rom[i..i + ilen]
                .into_iter()
                .map(|&n| format!("{:02x}", n))
                .collect::<Vec<_>>()
                .join(" "),
            format_instruction(&instr),
        );

        i = i + isa[&opcode].size;
    }

    Ok(())
}

fn decode<'a>(opcode: u8, rom: &[u8], isa: &'a ISA) -> Instruction<'a> {
    let desc = &isa[&opcode];
    let ilen = desc.size;

    match ilen {
        1 => Instruction { desc, imm: None },
        2 => Instruction {
            desc,
            imm: Some(rom[1] as u16),
        },
        _ => Instruction {
            desc,
            imm: Some(((rom[2] as u16) << 8) | rom[1] as u16),
        },
    }
}

fn format_instruction(instr: &Instruction) -> String {
    let desc = instr.desc;

    format!(
        "{:<8}{:<16}; {}",
        &desc.name,
        &desc
            .operands
            .iter()
            .map(|op| match op {
                OperandType::Register(r) => r.clone(),
                OperandType::Immediate => format!("#0x{:04x}", instr.imm.unwrap()),
                OperandType::Address => format!("${:04x}", instr.imm.unwrap()),
            })
            .collect::<Vec<_>>()
            .join(", "),
        &desc.notes
    )
}
