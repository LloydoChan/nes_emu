// 6502.rs for nes-emu
// looking to do a lot of high level stuff regarding the 6502 in here
// will put instructions in different .rs files.

#[derive(Debug)]
struct Nes6502{
    accumulator : u8,
    x : u8,
    y : u8,
    status_flags : u8, // only need 7 bits of this
    stack_pointer : u8,
    pc_counter : u16
}

impl Nes6502 {
    pub fn new() -> Self{
        Nes6502{
            accumulator : 0,
            x : 0,
            y : 0,
            status_flags : 0xFD, // only need 7 bits of this
            stack_pointer : 0,
            pc_counter : 0x34
        }
    }
}