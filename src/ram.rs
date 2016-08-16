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

    pub fn len(&self) -> usize {
        self.mem.len()
    }
}

impl Memory for RAM {

    fn rb(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    fn wb(&mut self, addr: u16, data: u8) {
        self.mem[addr as usize] = data;
    }
}
