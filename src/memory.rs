/// Memory map of the 1978 Space Invaders machine.
pub struct MemoryMap {
    rom: [u8; 0x2000],
    wram: [u8; 0x0400],
    vram: [u8; 0x1C00],
}

impl MemoryMap {
    /// Read an u8 from the requested address in memory.
    pub fn load_u8(&self, addr: u16) -> u8 {
        self.decode(addr)[0]
    }

    /// Read an u16 from the requested address in memory.
    pub fn load_u16(&self, addr: u16) -> u16 {
        let mem = self.decode(addr);
        (mem[1] as u16) << 8 | mem[0] as u16
    }

    /// Write an u8 to the requested address in memory.
    pub fn store_u8(&mut self, addr: u16, val: u8) {
        self.decode_mut(addr)[0] = val;
    }

    /// Write an u16 to the requested address in memory.
    pub fn store_u16(&mut self, addr: u16, val: u16) {
        let mem = self.decode_mut(addr);
        mem[0] = (val & 0xFF) as u8;
        mem[1] = (val >> 8) as u8;
    }

    fn decode(&self, addr: u16) -> &[u8] {
        match (addr & 0xFF00) >> 8 {
            0x00...0x1F => &self.rom[addr as usize..],
            0x20...0x23 => &self.wram[(addr - 0x2000) as usize..],
            0x24...0x3f => &self.vram[(addr - 0x2400) as usize..],
            0x40...0x43 => &self.wram[(addr - 0x4000) as usize..],
            0x44...0x5f => &self.vram[(addr - 0x4400) as usize..],
            _ => unreachable!(),
        }
    }

    // TODO: find a safe way to merge decode and decode_mut
    fn decode_mut(&mut self, addr: u16) -> &mut [u8] {
        match (addr & 0xFF00) >> 8 {
            0x00...0x1F => &mut self.rom[addr as usize..],
            0x20...0x23 => &mut self.wram[(addr - 0x2000) as usize..],
            0x24...0x3f => &mut self.vram[(addr - 0x2400) as usize..],
            0x40...0x43 => &mut self.wram[(addr - 0x4000) as usize..],
            0x44...0x5f => &mut self.vram[(addr - 0x4400) as usize..],
            _ => unreachable!(),
        }
    }
}
