// Auto-generated from "../res/8080-isa.json" using isagen v0.1.0
// sha256: 78bc791796f1dddbf49c824e8f6e93d1556e7cb4e0f25b92d684c7e216f36
use cpu::CPU;
use isa::Instruction;

#[allow(
    non_snake_case,
    unused_variables,
    unused_parens,
    unused_mut,
    unused_assignments
)]
impl CPU {
    pub fn execute(&mut self, instr: &Instruction) -> u16 {
        match instr.opcode {
            0x00 => self.opc_0x00(instr),
            0x01 => self.opc_0x01(instr),
            0x02 => self.opc_0x02(instr),
            0x03 => self.opc_0x03(instr),
            0x04 => self.opc_0x04(instr),
            0x05 => self.opc_0x05(instr),
            0x06 => self.opc_0x06(instr),
            0x07 => self.opc_0x07(instr),
            0x08 => self.opc_0x08(instr),
            0x09 => self.opc_0x09(instr),
            0x0A => self.opc_0x0A(instr),
            0x0B => self.opc_0x0B(instr),
            0x0C => self.opc_0x0C(instr),
            0x0D => self.opc_0x0D(instr),
            0x0E => self.opc_0x0E(instr),
            0x0F => self.opc_0x0F(instr),
            0x10 => self.opc_0x10(instr),
            0x11 => self.opc_0x11(instr),
            0x12 => self.opc_0x12(instr),
            0x13 => self.opc_0x13(instr),
            0x14 => self.opc_0x14(instr),
            0x15 => self.opc_0x15(instr),
            0x16 => self.opc_0x16(instr),
            0x17 => self.opc_0x17(instr),
            0x18 => self.opc_0x18(instr),
            0x19 => self.opc_0x19(instr),
            0x1A => self.opc_0x1A(instr),
            0x1B => self.opc_0x1B(instr),
            0x1C => self.opc_0x1C(instr),
            0x1D => self.opc_0x1D(instr),
            0x1E => self.opc_0x1E(instr),
            0x1F => self.opc_0x1F(instr),
            0x20 => self.opc_0x20(instr),
            0x21 => self.opc_0x21(instr),
            0x22 => self.opc_0x22(instr),
            0x23 => self.opc_0x23(instr),
            0x24 => self.opc_0x24(instr),
            0x25 => self.opc_0x25(instr),
            0x26 => self.opc_0x26(instr),
            0x27 => self.opc_0x27(instr),
            0x28 => self.opc_0x28(instr),
            0x29 => self.opc_0x29(instr),
            0x2A => self.opc_0x2A(instr),
            0x2B => self.opc_0x2B(instr),
            0x2C => self.opc_0x2C(instr),
            0x2D => self.opc_0x2D(instr),
            0x2E => self.opc_0x2E(instr),
            0x2F => self.opc_0x2F(instr),
            0x30 => self.opc_0x30(instr),
            0x31 => self.opc_0x31(instr),
            0x32 => self.opc_0x32(instr),
            0x33 => self.opc_0x33(instr),
            0x34 => self.opc_0x34(instr),
            0x35 => self.opc_0x35(instr),
            0x36 => self.opc_0x36(instr),
            0x37 => self.opc_0x37(instr),
            0x38 => self.opc_0x38(instr),
            0x39 => self.opc_0x39(instr),
            0x3A => self.opc_0x3A(instr),
            0x3B => self.opc_0x3B(instr),
            0x3C => self.opc_0x3C(instr),
            0x3D => self.opc_0x3D(instr),
            0x3E => self.opc_0x3E(instr),
            0x3F => self.opc_0x3F(instr),
            0x40 => self.opc_0x40(instr),
            0x41 => self.opc_0x41(instr),
            0x42 => self.opc_0x42(instr),
            0x43 => self.opc_0x43(instr),
            0x44 => self.opc_0x44(instr),
            0x45 => self.opc_0x45(instr),
            0x46 => self.opc_0x46(instr),
            0x47 => self.opc_0x47(instr),
            0x48 => self.opc_0x48(instr),
            0x49 => self.opc_0x49(instr),
            0x4A => self.opc_0x4A(instr),
            0x4B => self.opc_0x4B(instr),
            0x4C => self.opc_0x4C(instr),
            0x4D => self.opc_0x4D(instr),
            0x4E => self.opc_0x4E(instr),
            0x4F => self.opc_0x4F(instr),
            0x50 => self.opc_0x50(instr),
            0x51 => self.opc_0x51(instr),
            0x52 => self.opc_0x52(instr),
            0x53 => self.opc_0x53(instr),
            0x54 => self.opc_0x54(instr),
            0x55 => self.opc_0x55(instr),
            0x56 => self.opc_0x56(instr),
            0x57 => self.opc_0x57(instr),
            0x58 => self.opc_0x58(instr),
            0x59 => self.opc_0x59(instr),
            0x5A => self.opc_0x5A(instr),
            0x5B => self.opc_0x5B(instr),
            0x5C => self.opc_0x5C(instr),
            0x5D => self.opc_0x5D(instr),
            0x5E => self.opc_0x5E(instr),
            0x5F => self.opc_0x5F(instr),
            0x60 => self.opc_0x60(instr),
            0x61 => self.opc_0x61(instr),
            0x62 => self.opc_0x62(instr),
            0x63 => self.opc_0x63(instr),
            0x64 => self.opc_0x64(instr),
            0x65 => self.opc_0x65(instr),
            0x66 => self.opc_0x66(instr),
            0x67 => self.opc_0x67(instr),
            0x68 => self.opc_0x68(instr),
            0x69 => self.opc_0x69(instr),
            0x6A => self.opc_0x6A(instr),
            0x6B => self.opc_0x6B(instr),
            0x6C => self.opc_0x6C(instr),
            0x6D => self.opc_0x6D(instr),
            0x6E => self.opc_0x6E(instr),
            0x6F => self.opc_0x6F(instr),
            0x70 => self.opc_0x70(instr),
            0x71 => self.opc_0x71(instr),
            0x72 => self.opc_0x72(instr),
            0x73 => self.opc_0x73(instr),
            0x74 => self.opc_0x74(instr),
            0x75 => self.opc_0x75(instr),
            0x76 => self.opc_0x76(instr),
            0x77 => self.opc_0x77(instr),
            0x78 => self.opc_0x78(instr),
            0x79 => self.opc_0x79(instr),
            0x7A => self.opc_0x7A(instr),
            0x7B => self.opc_0x7B(instr),
            0x7C => self.opc_0x7C(instr),
            0x7D => self.opc_0x7D(instr),
            0x7E => self.opc_0x7E(instr),
            0x7F => self.opc_0x7F(instr),
            0x80 => self.opc_0x80(instr),
            0x81 => self.opc_0x81(instr),
            0x82 => self.opc_0x82(instr),
            0x83 => self.opc_0x83(instr),
            0x84 => self.opc_0x84(instr),
            0x85 => self.opc_0x85(instr),
            0x86 => self.opc_0x86(instr),
            0x87 => self.opc_0x87(instr),
            0x88 => self.opc_0x88(instr),
            0x89 => self.opc_0x89(instr),
            0x8A => self.opc_0x8A(instr),
            0x8B => self.opc_0x8B(instr),
            0x8C => self.opc_0x8C(instr),
            0x8D => self.opc_0x8D(instr),
            0x8E => self.opc_0x8E(instr),
            0x8F => self.opc_0x8F(instr),
            0x90 => self.opc_0x90(instr),
            0x91 => self.opc_0x91(instr),
            0x92 => self.opc_0x92(instr),
            0x93 => self.opc_0x93(instr),
            0x94 => self.opc_0x94(instr),
            0x95 => self.opc_0x95(instr),
            0x96 => self.opc_0x96(instr),
            0x97 => self.opc_0x97(instr),
            0x98 => self.opc_0x98(instr),
            0x99 => self.opc_0x99(instr),
            0x9A => self.opc_0x9A(instr),
            0x9B => self.opc_0x9B(instr),
            0x9C => self.opc_0x9C(instr),
            0x9D => self.opc_0x9D(instr),
            0x9E => self.opc_0x9E(instr),
            0x9F => self.opc_0x9F(instr),
            0xA0 => self.opc_0xA0(instr),
            0xA1 => self.opc_0xA1(instr),
            0xA2 => self.opc_0xA2(instr),
            0xA3 => self.opc_0xA3(instr),
            0xA4 => self.opc_0xA4(instr),
            0xA5 => self.opc_0xA5(instr),
            0xA6 => self.opc_0xA6(instr),
            0xA7 => self.opc_0xA7(instr),
            0xA8 => self.opc_0xA8(instr),
            0xA9 => self.opc_0xA9(instr),
            0xAA => self.opc_0xAA(instr),
            0xAB => self.opc_0xAB(instr),
            0xAC => self.opc_0xAC(instr),
            0xAD => self.opc_0xAD(instr),
            0xAE => self.opc_0xAE(instr),
            0xAF => self.opc_0xAF(instr),
            0xB0 => self.opc_0xB0(instr),
            0xB1 => self.opc_0xB1(instr),
            0xB2 => self.opc_0xB2(instr),
            0xB3 => self.opc_0xB3(instr),
            0xB4 => self.opc_0xB4(instr),
            0xB5 => self.opc_0xB5(instr),
            0xB6 => self.opc_0xB6(instr),
            0xB7 => self.opc_0xB7(instr),
            0xB8 => self.opc_0xB8(instr),
            0xB9 => self.opc_0xB9(instr),
            0xBA => self.opc_0xBA(instr),
            0xBB => self.opc_0xBB(instr),
            0xBC => self.opc_0xBC(instr),
            0xBD => self.opc_0xBD(instr),
            0xBE => self.opc_0xBE(instr),
            0xBF => self.opc_0xBF(instr),
            0xC0 => self.opc_0xC0(instr),
            0xC1 => self.opc_0xC1(instr),
            0xC2 => self.opc_0xC2(instr),
            0xC3 => self.opc_0xC3(instr),
            0xC4 => self.opc_0xC4(instr),
            0xC5 => self.opc_0xC5(instr),
            0xC6 => self.opc_0xC6(instr),
            0xC7 => self.opc_0xC7(instr),
            0xC8 => self.opc_0xC8(instr),
            0xC9 => self.opc_0xC9(instr),
            0xCA => self.opc_0xCA(instr),
            0xCB => self.opc_0xCB(instr),
            0xCC => self.opc_0xCC(instr),
            0xCD => self.opc_0xCD(instr),
            0xCE => self.opc_0xCE(instr),
            0xCF => self.opc_0xCF(instr),
            0xD0 => self.opc_0xD0(instr),
            0xD1 => self.opc_0xD1(instr),
            0xD2 => self.opc_0xD2(instr),
            0xD3 => self.opc_0xD3(instr),
            0xD4 => self.opc_0xD4(instr),
            0xD5 => self.opc_0xD5(instr),
            0xD6 => self.opc_0xD6(instr),
            0xD7 => self.opc_0xD7(instr),
            0xD8 => self.opc_0xD8(instr),
            0xD9 => self.opc_0xD9(instr),
            0xDA => self.opc_0xDA(instr),
            0xDB => self.opc_0xDB(instr),
            0xDC => self.opc_0xDC(instr),
            0xDD => self.opc_0xDD(instr),
            0xDE => self.opc_0xDE(instr),
            0xDF => self.opc_0xDF(instr),
            0xE0 => self.opc_0xE0(instr),
            0xE1 => self.opc_0xE1(instr),
            0xE2 => self.opc_0xE2(instr),
            0xE3 => self.opc_0xE3(instr),
            0xE4 => self.opc_0xE4(instr),
            0xE5 => self.opc_0xE5(instr),
            0xE6 => self.opc_0xE6(instr),
            0xE7 => self.opc_0xE7(instr),
            0xE8 => self.opc_0xE8(instr),
            0xE9 => self.opc_0xE9(instr),
            0xEA => self.opc_0xEA(instr),
            0xEB => self.opc_0xEB(instr),
            0xEC => self.opc_0xEC(instr),
            0xED => self.opc_0xED(instr),
            0xEE => self.opc_0xEE(instr),
            0xEF => self.opc_0xEF(instr),
            0xF0 => self.opc_0xF0(instr),
            0xF1 => self.opc_0xF1(instr),
            0xF2 => self.opc_0xF2(instr),
            0xF3 => self.opc_0xF3(instr),
            0xF4 => self.opc_0xF4(instr),
            0xF5 => self.opc_0xF5(instr),
            0xF6 => self.opc_0xF6(instr),
            0xF7 => self.opc_0xF7(instr),
            0xF8 => self.opc_0xF8(instr),
            0xF9 => self.opc_0xF9(instr),
            0xFA => self.opc_0xFA(instr),
            0xFB => self.opc_0xFB(instr),
            0xFC => self.opc_0xFC(instr),
            0xFD => self.opc_0xFD(instr),
            0xFE => self.opc_0xFE(instr),
            0xFF => self.opc_0xFF(instr),
            _ => unreachable!(),
        }
    }

    pub fn opc_0x00(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        pc
    }

    pub fn opc_0x01(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "BC <- imm"
        {
            let __0 = instr.data.unwrap();
            self.regs.set_BC(__0 as u16);
        }
        pc
    }

    pub fn opc_0x02(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "(BC) <- A"
        {
            let __0 = self.regs.A() as u16;
            let __1 = self.regs.BC();
            self.mem.store_u8(__1, __0 as u8);
        }
        pc
    }

    pub fn opc_0x03(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "BC <- BC + 1"
        {
            let __1 = 1;
            let __0 = self.regs.BC() + __1;
            self.regs.set_BC(__0 as u16);
        }
        pc
    }

    pub fn opc_0x04(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "B <- B + 1"
        {
            let __1 = 1;
            let __0 = self.regs.B() as u16 + __1;
            self.regs.set_B(__0 as u8);
        }
        pc
    }

    pub fn opc_0x05(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "B <- B - 1"
        {
            let __1 = 1;
            let __0 = self.regs.B() as u16 - __1;
            self.regs.set_B(__0 as u8);
        }
        pc
    }

    pub fn opc_0x06(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "B <- imm"
        {
            let __0 = instr.data.unwrap();
            self.regs.set_B(__0 as u8);
        }
        pc
    }

    pub fn opc_0x07(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A rlc 1"
        {
            let __1 = 1;
            let __0 = /* TODO: implement me! */ 0;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x08(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        pc
    }

    pub fn opc_0x09(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "HL <- HL + BC"
        {
            let __1 = self.regs.BC();
            let __0 = self.regs.HL() + __1;
            self.regs.set_HL(__0 as u16);
        }
        pc
    }

    pub fn opc_0x0A(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- (BC)"
        {
            let __1 = self.regs.BC();
            let __0 = self.mem.load_u8(__1) as u16;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x0B(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "BC <- BC - 1"
        {
            let __1 = 1;
            let __0 = self.regs.BC() - __1;
            self.regs.set_BC(__0 as u16);
        }
        pc
    }

    pub fn opc_0x0C(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "C <- C + 1"
        {
            let __1 = 1;
            let __0 = self.regs.C() as u16 + __1;
            self.regs.set_C(__0 as u8);
        }
        pc
    }

    pub fn opc_0x0D(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "C <- C - 1"
        {
            let __1 = 1;
            let __0 = self.regs.C() as u16 - __1;
            self.regs.set_C(__0 as u8);
        }
        pc
    }

    pub fn opc_0x0E(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "C <- imm"
        {
            let __0 = instr.data.unwrap();
            self.regs.set_C(__0 as u8);
        }
        pc
    }

    pub fn opc_0x0F(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A rrc 1"
        {
            let __1 = 1;
            let __0 = /* TODO: implement me! */ 0;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x10(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        pc
    }

    pub fn opc_0x11(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "DE <- imm"
        {
            let __0 = instr.data.unwrap();
            self.regs.set_DE(__0 as u16);
        }
        pc
    }

    pub fn opc_0x12(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "(DE) <- A"
        {
            let __0 = self.regs.A() as u16;
            let __1 = self.regs.DE();
            self.mem.store_u8(__1, __0 as u8);
        }
        pc
    }

    pub fn opc_0x13(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "DE <- DE + 1"
        {
            let __1 = 1;
            let __0 = self.regs.DE() + __1;
            self.regs.set_DE(__0 as u16);
        }
        pc
    }

    pub fn opc_0x14(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "D <- D + 1"
        {
            let __1 = 1;
            let __0 = self.regs.D() as u16 + __1;
            self.regs.set_D(__0 as u8);
        }
        pc
    }

    pub fn opc_0x15(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "D <- D - 1"
        {
            let __1 = 1;
            let __0 = self.regs.D() as u16 - __1;
            self.regs.set_D(__0 as u8);
        }
        pc
    }

    pub fn opc_0x16(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "D <- imm"
        {
            let __0 = instr.data.unwrap();
            self.regs.set_D(__0 as u8);
        }
        pc
    }

    pub fn opc_0x17(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A ral 1"
        {
            let __1 = 1;
            let __0 = /* TODO: implement me! */ 0;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x18(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        pc
    }

    pub fn opc_0x19(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "HL <- HL + DE"
        {
            let __1 = self.regs.DE();
            let __0 = self.regs.HL() + __1;
            self.regs.set_HL(__0 as u16);
        }
        pc
    }

    pub fn opc_0x1A(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- (DE)"
        {
            let __1 = self.regs.DE();
            let __0 = self.mem.load_u8(__1) as u16;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x1B(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "DE <- DE - 1"
        {
            let __1 = 1;
            let __0 = self.regs.DE() - __1;
            self.regs.set_DE(__0 as u16);
        }
        pc
    }

    pub fn opc_0x1C(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "E <- E + 1"
        {
            let __1 = 1;
            let __0 = self.regs.E() as u16 + __1;
            self.regs.set_E(__0 as u8);
        }
        pc
    }

    pub fn opc_0x1D(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "E <- E - 1"
        {
            let __1 = 1;
            let __0 = self.regs.E() as u16 - __1;
            self.regs.set_E(__0 as u8);
        }
        pc
    }

    pub fn opc_0x1E(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "E <- imm"
        {
            let __0 = instr.data.unwrap();
            self.regs.set_E(__0 as u8);
        }
        pc
    }

    pub fn opc_0x1F(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A rar 1"
        {
            let __1 = 1;
            let __0 = /* TODO: implement me! */ 0;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x20(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "special"
        {
            // TODO: implement me
        }
        pc
    }

    pub fn opc_0x21(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "HL <- imm"
        {
            let __0 = instr.data.unwrap();
            self.regs.set_HL(__0 as u16);
        }
        pc
    }

    pub fn opc_0x22(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "(adr) <- L"
        {
            let __0 = self.regs.L() as u16;
            let __1 = instr.data.unwrap();
            self.mem.store_u8(__1, __0 as u8);
        }
        // Statement: "(adr + 1) <- H"
        {
            let __0 = self.regs.H() as u16;
            let __2 = 1;
            let __1 = instr.data.unwrap() + __2;
            self.mem.store_u8(__1, __0 as u8);
        }
        pc
    }

    pub fn opc_0x23(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "HL <- HL + 1"
        {
            let __1 = 1;
            let __0 = self.regs.HL() + __1;
            self.regs.set_HL(__0 as u16);
        }
        pc
    }

    pub fn opc_0x24(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "H <- H + 1"
        {
            let __1 = 1;
            let __0 = self.regs.H() as u16 + __1;
            self.regs.set_H(__0 as u8);
        }
        pc
    }

    pub fn opc_0x25(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "H <- H - 1"
        {
            let __1 = 1;
            let __0 = self.regs.H() as u16 - __1;
            self.regs.set_H(__0 as u8);
        }
        pc
    }

    pub fn opc_0x26(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "H <- imm"
        {
            let __0 = instr.data.unwrap();
            self.regs.set_H(__0 as u8);
        }
        pc
    }

    pub fn opc_0x27(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "special"
        {
            // TODO: implement me
        }
        pc
    }

    pub fn opc_0x28(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        pc
    }

    pub fn opc_0x29(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "HL <- HL + HL"
        {
            let __1 = self.regs.HL();
            let __0 = self.regs.HL() + __1;
            self.regs.set_HL(__0 as u16);
        }
        pc
    }

    pub fn opc_0x2A(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "L <- (adr)"
        {
            let __1 = instr.data.unwrap();
            let __0 = self.mem.load_u8(__1) as u16;
            self.regs.set_L(__0 as u8);
        }
        // Statement: "H <- (adr + 1)"
        {
            let __2 = 1;
            let __1 = instr.data.unwrap() + __2;
            let __0 = self.mem.load_u8(__1) as u16;
            self.regs.set_H(__0 as u8);
        }
        pc
    }

    pub fn opc_0x2B(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "HL <- HL - 1"
        {
            let __1 = 1;
            let __0 = self.regs.HL() - __1;
            self.regs.set_HL(__0 as u16);
        }
        pc
    }

    pub fn opc_0x2C(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "L <- L + 1"
        {
            let __1 = 1;
            let __0 = self.regs.L() as u16 + __1;
            self.regs.set_L(__0 as u8);
        }
        pc
    }

    pub fn opc_0x2D(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "L <- L - 1"
        {
            let __1 = 1;
            let __0 = self.regs.L() as u16 - __1;
            self.regs.set_L(__0 as u8);
        }
        pc
    }

    pub fn opc_0x2E(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "L <- imm"
        {
            let __0 = instr.data.unwrap();
            self.regs.set_L(__0 as u8);
        }
        pc
    }

    pub fn opc_0x2F(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- !A"
        {
            let __0 = !self.regs.A() as u16;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x30(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "special"
        {
            // TODO: implement me
        }
        pc
    }

    pub fn opc_0x31(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "SP <- imm"
        {
            let __0 = instr.data.unwrap();
            self.sp = __0 as u16;
        }
        pc
    }

    pub fn opc_0x32(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "(adr) <- A"
        {
            let __0 = self.regs.A() as u16;
            let __1 = instr.data.unwrap();
            self.mem.store_u8(__1, __0 as u8);
        }
        pc
    }

    pub fn opc_0x33(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "SP <- SP + 1"
        {
            let __1 = 1;
            let __0 = self.sp + __1;
            self.sp = __0 as u16;
        }
        pc
    }

    pub fn opc_0x34(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "(HL) <- (HL) + 1"
        {
            let __1 = 1;
            let __2 = self.regs.HL();
            let __0 = self.mem.load_u8(__2) as u16 + __1;
            let __3 = self.regs.HL();
            self.mem.store_u8(__3, __0 as u8);
        }
        pc
    }

    pub fn opc_0x35(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "(HL) <- (HL) - 1"
        {
            let __1 = 1;
            let __2 = self.regs.HL();
            let __0 = self.mem.load_u8(__2) as u16 - __1;
            let __3 = self.regs.HL();
            self.mem.store_u8(__3, __0 as u8);
        }
        pc
    }

    pub fn opc_0x36(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "(HL) <- imm"
        {
            let __0 = instr.data.unwrap();
            let __1 = self.regs.HL();
            self.mem.store_u8(__1, __0 as u8);
        }
        pc
    }

    pub fn opc_0x37(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "CY <- 1"
        {
            let __0 = 1;
            self.flags.CY = (__0 as u16) != 0;
        }
        pc
    }

    pub fn opc_0x38(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        pc
    }

    pub fn opc_0x39(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "HL <- HL + SP"
        {
            let __1 = self.sp;
            let __0 = self.regs.HL() + __1;
            self.regs.set_HL(__0 as u16);
        }
        pc
    }

    pub fn opc_0x3A(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- (adr)"
        {
            let __1 = instr.data.unwrap();
            let __0 = self.mem.load_u8(__1) as u16;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x3B(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "SP <- SP - 1"
        {
            let __1 = 1;
            let __0 = self.sp - __1;
            self.sp = __0 as u16;
        }
        pc
    }

    pub fn opc_0x3C(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A + 1"
        {
            let __1 = 1;
            let __0 = self.regs.A() as u16 + __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x3D(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A - 1"
        {
            let __1 = 1;
            let __0 = self.regs.A() as u16 - __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x3E(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- imm"
        {
            let __0 = instr.data.unwrap();
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x3F(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "CY <- !CY"
        {
            let __0 = !self.flags.CY as u16;
            self.flags.CY = (__0 as u16) != 0;
        }
        pc
    }

    pub fn opc_0x40(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "B <- B"
        {
            let __0 = self.regs.B() as u16;
            self.regs.set_B(__0 as u8);
        }
        pc
    }

    pub fn opc_0x41(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "B <- C"
        {
            let __0 = self.regs.C() as u16;
            self.regs.set_B(__0 as u8);
        }
        pc
    }

    pub fn opc_0x42(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "B <- D"
        {
            let __0 = self.regs.D() as u16;
            self.regs.set_B(__0 as u8);
        }
        pc
    }

    pub fn opc_0x43(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "B <- E"
        {
            let __0 = self.regs.E() as u16;
            self.regs.set_B(__0 as u8);
        }
        pc
    }

    pub fn opc_0x44(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "B <- H"
        {
            let __0 = self.regs.H() as u16;
            self.regs.set_B(__0 as u8);
        }
        pc
    }

    pub fn opc_0x45(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "B <- L"
        {
            let __0 = self.regs.L() as u16;
            self.regs.set_B(__0 as u8);
        }
        pc
    }

    pub fn opc_0x46(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "B <- (HL)"
        {
            let __1 = self.regs.HL();
            let __0 = self.mem.load_u8(__1) as u16;
            self.regs.set_B(__0 as u8);
        }
        pc
    }

    pub fn opc_0x47(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "B <- A"
        {
            let __0 = self.regs.A() as u16;
            self.regs.set_B(__0 as u8);
        }
        pc
    }

    pub fn opc_0x48(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "C <- B"
        {
            let __0 = self.regs.B() as u16;
            self.regs.set_C(__0 as u8);
        }
        pc
    }

    pub fn opc_0x49(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "C <- C"
        {
            let __0 = self.regs.C() as u16;
            self.regs.set_C(__0 as u8);
        }
        pc
    }

    pub fn opc_0x4A(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "C <- D"
        {
            let __0 = self.regs.D() as u16;
            self.regs.set_C(__0 as u8);
        }
        pc
    }

    pub fn opc_0x4B(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "C <- E"
        {
            let __0 = self.regs.E() as u16;
            self.regs.set_C(__0 as u8);
        }
        pc
    }

    pub fn opc_0x4C(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "C <- H"
        {
            let __0 = self.regs.H() as u16;
            self.regs.set_C(__0 as u8);
        }
        pc
    }

    pub fn opc_0x4D(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "C <- L"
        {
            let __0 = self.regs.L() as u16;
            self.regs.set_C(__0 as u8);
        }
        pc
    }

    pub fn opc_0x4E(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "C <- (HL)"
        {
            let __1 = self.regs.HL();
            let __0 = self.mem.load_u8(__1) as u16;
            self.regs.set_C(__0 as u8);
        }
        pc
    }

    pub fn opc_0x4F(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "C <- A"
        {
            let __0 = self.regs.A() as u16;
            self.regs.set_C(__0 as u8);
        }
        pc
    }

    pub fn opc_0x50(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "D <- B"
        {
            let __0 = self.regs.B() as u16;
            self.regs.set_D(__0 as u8);
        }
        pc
    }

    pub fn opc_0x51(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "D <- C"
        {
            let __0 = self.regs.C() as u16;
            self.regs.set_D(__0 as u8);
        }
        pc
    }

    pub fn opc_0x52(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "D <- D"
        {
            let __0 = self.regs.D() as u16;
            self.regs.set_D(__0 as u8);
        }
        pc
    }

    pub fn opc_0x53(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "D <- E"
        {
            let __0 = self.regs.E() as u16;
            self.regs.set_D(__0 as u8);
        }
        pc
    }

    pub fn opc_0x54(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "D <- H"
        {
            let __0 = self.regs.H() as u16;
            self.regs.set_D(__0 as u8);
        }
        pc
    }

    pub fn opc_0x55(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "D <- L"
        {
            let __0 = self.regs.L() as u16;
            self.regs.set_D(__0 as u8);
        }
        pc
    }

    pub fn opc_0x56(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "D <- (HL)"
        {
            let __1 = self.regs.HL();
            let __0 = self.mem.load_u8(__1) as u16;
            self.regs.set_D(__0 as u8);
        }
        pc
    }

    pub fn opc_0x57(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "D <- A"
        {
            let __0 = self.regs.A() as u16;
            self.regs.set_D(__0 as u8);
        }
        pc
    }

    pub fn opc_0x58(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "E <- B"
        {
            let __0 = self.regs.B() as u16;
            self.regs.set_E(__0 as u8);
        }
        pc
    }

    pub fn opc_0x59(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "E <- C"
        {
            let __0 = self.regs.C() as u16;
            self.regs.set_E(__0 as u8);
        }
        pc
    }

    pub fn opc_0x5A(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "E <- D"
        {
            let __0 = self.regs.D() as u16;
            self.regs.set_E(__0 as u8);
        }
        pc
    }

    pub fn opc_0x5B(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "E <- E"
        {
            let __0 = self.regs.E() as u16;
            self.regs.set_E(__0 as u8);
        }
        pc
    }

    pub fn opc_0x5C(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "E <- H"
        {
            let __0 = self.regs.H() as u16;
            self.regs.set_E(__0 as u8);
        }
        pc
    }

    pub fn opc_0x5D(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "E <- L"
        {
            let __0 = self.regs.L() as u16;
            self.regs.set_E(__0 as u8);
        }
        pc
    }

    pub fn opc_0x5E(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "E <- (HL)"
        {
            let __1 = self.regs.HL();
            let __0 = self.mem.load_u8(__1) as u16;
            self.regs.set_E(__0 as u8);
        }
        pc
    }

    pub fn opc_0x5F(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "E <- A"
        {
            let __0 = self.regs.A() as u16;
            self.regs.set_E(__0 as u8);
        }
        pc
    }

    pub fn opc_0x60(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "H <- B"
        {
            let __0 = self.regs.B() as u16;
            self.regs.set_H(__0 as u8);
        }
        pc
    }

    pub fn opc_0x61(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "H <- C"
        {
            let __0 = self.regs.C() as u16;
            self.regs.set_H(__0 as u8);
        }
        pc
    }

    pub fn opc_0x62(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "H <- D"
        {
            let __0 = self.regs.D() as u16;
            self.regs.set_H(__0 as u8);
        }
        pc
    }

    pub fn opc_0x63(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "H <- E"
        {
            let __0 = self.regs.E() as u16;
            self.regs.set_H(__0 as u8);
        }
        pc
    }

    pub fn opc_0x64(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "H <- H"
        {
            let __0 = self.regs.H() as u16;
            self.regs.set_H(__0 as u8);
        }
        pc
    }

    pub fn opc_0x65(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "H <- L"
        {
            let __0 = self.regs.L() as u16;
            self.regs.set_H(__0 as u8);
        }
        pc
    }

    pub fn opc_0x66(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "H <- (HL)"
        {
            let __1 = self.regs.HL();
            let __0 = self.mem.load_u8(__1) as u16;
            self.regs.set_H(__0 as u8);
        }
        pc
    }

    pub fn opc_0x67(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "H <- A"
        {
            let __0 = self.regs.A() as u16;
            self.regs.set_H(__0 as u8);
        }
        pc
    }

    pub fn opc_0x68(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "L <- B"
        {
            let __0 = self.regs.B() as u16;
            self.regs.set_L(__0 as u8);
        }
        pc
    }

    pub fn opc_0x69(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "L <- C"
        {
            let __0 = self.regs.C() as u16;
            self.regs.set_L(__0 as u8);
        }
        pc
    }

    pub fn opc_0x6A(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "L <- D"
        {
            let __0 = self.regs.D() as u16;
            self.regs.set_L(__0 as u8);
        }
        pc
    }

    pub fn opc_0x6B(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "L <- E"
        {
            let __0 = self.regs.E() as u16;
            self.regs.set_L(__0 as u8);
        }
        pc
    }

    pub fn opc_0x6C(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "L <- H"
        {
            let __0 = self.regs.H() as u16;
            self.regs.set_L(__0 as u8);
        }
        pc
    }

    pub fn opc_0x6D(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "L <- L"
        {
            let __0 = self.regs.L() as u16;
            self.regs.set_L(__0 as u8);
        }
        pc
    }

    pub fn opc_0x6E(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "L <- (HL)"
        {
            let __1 = self.regs.HL();
            let __0 = self.mem.load_u8(__1) as u16;
            self.regs.set_L(__0 as u8);
        }
        pc
    }

    pub fn opc_0x6F(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "L <- A"
        {
            let __0 = self.regs.A() as u16;
            self.regs.set_L(__0 as u8);
        }
        pc
    }

    pub fn opc_0x70(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "(HL) <- B"
        {
            let __0 = self.regs.B() as u16;
            let __1 = self.regs.HL();
            self.mem.store_u8(__1, __0 as u8);
        }
        pc
    }

    pub fn opc_0x71(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "(HL) <- C"
        {
            let __0 = self.regs.C() as u16;
            let __1 = self.regs.HL();
            self.mem.store_u8(__1, __0 as u8);
        }
        pc
    }

    pub fn opc_0x72(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "(HL) <- D"
        {
            let __0 = self.regs.D() as u16;
            let __1 = self.regs.HL();
            self.mem.store_u8(__1, __0 as u8);
        }
        pc
    }

    pub fn opc_0x73(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "(HL) <- E"
        {
            let __0 = self.regs.E() as u16;
            let __1 = self.regs.HL();
            self.mem.store_u8(__1, __0 as u8);
        }
        pc
    }

    pub fn opc_0x74(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "(HL) <- H"
        {
            let __0 = self.regs.H() as u16;
            let __1 = self.regs.HL();
            self.mem.store_u8(__1, __0 as u8);
        }
        pc
    }

    pub fn opc_0x75(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "(HL) <- L"
        {
            let __0 = self.regs.L() as u16;
            let __1 = self.regs.HL();
            self.mem.store_u8(__1, __0 as u8);
        }
        pc
    }

    pub fn opc_0x76(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "special"
        {
            // TODO: implement me
        }
        pc
    }

    pub fn opc_0x77(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "(HL) <- A"
        {
            let __0 = self.regs.A() as u16;
            let __1 = self.regs.HL();
            self.mem.store_u8(__1, __0 as u8);
        }
        pc
    }

    pub fn opc_0x78(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- B"
        {
            let __0 = self.regs.B() as u16;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x79(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- C"
        {
            let __0 = self.regs.C() as u16;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x7A(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- D"
        {
            let __0 = self.regs.D() as u16;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x7B(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- E"
        {
            let __0 = self.regs.E() as u16;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x7C(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- H"
        {
            let __0 = self.regs.H() as u16;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x7D(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- L"
        {
            let __0 = self.regs.L() as u16;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x7E(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- (HL)"
        {
            let __1 = self.regs.HL();
            let __0 = self.mem.load_u8(__1) as u16;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x7F(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A"
        {
            let __0 = self.regs.A() as u16;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x80(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A + B"
        {
            let __1 = self.regs.B() as u16;
            let __0 = self.regs.A() as u16 + __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x81(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A + C"
        {
            let __1 = self.regs.C() as u16;
            let __0 = self.regs.A() as u16 + __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x82(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A + D"
        {
            let __1 = self.regs.D() as u16;
            let __0 = self.regs.A() as u16 + __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x83(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A + E"
        {
            let __1 = self.regs.E() as u16;
            let __0 = self.regs.A() as u16 + __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x84(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A + H"
        {
            let __1 = self.regs.H() as u16;
            let __0 = self.regs.A() as u16 + __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x85(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A + L"
        {
            let __1 = self.regs.L() as u16;
            let __0 = self.regs.A() as u16 + __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x86(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A + (HL)"
        {
            let __2 = self.regs.HL();
            let __1 = self.mem.load_u8(__2) as u16;
            let __0 = self.regs.A() as u16 + __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x87(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A + A"
        {
            let __1 = self.regs.A() as u16;
            let __0 = self.regs.A() as u16 + __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x88(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A + B + CY"
        {
            let __2 = self.flags.CY as u16;
            let __1 = self.regs.B() as u16 + __2;
            let __0 = self.regs.A() as u16 + __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x89(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A + C + CY"
        {
            let __2 = self.flags.CY as u16;
            let __1 = self.regs.C() as u16 + __2;
            let __0 = self.regs.A() as u16 + __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x8A(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A + D + CY"
        {
            let __2 = self.flags.CY as u16;
            let __1 = self.regs.D() as u16 + __2;
            let __0 = self.regs.A() as u16 + __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x8B(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A + E + CY"
        {
            let __2 = self.flags.CY as u16;
            let __1 = self.regs.E() as u16 + __2;
            let __0 = self.regs.A() as u16 + __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x8C(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A + H + CY"
        {
            let __2 = self.flags.CY as u16;
            let __1 = self.regs.H() as u16 + __2;
            let __0 = self.regs.A() as u16 + __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x8D(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A + L + CY"
        {
            let __2 = self.flags.CY as u16;
            let __1 = self.regs.L() as u16 + __2;
            let __0 = self.regs.A() as u16 + __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x8E(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A + (HL) + CY"
        {
            let __2 = self.flags.CY as u16;
            let __3 = self.regs.HL();
            let __1 = self.mem.load_u8(__3) as u16 + __2;
            let __0 = self.regs.A() as u16 + __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x8F(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A + A + CY"
        {
            let __2 = self.flags.CY as u16;
            let __1 = self.regs.A() as u16 + __2;
            let __0 = self.regs.A() as u16 + __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x90(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A - B"
        {
            let __1 = self.regs.B() as u16;
            let __0 = self.regs.A() as u16 - __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x91(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A - C"
        {
            let __1 = self.regs.C() as u16;
            let __0 = self.regs.A() as u16 - __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x92(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A - D"
        {
            let __1 = self.regs.D() as u16;
            let __0 = self.regs.A() as u16 - __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x93(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A - E"
        {
            let __1 = self.regs.E() as u16;
            let __0 = self.regs.A() as u16 - __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x94(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A - H"
        {
            let __1 = self.regs.H() as u16;
            let __0 = self.regs.A() as u16 - __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x95(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A - L"
        {
            let __1 = self.regs.L() as u16;
            let __0 = self.regs.A() as u16 - __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x96(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A - (HL)"
        {
            let __2 = self.regs.HL();
            let __1 = self.mem.load_u8(__2) as u16;
            let __0 = self.regs.A() as u16 - __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x97(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A - A"
        {
            let __1 = self.regs.A() as u16;
            let __0 = self.regs.A() as u16 - __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x98(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A - B - CY"
        {
            let __2 = self.flags.CY as u16;
            let __1 = self.regs.B() as u16 - __2;
            let __0 = self.regs.A() as u16 - __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x99(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A - C - CY"
        {
            let __2 = self.flags.CY as u16;
            let __1 = self.regs.C() as u16 - __2;
            let __0 = self.regs.A() as u16 - __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x9A(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A - D - CY"
        {
            let __2 = self.flags.CY as u16;
            let __1 = self.regs.D() as u16 - __2;
            let __0 = self.regs.A() as u16 - __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x9B(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A - E - CY"
        {
            let __2 = self.flags.CY as u16;
            let __1 = self.regs.E() as u16 - __2;
            let __0 = self.regs.A() as u16 - __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x9C(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A - H - CY"
        {
            let __2 = self.flags.CY as u16;
            let __1 = self.regs.H() as u16 - __2;
            let __0 = self.regs.A() as u16 - __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x9D(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A - L - CY"
        {
            let __2 = self.flags.CY as u16;
            let __1 = self.regs.L() as u16 - __2;
            let __0 = self.regs.A() as u16 - __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x9E(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A - (HL) - CY"
        {
            let __2 = self.flags.CY as u16;
            let __3 = self.regs.HL();
            let __1 = self.mem.load_u8(__3) as u16 - __2;
            let __0 = self.regs.A() as u16 - __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0x9F(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A - A - CY"
        {
            let __2 = self.flags.CY as u16;
            let __1 = self.regs.A() as u16 - __2;
            let __0 = self.regs.A() as u16 - __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xA0(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A & B"
        {
            let __1 = self.regs.B() as u16;
            let __0 = self.regs.A() as u16 & __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xA1(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A & C"
        {
            let __1 = self.regs.C() as u16;
            let __0 = self.regs.A() as u16 & __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xA2(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A & D"
        {
            let __1 = self.regs.D() as u16;
            let __0 = self.regs.A() as u16 & __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xA3(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A & E"
        {
            let __1 = self.regs.E() as u16;
            let __0 = self.regs.A() as u16 & __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xA4(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A & H"
        {
            let __1 = self.regs.H() as u16;
            let __0 = self.regs.A() as u16 & __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xA5(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A & L"
        {
            let __1 = self.regs.L() as u16;
            let __0 = self.regs.A() as u16 & __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xA6(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A & (HL)"
        {
            let __2 = self.regs.HL();
            let __1 = self.mem.load_u8(__2) as u16;
            let __0 = self.regs.A() as u16 & __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xA7(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A & A"
        {
            let __1 = self.regs.A() as u16;
            let __0 = self.regs.A() as u16 & __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xA8(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A ^ B"
        {
            let __1 = self.regs.B() as u16;
            let __0 = self.regs.A() as u16 ^ __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xA9(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A ^ C"
        {
            let __1 = self.regs.C() as u16;
            let __0 = self.regs.A() as u16 ^ __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xAA(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A ^ D"
        {
            let __1 = self.regs.D() as u16;
            let __0 = self.regs.A() as u16 ^ __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xAB(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A ^ E"
        {
            let __1 = self.regs.E() as u16;
            let __0 = self.regs.A() as u16 ^ __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xAC(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A ^ H"
        {
            let __1 = self.regs.H() as u16;
            let __0 = self.regs.A() as u16 ^ __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xAD(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A ^ L"
        {
            let __1 = self.regs.L() as u16;
            let __0 = self.regs.A() as u16 ^ __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xAE(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A ^ (HL)"
        {
            let __2 = self.regs.HL();
            let __1 = self.mem.load_u8(__2) as u16;
            let __0 = self.regs.A() as u16 ^ __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xAF(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A ^ A"
        {
            let __1 = self.regs.A() as u16;
            let __0 = self.regs.A() as u16 ^ __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xB0(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A | B"
        {
            let __1 = self.regs.B() as u16;
            let __0 = self.regs.A() as u16 | __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xB1(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A | C"
        {
            let __1 = self.regs.C() as u16;
            let __0 = self.regs.A() as u16 | __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xB2(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A | D"
        {
            let __1 = self.regs.D() as u16;
            let __0 = self.regs.A() as u16 | __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xB3(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A | E"
        {
            let __1 = self.regs.E() as u16;
            let __0 = self.regs.A() as u16 | __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xB4(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A | H"
        {
            let __1 = self.regs.H() as u16;
            let __0 = self.regs.A() as u16 | __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xB5(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A | L"
        {
            let __1 = self.regs.L() as u16;
            let __0 = self.regs.A() as u16 | __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xB6(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A | (HL)"
        {
            let __2 = self.regs.HL();
            let __1 = self.mem.load_u8(__2) as u16;
            let __0 = self.regs.A() as u16 | __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xB7(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A | A"
        {
            let __1 = self.regs.A() as u16;
            let __0 = self.regs.A() as u16 | __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xB8(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A - B"
        {
            let __1 = self.regs.B() as u16;
            let __0 = self.regs.A() as u16 - __1;
        }
        pc
    }

    pub fn opc_0xB9(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A - C"
        {
            let __1 = self.regs.C() as u16;
            let __0 = self.regs.A() as u16 - __1;
        }
        pc
    }

    pub fn opc_0xBA(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A - D"
        {
            let __1 = self.regs.D() as u16;
            let __0 = self.regs.A() as u16 - __1;
        }
        pc
    }

    pub fn opc_0xBB(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A - E"
        {
            let __1 = self.regs.E() as u16;
            let __0 = self.regs.A() as u16 - __1;
        }
        pc
    }

    pub fn opc_0xBC(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A - H"
        {
            let __1 = self.regs.H() as u16;
            let __0 = self.regs.A() as u16 - __1;
        }
        pc
    }

    pub fn opc_0xBD(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A - L"
        {
            let __1 = self.regs.L() as u16;
            let __0 = self.regs.A() as u16 - __1;
        }
        pc
    }

    pub fn opc_0xBE(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A - (HL)"
        {
            let __2 = self.regs.HL();
            let __1 = self.mem.load_u8(__2) as u16;
            let __0 = self.regs.A() as u16 - __1;
        }
        pc
    }

    pub fn opc_0xBF(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A - A"
        {
            let __1 = self.regs.A() as u16;
            let __0 = self.regs.A() as u16 - __1;
        }
        pc
    }

    pub fn opc_0xC0(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if !Z, RET"
        {
            let __0 = !self.flags.Z as u16;
            if __0 != 0 {
                // Statement: "RET"
                {
                    let __0 = self.opc_0xC9(instr);
                }
            }
        }
        pc
    }

    pub fn opc_0xC1(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "C <- (SP)"
        {
            let __1 = self.sp;
            let __0 = self.mem.load_u8(__1) as u16;
            self.regs.set_C(__0 as u8);
        }
        // Statement: "B <- (SP + 1)"
        {
            let __2 = 1;
            let __1 = self.sp + __2;
            let __0 = self.mem.load_u8(__1) as u16;
            self.regs.set_B(__0 as u8);
        }
        // Statement: "SP <- SP + 2"
        {
            let __1 = 2;
            let __0 = self.sp + __1;
            self.sp = __0 as u16;
        }
        pc
    }

    pub fn opc_0xC2(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if !Z, PC <- adr"
        {
            let __0 = !self.flags.Z as u16;
            if __0 != 0 {
                // Statement: "PC <- adr"
                {
                    let __0 = instr.data.unwrap();
                    pc = __0 as u16;
                }
            }
        }
        pc
    }

    pub fn opc_0xC3(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "PC <- adr"
        {
            let __0 = instr.data.unwrap();
            pc = __0 as u16;
        }
        pc
    }

    pub fn opc_0xC4(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if !Z, CALL adr"
        {
            let __0 = !self.flags.Z as u16;
            if __0 != 0 {
                // Statement: "CALL adr"
                {
                    let __0 = self.opc_0xCD(&Instruction {
                        opcode: 0xCD,
                        data: Some(instr.data.unwrap()),
                    });
                }
            }
        }
        pc
    }

    pub fn opc_0xC5(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "(SP - 2) <- C"
        {
            let __0 = self.regs.C() as u16;
            let __2 = 2;
            let __1 = self.sp - __2;
            self.mem.store_u8(__1, __0 as u8);
        }
        // Statement: "(SP - 1) <- B"
        {
            let __0 = self.regs.B() as u16;
            let __2 = 1;
            let __1 = self.sp - __2;
            self.mem.store_u8(__1, __0 as u8);
        }
        // Statement: "SP <- SP - 2"
        {
            let __1 = 2;
            let __0 = self.sp - __1;
            self.sp = __0 as u16;
        }
        pc
    }

    pub fn opc_0xC6(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A + imm"
        {
            let __1 = instr.data.unwrap();
            let __0 = self.regs.A() as u16 + __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xC7(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "CALL $0"
        {
            let __0 = self.opc_0xCD(&Instruction {
                opcode: 0xCD,
                data: Some(0),
            });
        }
        pc
    }

    pub fn opc_0xC8(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if Z, RET"
        {
            let __0 = self.flags.Z as u16;
            if __0 != 0 {
                // Statement: "RET"
                {
                    let __0 = self.opc_0xC9(instr);
                }
            }
        }
        pc
    }

    pub fn opc_0xC9(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "PC.lo <- (SP)"
        {
            let __1 = self.sp;
            let __0 = self.mem.load_u8(__1) as u16;
            pc = (pc & 0xFF00) | ((__0 as u16) & 0xFF);
        }
        // Statement: "PC.hi <- (SP + 1)"
        {
            let __2 = 1;
            let __1 = self.sp + __2;
            let __0 = self.mem.load_u8(__1) as u16;
            pc = (pc & 0x00FF) | ((__0 as u16) << 8);
        }
        // Statement: "SP <- SP + 2"
        {
            let __1 = 2;
            let __0 = self.sp + __1;
            self.sp = __0 as u16;
        }
        pc
    }

    pub fn opc_0xCA(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if Z, PC <- adr"
        {
            let __0 = self.flags.Z as u16;
            if __0 != 0 {
                // Statement: "PC <- adr"
                {
                    let __0 = instr.data.unwrap();
                    pc = __0 as u16;
                }
            }
        }
        pc
    }

    pub fn opc_0xCB(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        pc
    }

    pub fn opc_0xCC(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if Z, CALL adr"
        {
            let __0 = self.flags.Z as u16;
            if __0 != 0 {
                // Statement: "CALL adr"
                {
                    let __0 = self.opc_0xCD(&Instruction {
                        opcode: 0xCD,
                        data: Some(instr.data.unwrap()),
                    });
                }
            }
        }
        pc
    }

    pub fn opc_0xCD(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "(SP - 1) <- PC.hi"
        {
            let __0 = (pc >> 8);
            let __2 = 1;
            let __1 = self.sp - __2;
            self.mem.store_u8(__1, __0 as u8);
        }
        // Statement: "(SP - 2) <- PC.lo"
        {
            let __0 = (pc & 0xFF);
            let __2 = 2;
            let __1 = self.sp - __2;
            self.mem.store_u8(__1, __0 as u8);
        }
        // Statement: "SP <- SP + 2"
        {
            let __1 = 2;
            let __0 = self.sp + __1;
            self.sp = __0 as u16;
        }
        // Statement: "PC <- adr"
        {
            let __0 = instr.data.unwrap();
            pc = __0 as u16;
        }
        pc
    }

    pub fn opc_0xCE(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A + imm + CY"
        {
            let __2 = self.flags.CY as u16;
            let __1 = instr.data.unwrap() + __2;
            let __0 = self.regs.A() as u16 + __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xCF(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "CALL $8"
        {
            let __0 = self.opc_0xCD(&Instruction {
                opcode: 0xCD,
                data: Some(8),
            });
        }
        pc
    }

    pub fn opc_0xD0(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if !CY, RET"
        {
            let __0 = !self.flags.CY as u16;
            if __0 != 0 {
                // Statement: "RET"
                {
                    let __0 = self.opc_0xC9(instr);
                }
            }
        }
        pc
    }

    pub fn opc_0xD1(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "E <- (SP)"
        {
            let __1 = self.sp;
            let __0 = self.mem.load_u8(__1) as u16;
            self.regs.set_E(__0 as u8);
        }
        // Statement: "D <- (SP + 1)"
        {
            let __2 = 1;
            let __1 = self.sp + __2;
            let __0 = self.mem.load_u8(__1) as u16;
            self.regs.set_D(__0 as u8);
        }
        // Statement: "SP <- SP + 2"
        {
            let __1 = 2;
            let __0 = self.sp + __1;
            self.sp = __0 as u16;
        }
        pc
    }

    pub fn opc_0xD2(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if !CY, PC <- adr"
        {
            let __0 = !self.flags.CY as u16;
            if __0 != 0 {
                // Statement: "PC <- adr"
                {
                    let __0 = instr.data.unwrap();
                    pc = __0 as u16;
                }
            }
        }
        pc
    }

    pub fn opc_0xD3(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "special"
        {
            // TODO: implement me
        }
        pc
    }

    pub fn opc_0xD4(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if !CY, CALL adr"
        {
            let __0 = !self.flags.CY as u16;
            if __0 != 0 {
                // Statement: "CALL adr"
                {
                    let __0 = self.opc_0xCD(&Instruction {
                        opcode: 0xCD,
                        data: Some(instr.data.unwrap()),
                    });
                }
            }
        }
        pc
    }

    pub fn opc_0xD5(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "(SP - 2) <- E"
        {
            let __0 = self.regs.E() as u16;
            let __2 = 2;
            let __1 = self.sp - __2;
            self.mem.store_u8(__1, __0 as u8);
        }
        // Statement: "(SP - 1) <- D"
        {
            let __0 = self.regs.D() as u16;
            let __2 = 1;
            let __1 = self.sp - __2;
            self.mem.store_u8(__1, __0 as u8);
        }
        // Statement: "SP <- SP - 2"
        {
            let __1 = 2;
            let __0 = self.sp - __1;
            self.sp = __0 as u16;
        }
        pc
    }

    pub fn opc_0xD6(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A - imm"
        {
            let __1 = instr.data.unwrap();
            let __0 = self.regs.A() as u16 - __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xD7(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "CALL $10"
        {
            let __0 = self.opc_0xCD(&Instruction {
                opcode: 0xCD,
                data: Some(10),
            });
        }
        pc
    }

    pub fn opc_0xD8(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if CY, RET"
        {
            let __0 = self.flags.CY as u16;
            if __0 != 0 {
                // Statement: "RET"
                {
                    let __0 = self.opc_0xC9(instr);
                }
            }
        }
        pc
    }

    pub fn opc_0xD9(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        pc
    }

    pub fn opc_0xDA(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if CY, PC <- adr"
        {
            let __0 = self.flags.CY as u16;
            if __0 != 0 {
                // Statement: "PC <- adr"
                {
                    let __0 = instr.data.unwrap();
                    pc = __0 as u16;
                }
            }
        }
        pc
    }

    pub fn opc_0xDB(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "special"
        {
            // TODO: implement me
        }
        pc
    }

    pub fn opc_0xDC(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if CY, CALL adr"
        {
            let __0 = self.flags.CY as u16;
            if __0 != 0 {
                // Statement: "CALL adr"
                {
                    let __0 = self.opc_0xCD(&Instruction {
                        opcode: 0xCD,
                        data: Some(instr.data.unwrap()),
                    });
                }
            }
        }
        pc
    }

    pub fn opc_0xDD(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        pc
    }

    pub fn opc_0xDE(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A - imm - CY"
        {
            let __2 = self.flags.CY as u16;
            let __1 = instr.data.unwrap() - __2;
            let __0 = self.regs.A() as u16 - __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xDF(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "CALL $18"
        {
            let __0 = self.opc_0xCD(&Instruction {
                opcode: 0xCD,
                data: Some(18),
            });
        }
        pc
    }

    pub fn opc_0xE0(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if !P, RET"
        {
            let __0 = !self.flags.P as u16;
            if __0 != 0 {
                // Statement: "RET"
                {
                    let __0 = self.opc_0xC9(instr);
                }
            }
        }
        pc
    }

    pub fn opc_0xE1(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "L <- (SP)"
        {
            let __1 = self.sp;
            let __0 = self.mem.load_u8(__1) as u16;
            self.regs.set_L(__0 as u8);
        }
        // Statement: "H <- (SP + 1)"
        {
            let __2 = 1;
            let __1 = self.sp + __2;
            let __0 = self.mem.load_u8(__1) as u16;
            self.regs.set_H(__0 as u8);
        }
        // Statement: "SP <- SP + 2"
        {
            let __1 = 2;
            let __0 = self.sp + __1;
            self.sp = __0 as u16;
        }
        pc
    }

    pub fn opc_0xE2(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if !P, PC <- adr"
        {
            let __0 = !self.flags.P as u16;
            if __0 != 0 {
                // Statement: "PC <- adr"
                {
                    let __0 = instr.data.unwrap();
                    pc = __0 as u16;
                }
            }
        }
        pc
    }

    pub fn opc_0xE3(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "L <-> (SP)"
        {
            let __2 = self.sp;
            let __1 = self.mem.load_u8(__2) as u16;
            let __0 = /* TODO: implement me! */ 0;
        }
        // Statement: "H <-> (SP + 1)"
        {
            let __3 = 1;
            let __2 = self.sp + __3;
            let __1 = self.mem.load_u8(__2) as u16;
            let __0 = /* TODO: implement me! */ 0;
        }
        pc
    }

    pub fn opc_0xE4(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if !P, CALL adr"
        {
            let __0 = !self.flags.P as u16;
            if __0 != 0 {
                // Statement: "CALL adr"
                {
                    let __0 = self.opc_0xCD(&Instruction {
                        opcode: 0xCD,
                        data: Some(instr.data.unwrap()),
                    });
                }
            }
        }
        pc
    }

    pub fn opc_0xE5(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "(SP - 2) <- L"
        {
            let __0 = self.regs.L() as u16;
            let __2 = 2;
            let __1 = self.sp - __2;
            self.mem.store_u8(__1, __0 as u8);
        }
        // Statement: "(SP - 1) <- H"
        {
            let __0 = self.regs.H() as u16;
            let __2 = 1;
            let __1 = self.sp - __2;
            self.mem.store_u8(__1, __0 as u8);
        }
        // Statement: "SP <- SP - 2"
        {
            let __1 = 2;
            let __0 = self.sp - __1;
            self.sp = __0 as u16;
        }
        pc
    }

    pub fn opc_0xE6(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A & imm"
        {
            let __1 = instr.data.unwrap();
            let __0 = self.regs.A() as u16 & __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xE7(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "CALL $20"
        {
            let __0 = self.opc_0xCD(&Instruction {
                opcode: 0xCD,
                data: Some(20),
            });
        }
        pc
    }

    pub fn opc_0xE8(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if P, RET"
        {
            let __0 = self.flags.P as u16;
            if __0 != 0 {
                // Statement: "RET"
                {
                    let __0 = self.opc_0xC9(instr);
                }
            }
        }
        pc
    }

    pub fn opc_0xE9(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "PC.hi <- H"
        {
            let __0 = self.regs.H() as u16;
            pc = (pc & 0x00FF) | ((__0 as u16) << 8);
        }
        // Statement: "PC.lo <- L"
        {
            let __0 = self.regs.L() as u16;
            pc = (pc & 0xFF00) | ((__0 as u16) & 0xFF);
        }
        pc
    }

    pub fn opc_0xEA(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if P, PC <- adr"
        {
            let __0 = self.flags.P as u16;
            if __0 != 0 {
                // Statement: "PC <- adr"
                {
                    let __0 = instr.data.unwrap();
                    pc = __0 as u16;
                }
            }
        }
        pc
    }

    pub fn opc_0xEB(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "H <-> D"
        {
            let __1 = self.regs.D() as u16;
            let __0 = /* TODO: implement me! */ 0;
        }
        // Statement: "L <-> E"
        {
            let __1 = self.regs.E() as u16;
            let __0 = /* TODO: implement me! */ 0;
        }
        pc
    }

    pub fn opc_0xEC(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if P, CALL adr"
        {
            let __0 = self.flags.P as u16;
            if __0 != 0 {
                // Statement: "CALL adr"
                {
                    let __0 = self.opc_0xCD(&Instruction {
                        opcode: 0xCD,
                        data: Some(instr.data.unwrap()),
                    });
                }
            }
        }
        pc
    }

    pub fn opc_0xED(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        pc
    }

    pub fn opc_0xEE(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A ^ imm"
        {
            let __1 = instr.data.unwrap();
            let __0 = self.regs.A() as u16 ^ __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xEF(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "CALL $28"
        {
            let __0 = self.opc_0xCD(&Instruction {
                opcode: 0xCD,
                data: Some(28),
            });
        }
        pc
    }

    pub fn opc_0xF0(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if !S, RET"
        {
            let __0 = !self.flags.S as u16;
            if __0 != 0 {
                // Statement: "RET"
                {
                    let __0 = self.opc_0xC9(instr);
                }
            }
        }
        pc
    }

    pub fn opc_0xF1(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "flags <- (SP)"
        {
            let __1 = self.sp;
            let __0 = self.mem.load_u8(__1) as u16;
            self.flags.store(__0 as u8);
        }
        // Statement: "A <- (SP + 1)"
        {
            let __2 = 1;
            let __1 = self.sp + __2;
            let __0 = self.mem.load_u8(__1) as u16;
            self.regs.set_A(__0 as u8);
        }
        // Statement: "SP <- SP + 2"
        {
            let __1 = 2;
            let __0 = self.sp + __1;
            self.sp = __0 as u16;
        }
        pc
    }

    pub fn opc_0xF2(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if !S, PC <- adr"
        {
            let __0 = !self.flags.S as u16;
            if __0 != 0 {
                // Statement: "PC <- adr"
                {
                    let __0 = instr.data.unwrap();
                    pc = __0 as u16;
                }
            }
        }
        pc
    }

    pub fn opc_0xF3(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "special"
        {
            // TODO: implement me
        }
        pc
    }

    pub fn opc_0xF4(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if !S, CALL adr"
        {
            let __0 = !self.flags.S as u16;
            if __0 != 0 {
                // Statement: "CALL adr"
                {
                    let __0 = self.opc_0xCD(&Instruction {
                        opcode: 0xCD,
                        data: Some(instr.data.unwrap()),
                    });
                }
            }
        }
        pc
    }

    pub fn opc_0xF5(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "(SP - 2) <- flags"
        {
            let __0 = (self.flags.get() as u16);
            let __2 = 2;
            let __1 = self.sp - __2;
            self.mem.store_u8(__1, __0 as u8);
        }
        // Statement: "(SP - 1) <- A"
        {
            let __0 = self.regs.A() as u16;
            let __2 = 1;
            let __1 = self.sp - __2;
            self.mem.store_u8(__1, __0 as u8);
        }
        // Statement: "SP <- SP - 2"
        {
            let __1 = 2;
            let __0 = self.sp - __1;
            self.sp = __0 as u16;
        }
        pc
    }

    pub fn opc_0xF6(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A <- A | imm"
        {
            let __1 = instr.data.unwrap();
            let __0 = self.regs.A() as u16 | __1;
            self.regs.set_A(__0 as u8);
        }
        pc
    }

    pub fn opc_0xF7(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "CALL $30"
        {
            let __0 = self.opc_0xCD(&Instruction {
                opcode: 0xCD,
                data: Some(30),
            });
        }
        pc
    }

    pub fn opc_0xF8(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if S, RET"
        {
            let __0 = self.flags.S as u16;
            if __0 != 0 {
                // Statement: "RET"
                {
                    let __0 = self.opc_0xC9(instr);
                }
            }
        }
        pc
    }

    pub fn opc_0xF9(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "SP <- HL"
        {
            let __0 = self.regs.HL();
            self.sp = __0 as u16;
        }
        pc
    }

    pub fn opc_0xFA(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if S, PC <- adr"
        {
            let __0 = self.flags.S as u16;
            if __0 != 0 {
                // Statement: "PC <- adr"
                {
                    let __0 = instr.data.unwrap();
                    pc = __0 as u16;
                }
            }
        }
        pc
    }

    pub fn opc_0xFB(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "special"
        {
            // TODO: implement me
        }
        pc
    }

    pub fn opc_0xFC(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "if S, CALL adr"
        {
            let __0 = self.flags.S as u16;
            if __0 != 0 {
                // Statement: "CALL adr"
                {
                    let __0 = self.opc_0xCD(&Instruction {
                        opcode: 0xCD,
                        data: Some(instr.data.unwrap()),
                    });
                }
            }
        }
        pc
    }

    pub fn opc_0xFD(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        pc
    }

    pub fn opc_0xFE(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "A - imm"
        {
            let __1 = instr.data.unwrap();
            let __0 = self.regs.A() as u16 - __1;
        }
        pc
    }

    pub fn opc_0xFF(&mut self, instr: &Instruction) -> u16 {
        let mut pc = self.pc;
        // Statement: "CALL $38"
        {
            let __0 = self.opc_0xCD(&Instruction {
                opcode: 0xCD,
                data: Some(38),
            });
        }
        pc
    }
}
