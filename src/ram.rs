// RAM: emulates the shared physical memory of the console
// An instance of this may be shared between devices

use memory::Memory;

pub struct RAM {
    mem: Vec<u8>
}

impl RAM {
    pub fn new(size: u16) -> RAM {
        RAM { mem: vec!(0; size as usize) }
    }

    pub fn reset(&mut self) {
        for b in &mut self.mem {
            *b = 0;
        }
    }

    pub fn rb(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn wb(&mut self, addr: u16, data: u8) {
        self.mem[addr as usize] = data;
    }
}

impl Memory for RAM {
    fn rb(&self, addr: u16) -> u8 {
        self.rb(addr)
    }

    fn wb(&mut self, addr: u16, data: u8) {
        self.wb(addr, data);
    }
}
