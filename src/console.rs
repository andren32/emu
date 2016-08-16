// Console: represents the whole NES console with all
// peripheral devices. Capable of high level operations like
// start, reset, load game etc, much like a real physical machine

use cpu::CPU;
use mmu::MMU;
use ram::RAM;
use cartridge::Cartridge;

// NES contains 2 kB = 2^11 bytes of memory
const CPURAM: u16 = (1 << 11) as u16;

pub struct Console {
    cpu: CPU
}

impl Console {
    pub fn new() -> Console {
        let cartridge = Cartridge::new();
        let ram =  RAM::new(CPURAM);
        let mmu = MMU::new(ram);
        Console {
            cpu: CPU::new(mmu),
        }
    }

}
