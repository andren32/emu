// Console: represents the whole NES console with all
// peripheral devices. Capable of high level operations like
// start, reset, load game etc, much like a real physical machine

use cpu::CPU;
use mmu::MMU;
use ram::RAM;

pub struct Console {
    ram: RAM,
    mmu: MMU,
    cpu: CPU,
}

impl Console {
    pub fn new() -> Console {
        Console {
            cpu: CPU::new(),
            mmu: MMU::new(),
            ram: RAM::new()
        }
    }

}
