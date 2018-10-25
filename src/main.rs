#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod cpu;
mod isa;
mod memory;

use isa::*;

use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let mut f = File::open(&args[1]).expect("could not open ISA file");
    let isa = ISA::load(&mut f).expect("could not load ISA description");
    let rom = load_rom(&args[2]).expect("could not load ROM file");

    disassemble(&rom, &isa).unwrap();
}

fn load_rom(fname: &str) -> Result<Vec<u8>, io::Error> {
    File::open(fname)?.bytes().collect()
}

fn disassemble(rom: &[u8], isa: &ISA) -> Result<(), String> {
    let mut i = 0;

    while i < rom.len() {
        let opcode = rom[i];
        let instr = isa
            .decode(opcode, &rom[i..])
            .ok_or("error decoding opcode")?;
        let size = instr.desc.size;

        println!(
            "{:04x}  {:<8}  {}",
            i,
            rom[i..i + size]
                .into_iter()
                .map(|&n| format!("{:02x}", n))
                .collect::<Vec<_>>()
                .join(" "),
            instr.format(false),
        );

        i = i + size;
    }

    Ok(())
}
