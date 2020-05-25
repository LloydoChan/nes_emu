//flags.rs - code dealing with register flags

pub const CARRY_BIT  : u8 = 0b1;
pub const ZERO_BIT   : u8 = 0b10;
pub const INTERRUPT_DISABLE_BIT : u8 = 0b100;
pub const DECIMAL_MODE_BIT : u8 = 0b1000;
pub const BREAK_CMD_BIT : u8 = 0b1_0000;
pub const OVERFLOW_BIT : u8 = 0b10_0000;
pub const NEGATIVE_BIT: u8 = 0b100_0000;

pub fn set_carry(in_val : u16, mut carry: u8) -> u8 {
   
    if in_val > 255 {
        carry |= CARRY_BIT;
    }
  
    carry
}

pub fn set_zero(in_val : u16, mut zero: u8) -> u8 {
    
    if in_val == 0 {
        zero |= ZERO_BIT;
    }
            
    zero
}

pub fn set_negative(in_val : u16, mut negative: u8) -> u8 {
   
    if in_val & 128 != 0 {
        negative |= NEGATIVE_BIT;
    }
    
    negative
}

fn clear_flag(flags: &mut u8, bit_index: u8){
    *flags ^= bit_index;
}

fn clear_carry(mut flags: &mut u8){
    clear_flag(&mut flags, CARRY_BIT);
}

fn clear_overflow(mut flags: &mut u8){
    clear_flag(&mut flags, OVERFLOW_BIT);
}

fn clear_interrupt_disable(mut flags: &mut u8){
    clear_flag(&mut flags, INTERRUPT_DISABLE_BIT);
}

fn clear_decimal_mode(mut flags: &mut u8){
    clear_flag(&mut flags, DECIMAL_MODE_BIT);
}
