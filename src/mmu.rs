// MMU: emulates the MMU called by the CPU when accessing
// mapped memory. All access to peripheral devices and
// physical memory by the CPU goes through this module

// NOTE: For now only one MMU is in place, but another one
// may be needed by the PPU as has been done in
// https://github.com/fogleman/nes. The goal however is to
// give the logical devices as little access to each other as
// possible, to avoid complex mutation patterns

pub struct MMU {
}

impl MMU {
    pub fn new() -> MMU {
        MMU { }
    }
}
