// Cartridge: represents the physical game cartridge

pub struct Cartridge {
    prg: Vec<u8>,
    chr: Vec<u8>,
    sram: Vec<u8>,
}

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge {
            prg: Vec::new(),
            chr: Vec::new(),
            sram: Vec::new(),
        }
    }
}
