#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod cpu;
mod isa;
mod memory;
mod opcodes;

use isa::*;

use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let rom = load_rom(&args[2]).expect("could not load ROM file");

    // Load ISA description
    let mut isa_f = File::open(&args[1]).expect("could not open ISA file");
    let isa = ISA::load(&mut isa_f).expect("could not load ISA description");

    // Allocate memory and load rom into it
    let mut memmap = memory::MemoryMap::new();
    for (i, &b) in rom.iter().enumerate() {
        memmap.store_u8(i as u16, b)
    }

    // Create processor and start executing
    let mut cpu = cpu::CPU::new(isa, memmap);

    loop {
        cpu.step()
    }
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
