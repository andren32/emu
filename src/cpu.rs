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
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            a: 0, x: 0, y: 0, sp: 0, pc: 0,
            c: false, z: false, i: true, d: false, b: false, u: false, v: false, n: false,

            mem: vec!(0; 0x10000),
        }
    }

    fn reset(&mut self) {

    }

    fn read_mem(&mut self, &addr: &u16) -> u8 {
        0
    }

    fn write_mem(&mut self, &addr: &u16, &data: &u8) {

    }

    fn execute(&mut self) {

    }
}
