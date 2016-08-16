// Console: represents the whole NES console with all
// peripheral devices. Capable of high level operations like
// start, reset, load game etc, much like a real physical machine

use cpu::CPU;
use mmu::MMU;
use ram::RAM;
use cartridge::Cartridge;

pub struct Console {
    ram: RAM,
    cartridge: Cartridge,
    mmu: MMU,
    cpu: CPU,
}

impl Console {
    pub fn new() -> Console {
        Console {
            cartridge: Cartridge::new(),
            ram: RAM::new()
            cpu: CPU::new(),
            mmu: MMU::new(),
        }
    }

}
