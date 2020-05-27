//flags.rs - code dealing with register flags

pub const CARRY_BIT  : u8 = 0b1;
pub const ZERO_BIT   : u8 = 0b10;
pub const INTERRUPT_DISABLE_BIT : u8 = 0b100;
pub const DECIMAL_MODE_BIT : u8 = 0b1000;
pub const BREAK_CMD_BIT : u8 = 0b1_0000;
pub const OVERFLOW_BIT : u8 = 0b10_0000;
pub const NEGATIVE_BIT: u8 = 0b100_0000;


fn set_flag(flags: &mut u8, bit_index: u8){
    *flags |= bit_index;
}

pub fn set_carry(mut flags: &mut u8){
   set_flag(flags, CARRY_BIT);
}

pub fn set_zero(mut flags: &mut u8){
    set_flag(flags, ZERO_BIT);
}

pub fn set_negative(mut flags: &mut u8){
    set_flag(flags, NEGATIVE_BIT);
}

pub fn set_decimal(mut flags: &mut u8){
    set_flag(flags, DECIMAL_MODE_BIT);
}

pub fn set_interrupt_disable(mut flags: &mut u8){
    set_flag(flags, INTERRUPT_DISABLE_BIT);
}

fn clear_flag(flags: &mut u8, bit_index: u8){
    *flags ^= bit_index;
}

pub fn clear_carry(mut flags: &mut u8){
    clear_flag(&mut flags, CARRY_BIT);
}

pub fn clear_overflow(mut flags: &mut u8){
    clear_flag(&mut flags, OVERFLOW_BIT);
}

pub fn clear_interrupt_disable(mut flags: &mut u8){
    clear_flag(&mut flags, INTERRUPT_DISABLE_BIT);
}

pub fn clear_decimal_mode(mut flags: &mut u8){
    clear_flag(&mut flags, DECIMAL_MODE_BIT);
}
