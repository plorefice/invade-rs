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
    // let mut cpu = cpu::CPU::new(isa, memmap);
    // loop {
    //     cpu.step()
    // }

    disassemble(&memmap, &isa).expect("error disassembling");
}

fn load_rom(fname: &str) -> Result<Vec<u8>, io::Error> {
    File::open(fname)?.bytes().collect()
}

fn disassemble(memmap: &memory::MemoryMap, isa: &ISA) -> Result<(), String> {
    let mut pc = 0;

    // Loop over the rom size
    while pc < 0x2000 {
        let opcode = memmap.load_u8(pc);
        let instr = isa.decode(opcode, pc, memmap).ok_or("invalid opcode")?;
        let desc = isa.get_description(opcode).ok_or("invalid opcode")?;
        let size = desc.size;

        println!(
            "{:04x}  {:<8}  {}",
            pc,
            (pc..pc + size as u16)
                .collect::<Vec<_>>()
                .iter()
                .map(|&loc| format!("{:02x}", memmap.load_u8(loc)))
                .collect::<Vec<_>>()
                .join(" "),
            instr.format(isa, false).unwrap(),
        );

        pc = pc + size as u16;
    }

    Ok(())
}
