#![allow(dead_code)]

use std::fs::File;
use std::io::Read;

mod cpu;

fn main() {
    let cpu = cpu::CPU::new();
}

fn read_rom() -> Vec<u8> {
    let mut f = File::open("smb.nes").unwrap();
    let mut rom: Vec<u8> = Vec::new();
    f.read_to_end(&mut rom).unwrap();

    rom
}
