// the laborious task of writing the memory map, and it's many ranges and behaviour depending on the address being written to!

pub const INTERNAL_RAM_START : usize = 0x0;
pub const INTERNAL_RAM_END : usize = 0x7FF;

pub const INTERNAL_RAM_MIRROR_ONE_START : usize = 0x800;
pub const INTERNAL_RAM_MIRROR_ONE_END   : usize = 0xFFF;

pub const INTERNAL_RAM_MIRROR_TWO_START : usize = 0x1000;
pub const INTERNAL_RAM_MIRROR_TWO_END   : usize = 0x17FF;

pub const INTERNAL_RAM_MIRROR_THREE_START : usize = 0x1800;
pub const INTERNAL_RAM_MIRROR_THREE_END   : usize = 0x1FFF;

pub const PPU_REGISTERS_START : usize = 0x2000;
pub const PPU_REGISTERS_END : usize = 0x2007;

pub const PPU_REGISTERS_MIRRORS_START : usize = 0x2008;
pub const PPU_REGISTERS_MIRRORS_END : usize = 0x3FFF;

pub const APU_REGISTERS_START: usize = 0x4000;
pub const APU_REGISTERS_END: usize = 0x4015;

pub const JOYPAD_ONE: usize = 0x4016;
pub const JOYPAD_TWO: usize = 0x4017;