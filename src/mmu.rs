// MMU: emulates the MMU called by the CPU when accessing
// mapped memory. All access to peripheral devices and
// physical memory by the CPU goes through this module

// NOTE: For now only one MMU is in place, but another one
// may be needed by the PPU as has been done in
// https://github.com/fogleman/nes. The goal however is to
// give the logical devices as little access to each other as
// possible, to avoid complex mutation patterns

use ram::RAM;
use memory::Memory;

pub struct MMU {
    ram: RAM,
}

impl MMU {
    pub fn new(ram: RAM) -> MMU {
        MMU {ram: ram}
    }

    pub fn rb(&self, addr: u16) -> u8 {
        self.ram.rb(addr)
    }

    pub fn wb(&mut self, addr: u16, data: u8) {
        self.ram.wb(addr, data);
    }
}

impl Memory for MMU {
    fn rb(&self, addr: u16) -> u8 {
        self.rb(addr)
    }

    fn wb(&mut self, addr: u16, data: u8) {
        self.wb(addr, data);
    }
}
