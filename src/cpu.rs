// CPU: emulates the 6502 cpu inside an NES
// efforts will be focused on the NTSC version for simplicity

use mmu::MMU;

pub struct CPU {
    a: u8, // accumulator register
    x: u8, // index register 1
    y: u8, // index register 2
    sp: u8, //stack pointer
    pc: u16, // program counter

    // These flags represent the p register which is actually just a one byte register
    // here I represent it as bools just because it's a bit easier
    c: bool, // carry flag
    z: bool, // zero flag
    i: bool, // interupt disable flag
    d: bool, // decimal flag <-- not supported by the NES version of the 6502 CPU
    b: bool, // break command flag
    u: bool, // unused flag
    v: bool, // overflow flag
    n: bool, // negative flag


    // Memory
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
    mem: Vec<u8>,
    mmu: MMU,
}

impl CPU {
    pub fn new(mmu: MMU) -> CPU {
        CPU {
            a: 0, x: 0, y: 0, sp: 0xFD, pc: 0,
            c: false, z: false, i: true, d: false, b: false, u: false, v: false, n: false,

            mem: vec!(0; 0x10000),
            mmu: mmu
        }
    }

    // pop element from stack
    fn pop(&mut self) -> u8 {
        // stack starts at 0x100 and goes to 0x1FF stackpointer goes from 0x00 to 0xFF
        // so bitwise or gets us the address
        let addr = 0x100 | (self.sp as u16);
        self.sp += 1;

        self.mmu.rb(addr)
    }

    // push element on stack
    fn push(&mut self, data: u8) {
        // stack starts at 0x100 and goes to 0x1FF stackpointer goes from 0x00 to 0xFF
        // so bitwise or gets us the address
        let addr = 0x100 | (self.sp as u16);
        self.sp -= 1;

        self.mmu.wb(addr, data);
    }

    // execute an instruction
    fn execute(&mut self) {

    }
}
