use std::io::{self, Read};

/// Memory map of the 1978 Space Invaders machine.
pub struct MemoryMap {
    pub rom: [u8; 0x2000],
    pub wram: [u8; 0x0400],
    pub vram: [u8; 0x1C00],
}

impl MemoryMap {
    /// Create a new memory map
    pub fn new() -> MemoryMap {
        MemoryMap {
            rom: [0; 0x2000],
            wram: [0; 0x0400],
            vram: [0; 0x1C00],
        }
    }

    /// Load ROM into memory.
    pub fn load_rom(&mut self, rd: &mut Read) -> Result<usize, io::Error> {
        rd.read(&mut self.rom[..])
    }

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
