#![feature(nll)]
#![feature(iterator_flatten)]
#![feature(range_contains)]

extern crate clap;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate termion;
extern crate tui;

mod cpu;
mod isa;
mod logic;
mod memory;
mod ui;

use std::error::Error;
use std::fs::File;

use clap::Arg;
use isa::*;

fn main() -> Result<(), Box<Error>> {
    let matches = clap::App::new("invade-rs")
        .version("0.1")
        .author("Pietro L. <pietro.lorefice@gmail.com>")
        .about("Old-school Space Invaders emulator")
        .arg(
            Arg::with_name("ISA")
                .help("ISA description file in JSON format")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("ROM")
                .help("ROM file")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    // Load ISA description
    let mut isa_file =
        File::open(matches.value_of("ISA").unwrap()).expect("could not open ISA file");

    let isa = ISA::load(&mut isa_file).expect("could not load ISA description");

    // Allocate memory and load rom into it
    let mut rom_file =
        File::open(matches.value_of("ROM").unwrap()).expect("could not load ROM file");

    let mut mem = memory::MemoryMap::new();
    mem.load_rom(&mut rom_file)
        .expect("could not read ROM file");

    // Create and run UI
    let cpu = cpu::CPU::new(isa, mem);
    let mut app = ui::App::new(cpu);
    app.run()
}
