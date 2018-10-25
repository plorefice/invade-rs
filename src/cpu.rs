use isa::ISA;
use memory::MemoryMap;

/// The current state of the 8080 CPU.
struct CPU {
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

    pub fn step(&mut self) {
        let opc = self.mem.load_u8(self.pc);
        let instr = self.isa.get_description(opc).unwrap();

        // TODO: actually execute instruction

        self.pc = self.pc + instr.size as u16;
    }
}

/// Internal flag bits (status register) of the processor.
/// These get modified based on the result of arithmetic and logical instructions.
#[derive(Default)]
struct Flags {
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

/// General-purpose registers.
#[derive(Default)]
struct RegisterBank {
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
