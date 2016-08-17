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
        MMU { ram: ram }
    }

    // ram is mirrored, wrap around 2 kb
    fn rb_ram(&self, addr: u16) -> u8 {
        let phys = addr % 0x0800;
        self.ram.rb(phys)
    }

    fn wb_ram(&mut self, addr: u16, data: u8) {
        let phys = addr % 0x0800;
        self.ram.wb(phys, data);
    }

    // io registers mirrored
    fn rb_io_low(&self, addr: u16) -> u8 {
        let io_device = (addr - 0x2000) % 8;
        // read from io
        unimplemented!();
    }

    fn wb_io_low(&mut self, addr: u16, data: u8) {
        let io_device = (addr - 0x2000) % 8;
        // write to io
        unimplemented!();
    }

    fn rb_io_high(&self, addr: u16) -> u8 {
        let io_device = (addr - 0x4000) % 8;
        unimplemented!();
    }

    fn wb_io_high(&mut self, addr: u16, data: u8) {
        let io_device = (addr - 0x4000) % 8;
        // write to io
        unimplemented!();
    }

    fn rb_exp_rom(&self, addr: u16) -> u8 {
        unimplemented!();
    }

    fn wb_exp_rom(&mut self, addr: u16, data: u8) {
        unimplemented!();
    }

    fn rb_sram(&self, addr: u16) -> u8 {
        unimplemented!();
    }

    fn wb_sram(&mut self, addr: u16, data: u8) {
        unimplemented!();
    }

    fn rb_prg_rom(&self, addr: u16) -> u8 {
        unimplemented!();
    }

    fn wb_prg_rom(&mut self, addr: u16, data: u8) {
        unimplemented!();
    }
}

impl Memory for MMU {
    fn rb(&self, addr: u16) -> u8 {
        match addr {
            0x0000...0x1FFF => self.rb_ram(addr),
            0x2000...0x3FFF => self.rb_io_low(addr),
            0x4000...0x401F => self.rb_io_high(addr),
            0x4020...0x5FFF => self.rb_exp_rom(addr),
            0x6000...0x7FFF => self.rb_sram(addr),
            0x8000...0xFFFF => self.rb_prg_rom(addr),
            _ => panic!("Invalid memory access at {0:x}", addr),
        }
    }

    fn wb(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000...0x2000 => self.wb_ram(addr, data),
            0x2000...0x3FFF => self.wb_io_low(addr, data),
            0x4000...0x401F => self.wb_io_high(addr, data),
            0x4020...0x5FFF => self.wb_exp_rom(addr, data),
            0x6000...0x7FFF => self.wb_sram(addr, data),
            0x8000...0xFFFF => self.wb_prg_rom(addr, data),
            _ => panic!("Invalid memory access at {0:x}", addr),
        }
    }
}
