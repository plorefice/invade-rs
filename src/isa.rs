use std::collections::HashMap;
use std::error::Error;
use std::io::Read;

use logic::{self, Logic};
use memory::MemoryMap;

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
    pub fn decode(&self, opcode: u8, pc: u16, memmap: &MemoryMap) -> Option<Instruction> {
        let description = self.get_description(opcode)?;

        let data = match description.size {
            1 => None,
            2 => Some(memmap.load_u8(pc + 1) as u16),
            3 => Some(memmap.load_u16(pc + 1)),
            _ => unreachable!(),
        };

        Some(Instruction { opcode, data })
    }

    /// Disassemble and format a single instruction at the program counter.
    pub fn disassemble(&self, pc: u16, memmap: &MemoryMap) -> Result<String, String> {
        let opcode = memmap.load_u8(pc);
        let instr = self.decode(opcode, pc, memmap).ok_or("invalid opcode")?;
        let desc = self.get_description(opcode).ok_or("invalid opcode")?;
        let size = desc.size;

        Ok(format!(
            "{:04x}  {:<8}  {}",
            pc,
            (pc..pc + size as u16)
                .collect::<Vec<_>>()
                .iter()
                .map(|&loc| format!("{:02x}", memmap.load_u8(loc)))
                .collect::<Vec<_>>()
                .join(" "),
            instr.format(self).unwrap(),
        ))
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
    #[serde(deserialize_with = "from_logic")]
    pub logic: Logic,
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

/// Convert a string representing instruction logic into its AST representation.
fn from_logic<'de, D>(deserializer: D) -> Result<Logic, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let s: String = Deserialize::deserialize(deserializer)?;
    logic::parse_str(&s).map_err(D::Error::custom)
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
pub struct Instruction {
    // General description of the decoded instruction
    pub opcode: u8,
    // Immediate data associated with the instruction, if any
    pub data: Option<u16>,
}

impl Instruction {
    /// Format a disassembled instruction in a human-readable format.
    pub fn format(&self, isa: &ISA) -> Result<String, Box<Error>> {
        let desc = isa.get_description(self.opcode).ok_or("invalid opcode")?;

        Ok(format!(
            "{:<8}{:<16}",
            &desc.name,
            &desc
                .operands
                .iter()
                .map(|op| match op {
                    OperandType::Register(r) => r.clone(),
                    OperandType::Immediate => format!("#0x{:04x}", self.data.unwrap()),
                    OperandType::Address => format!("${:04x}", self.data.unwrap()),
                })
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}
