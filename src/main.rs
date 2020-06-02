//main.rs for nes-emu
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

use nes_emu::nes_6502::Nes6502;
use nes_emu::memory::RAM;


fn main() {
    // todo 
    // get env args
    // read rom into mem
    // start cpu execution

    let args : Vec<String> = env::args().collect();
    let rom_path = &args[1];

    let rom_data = load_binary(rom_path);
    let mut ram : RAM = RAM::new();
    ram.load_rom(rom_data);
    
    let mut cpu : Nes6502 = Nes6502::new();
    loop {
        cpu.run(&mut ram);
    }
}

fn load_binary<P: AsRef<Path>>(path : P) -> Box<[u8]> {
    let mut file = fs::File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf);
    file_buf.into_boxed_slice()
}
