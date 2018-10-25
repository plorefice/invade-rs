use std::collections::HashMap;
use std::error::Error;
use std::io::Read;

use serde::{Deserialize, Deserializer};
use serde_json;

/// Utility type to quickly access an instruction description.
pub struct ISA(HashMap<u8, OpcodeDescription>);

impl ISA {
    /// Load a JSON-formatted ISA description from a reader.
    /// This will fail if the ISA format is unexpected.
    pub fn load(rd: &mut Read) -> Result<ISA, Box<Error>> {
        let desc: Vec<OpcodeDescription> = serde_json::from_reader(rd)?;
        let mut isa = HashMap::new();

        for d in desc {
            isa.insert(d.opcode, d);
        }

        Ok(ISA(isa))
    }

    /// Return an opcode description, if the opcode is valid.
    pub fn get_description(&self, opcode: u8) -> Option<&OpcodeDescription> {
        self.0.get(&opcode)
    }

    /// Decode an opcode and obtain the related Instruction by decoding bytes from mem.
    /// mem should be big enough to decode the instruction, otherwise decoding will fail.
    pub fn decode(&self, opcode: u8, mem: &[u8]) -> Option<Instruction> {
        let desc = self.get_description(opcode)?;

        if mem.len() < desc.size {
            None
        } else {
            Some(match desc.size {
                1 => Instruction { desc, imm: None },
                2 => Instruction {
                    desc,
                    imm: Some(mem[1] as u16),
                },
                3 => Instruction {
                    desc,
                    imm: Some(((mem[2] as u16) << 8) | mem[1] as u16),
                },
                _ => unreachable!(),
            })
        }
    }
}

/// A self-contained description of an instruction, including its human-readable
/// description, opcode, size, operands and side-effects.
#[derive(Deserialize, Debug)]
pub struct OpcodeDescription {
    #[serde(deserialize_with = "from_hex")]
    pub opcode: u8,
    pub name: String,
    pub size: usize,
    pub operands: Vec<OperandType>,
    pub flags: String,
    pub notes: String,
}

/// Convert a string representing an hexadecimal integer (eg. "0x42") into
/// its numerical type (eg. the 0x42 u8). Required since JSON does not support
/// hex-encoded integer literals.
fn from_hex<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let s: String = Deserialize::deserialize(deserializer)?;
    u8::from_str_radix(&s[2..], 16).map_err(D::Error::custom)
}

/// A possible operand type for an instruction. In the 8080 architecture,
/// operands can be either registers, immediate values or addresses.
#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum OperandType {
    Register(String),
    Immediate,
    Address,
}

/// A decoded instruction, including any immediate data associated with it.
#[derive(Debug)]
pub struct Instruction<'a> {
    pub desc: &'a OpcodeDescription,
    pub imm: Option<u16>,
}

impl<'a> Instruction<'a> {
    /// Format a disassembled instruction in a human-readable format.
    pub fn format(&self, verbose: bool) -> String {
        let desc = self.desc;

        let mut formatted = format!(
            "{:<8}{:<16}",
            &desc.name,
            &desc
                .operands
                .iter()
                .map(|op| match op {
                    OperandType::Register(r) => r.clone(),
                    OperandType::Immediate => format!("#0x{:04x}", self.imm.unwrap()),
                    OperandType::Address => format!("${:04x}", self.imm.unwrap()),
                })
                .collect::<Vec<_>>()
                .join(", ")
        );

        if verbose {
            formatted.push_str("; ");
            formatted.push_str(&desc.notes);
        }

        formatted
    }
}