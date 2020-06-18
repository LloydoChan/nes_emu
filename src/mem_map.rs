// the laborious task of writing the memory map, and it's many ranges and behaviour depending on the address being written to!

pub const STACK_START: usize = 0x100;
pub const STACK_END: usize = 0x1FF;

pub const INTERNAL_RAM_START: usize = 0x0;
pub const INTERNAL_RAM_END: usize = 0x7FF;

pub const INTERNAL_RAM_MIRROR_ONE_START: usize = 0x800;
pub const INTERNAL_RAM_MIRROR_ONE_END: usize = 0xFFF;

pub const INTERNAL_RAM_MIRROR_TWO_START: usize = 0x1000;
pub const INTERNAL_RAM_MIRROR_TWO_END: usize = 0x17FF;

pub const INTERNAL_RAM_MIRROR_THREE_START: usize = 0x1800;
pub const INTERNAL_RAM_MIRROR_THREE_END: usize = 0x1FFF;

pub const PPU_REGISTERS_START: usize = 0x2000;
pub const PPU_REGISTERS_END: usize = 0x2007;

pub const PPUCTRL: usize = 0x2000;
pub const PPUMASK: usize = 0x2001;
pub const PPUSTATUS: usize = 0x2002;
pub const OAMADDR: usize = 0x2003;
pub const OAMDATA: usize = 0x2004;
pub const PPUSCROLL: usize = 0x2005;
pub const PPUADDR: usize = 0x2006;
pub const PPUDATA: usize = 0x2007;

pub const PPU_REGISTERS_MIRRORS_START: usize = 0x2008;
pub const PPU_REGISTERS_MIRRORS_END: usize = 0x3FFF;

pub const APU_REGISTERS_START: usize = 0x4000;
pub const APU_REGISTERS_END: usize = 0x4015;

pub const JOYPAD_ONE: usize = 0x4016;
pub const JOYPAD_TWO: usize = 0x4017;

pub const MIRROR_ONE_ROM_START: usize = 0x8000;
pub const MIRROR_ONE_ROM_END: usize = 0xBFFF;

pub const MIRROR_TWO_ROM_START: usize = 0xC000;
pub const MIRROR_TWO_ROM_END: usize = 0xFFFF;

// VRAM
//pattern tables one and two
pub const PATTERN_TABLE_ZERO_START: usize = 0x0;
pub const PATTERN_TABLE_ZERO_END: usize = 0xFFF;

pub const PATTERN_TABLE_ONE_START: usize = 0x1000;
pub const PATTERN_TABLE_ONE_END: usize = 0x1FFF;

pub const NAME_TABLE_ZERO_START: usize = 0x2000;
pub const NAME_TABLE_ZERO_END: usize = 0x23FF;

pub const NAME_TABLE_ONE_START: usize = 0x2400;
pub const NAME_TABLE_ONE_END: usize = 0x27FF;

pub const NAME_TABLE_TWO_START: usize = 0x2800;
pub const NAME_TABLE_TWO_END: usize = 0x2BFF;

pub const NAME_TABLE_THREE_START: usize = 0x2C00;
pub const NAME_TABLE_THREE_END: usize = 0x2FFF;

pub const NAME_TABLE_ZERO_MIRROR_START: usize = 0x3000;
pub const NAME_TABLE_ZERO_MIRROR_END: usize = 0x33FF;

pub const NAME_TABLE_ONE_MIRROR_START: usize = 0x3400;
pub const NAME_TABLE_ONE_MIRROR_END: usize = 0x37FF;

pub const NAME_TABLE_TWO_MIRROR_START: usize = 0x3800;
pub const NAME_TABLE_TWO_MIRROR_END: usize = 0x3BFF;

pub const NAME_TABLE_THREE_MIRROR_START: usize = 0x3C00;
pub const NAME_TABLE_THREE_MIRROR_END: usize = 0x3EFF;

pub const PALLETE_RAM_INDICES_START: usize = 0x3F00;
pub const PALLETE_RAM_INDICES_END: usize = 0x3F1F;

pub const OAM_DMA: usize = 0x4014;
