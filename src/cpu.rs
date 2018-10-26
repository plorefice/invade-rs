use isa::{Instruction, ISA};
use memory::MemoryMap;

use std::error::Error;

/// The current state of the 8080 CPU.
pub struct CPU {
    /// A snapshot of the CPU state flags at a point in time
    pub flags: Flags,
    /// The CPU general-purpose registers
    pub regs: RegisterBank,
    /// Current stack pointer
    pub sp: u16,
    /// Current program counter
    pub pc: u16,
    /// Processor memory bus
    pub mem: MemoryMap,
    /// Set if an interrupt needs to be served
    pub intr: bool,

    /// Instruction set description
    isa: ISA,
}

/// Internal flag bits (status register) of the processor.
/// These get modified based on the result of arithmetic and logical instructions.
#[derive(Default)]
pub struct Flags {
    /// Zero: set if the result is 0
    pub Z: bool,
    /// Sign: set if the result is negative
    pub S: bool,
    /// Parity: set if the number of 1 bits in the result is even
    pub P: bool,
    /// Carry: set if the last addition overflowed or subtraction required a borrow
    pub CY: bool,
    /// Auxiliary carry: used for BCD arithmetics
    pub AC: bool,
}

impl Flags {
    pub fn update_arith(&mut self, val: u16) {
        self.Z = (val & 0xFF) == 0x00;
        self.S = (val & 0x80) != 0;
        self.P = Flags::parity(val & 0xFF);
        // TODO: implement AC (unused by Space Invaders)
    }

    pub fn update_cy(&mut self, val: u16) {
        self.CY = val > 0xFF;
    }

    pub fn update(&mut self, val: u16) {
        self.update_arith(val);
        self.update_cy(val);
    }

    pub fn parity(val: u16) -> bool {
        let mut cnt = 0;
        for i in 0..7 {
            if (val & (1 << i)) != 0 {
                cnt = cnt + 1;
            }
        }
        (cnt % 2) == 0
    }
}

/// General-purpose registers.
#[derive(Default)]
pub struct RegisterBank {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

impl RegisterBank {
    /// Get the content of the A register.
    pub fn A(&self) -> u8 {
        self.a
    }

    /// Set the content of the A register.
    pub fn set_A(&mut self, val: u8) {
        self.a = val;
    }

    /// Get the content of the B register.
    pub fn B(&self) -> u8 {
        self.b
    }

    /// Set the content of the B register.
    pub fn set_B(&mut self, val: u8) {
        self.b = val;
    }

    /// Get the content of the C register.
    pub fn C(&self) -> u8 {
        self.c
    }

    /// Set the content of the C register.
    pub fn set_C(&mut self, val: u8) {
        self.c = val;
    }

    /// Get the content of the BC register.
    pub fn BC(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    /// Set the content of the BC register.
    pub fn set_BC(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }

    /// Get the content of the D register.
    pub fn D(&self) -> u8 {
        self.d
    }

    /// Set the content of the D register.
    pub fn set_D(&mut self, val: u8) {
        self.d = val;
    }

    /// Get the content of the E register.
    pub fn E(&self) -> u8 {
        self.e
    }

    /// Set the content of the E register.
    pub fn set_E(&mut self, val: u8) {
        self.e = val;
    }

    /// Get the content of the DE register.
    pub fn DE(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    /// Set the content of the DE register.
    pub fn set_DE(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }

    /// Get the content of the H register.
    pub fn H(&self) -> u8 {
        self.h
    }

    /// Set the content of the H register.
    pub fn set_H(&mut self, val: u8) {
        self.h = val;
    }

    /// Get the content of the L register.
    pub fn L(&self) -> u8 {
        self.l
    }

    /// Set the content of the L register.
    pub fn set_L(&mut self, val: u8) {
        self.l = val;
    }

    /// Get the content of the HL register.
    pub fn HL(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    /// Set the content of the HL register.
    pub fn set_HL(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }
}

impl CPU {
    pub fn new(isa: ISA, mem: MemoryMap) -> CPU {
        CPU {
            flags: Flags::default(),
            regs: RegisterBank::default(),
            sp: 0,
            pc: 0,
            mem,
            intr: false,
            isa,
        }
    }

    pub fn disassemble(&self, pc: u16) -> Result<String, String> {
        self.isa.disassemble(pc, &self.mem)
    }

    pub fn step(&mut self) -> Result<(), Box<Error>> {
        let opcode = self.mem.load_u8(self.pc);
        let instr = self
            .isa
            .decode(opcode, self.pc, &self.mem)
            .ok_or("error decoding opcode")?;

        // TODO: find a nicer way to register opcode execution functions
        match opcode {
            0x00 => self.opc_0x00(&instr),
            0x01 => self.opc_0x01(&instr),
            0x02 => self.opc_0x02(&instr),
            0x03 => self.opc_0x03(&instr),
            0x04 => self.opc_0x04(&instr),
            0x05 => self.opc_0x05(&instr),
            0x06 => self.opc_0x06(&instr),
            0x07 => self.opc_0x07(&instr),
            0x09 => self.opc_0x09(&instr),
            0x0A => self.opc_0x0A(&instr),
            _ => self.unimplemented_instruction(),
        };

        let desc = self
            .isa
            .get_description(opcode)
            .ok_or("error decoding opcode")?;

        self.pc = self.pc + desc.size as u16;

        Ok(())
    }

    fn unimplemented_instruction(&mut self) -> ! {
        panic!("instruction not implemented yet")
    }

    fn opc_0x00(&mut self, _instr: &Instruction) {
        // What did you expect? :)
    }

    fn opc_0x01(&mut self, instr: &Instruction) {
        let imm = instr.data.unwrap();
        self.regs.set_BC(imm);
    }

    fn opc_0x02(&mut self, _instr: &Instruction) {
        self.mem.store_u8(self.regs.BC(), self.regs.A())
    }

    fn opc_0x03(&mut self, _instr: &Instruction) {
        let bc = self.regs.BC();
        self.regs.set_BC(bc + 1)
    }

    fn opc_0x04(&mut self, _instr: &Instruction) {
        let res = self.regs.B() as u16 + 1;
        self.regs.set_B(res as u8);
        self.flags.update_arith(res);
    }

    fn opc_0x05(&mut self, _instr: &Instruction) {
        let res = self.regs.B() as u16 - 1;
        self.regs.set_B(res as u8);
        self.flags.update_arith(res);
    }

    fn opc_0x06(&mut self, instr: &Instruction) {
        self.regs.set_B(instr.data.unwrap() as u8);
    }

    fn opc_0x07(&mut self, _instr: &Instruction) {
        let a = self.regs.A();
        self.regs.set_A(a.rotate_left(1));
        self.flags.update_cy((a as u16) << 1);
    }

    fn opc_0x09(&mut self, _instr: &Instruction) {
        let res = self.regs.HL() as u32 + self.regs.BC() as u32;
        self.regs.set_HL((res & 0xFFFF) as u16);
        self.flags.update_cy((res >> 8) as u16);
    }

    fn opc_0x0A(&mut self, _instr: &Instruction) {
        let bc = self.regs.BC();
        self.regs.set_A(self.mem.load_u8(bc));
    }
}
