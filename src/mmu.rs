// MMU: emulates the MMU called by the CPU when accessing
// mapped memory. All access to peripheral devices and
// physical memory by the CPU goes through this module

// NOTE: For now only one MMU is in place, but another one
// may be needed by the PPU as has been done in
// https://github.com/fogleman/nes. The goal however is to
// give the logical devices as little access to each other as
// possible, to avoid complex mutation patterns

// Memory map
// 0x100 => Zero Page
// 0x200 => Stack
// 0x800 => RAM
// 0x2000 => Mirrors (0-0x7FF)
// 0x2008 => I/O Registers
// 0x4000 => Mirrors (0x2000-0x2007)
// 0x4020 => I/O Registers
// 0x6000 => Expansion ROM
// 0x8000 => SRAM
// 0xC000 => PRG-ROM (Lower Bank)
// 0x10000 => PRG-ROM (Upper Bank)

use ram::RAM;
use memory::Memory;

pub struct MMU {
    ram: RAM,
}

impl MMU {
    pub fn new(ram: RAM) -> MMU {
        MMU {ram: ram}
    }

    // ram is mirrored, i.e. wrap around
    // for physical vector addr in
    // the specified range
    fn phys_addr(&self, addr: u16) -> u16 {
        ((addr as usize) % self.ram.len()) as u16
    }

    fn rb_ram(&self, addr: u16) -> u8 {
        self.ram.rb(self.phys_addr(addr))
    }

    fn wb_ram(&mut self, addr: u16, data: u8) {
        let phys = self.phys_addr(addr);
        self.ram.wb(phys, data);
    }
}

impl Memory for MMU {
    fn rb(&self, addr: u16) -> u8 {
        match addr {
            0x0000 ... 0x07ff => self.rb_ram(addr),
            _ => 0 as u8
        }
    }

    fn wb(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000 ... 0x07ff => self.wb_ram(addr, data),
            _ => ()
        }
    }
}
