// put addressing specific to ASL and RSL here
use crate::memory::*;

pub fn shift_zero_page(operand: u8, memory: &RAM) -> u8 {
    let mut mem_value = memory.read_mem_value(operand as u16);
    mem_value
}

pub fn shift_zero_page_x(x_val : u8, operand: u8, memory: &RAM) -> u8 {
    let addr = operand.wrapping_add(x_val);
    let mut mem_value = memory.read_mem_value(addr as u16);
    mem_value as u8    
}


pub fn shift_absolute(operand: u16, memory: &RAM) -> u8 {
    // little endian so swap around the operand bytes
    let addr = operand;
    let mut mem_value = memory.read_mem_value(addr as u16);

    mem_value as u8    
}

pub fn shift_absolute_x(reg : u8, operand: u16, memory: &RAM) -> u8 {
    // little endian so swap around the operand bytes
    let addr = operand;
    let mut mem_value = memory.read_mem_value(addr + reg as u16);
    mem_value    
}