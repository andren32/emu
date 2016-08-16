#![allow(dead_code)]

mod memory;
mod mmu;
mod console;
mod cpu;
mod ram;
mod cartridge;

use console::Console;
use std::fs::File;
use std::io::Read;

fn main() {
    let console = Console::new();
}

fn read_rom() -> Vec<u8> {
    let mut f = File::open("smb.nes").unwrap();
    let mut rom: Vec<u8> = Vec::new();
    f.read_to_end(&mut rom).unwrap();

    rom
}
