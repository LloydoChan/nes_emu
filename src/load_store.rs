// load_store - load and store instructions on 6502
use crate::memory::*;
use crate::flags::*;

fn set_flags(in_val: u8, status_flag: &mut u8){
   
    if in_val == 0 { 
        set_zero(status_flag) 
    }

     if (in_val & 0x80) != 0 {
        set_negative(status_flag)
    }
}

// not just immediate load but also zero page and zero page x, y
pub fn immediate_load(operand: u8, offset: u8, memory: &RAM, status_flag: &mut u8) -> u8 {
    let addr = operand.wrapping_add(offset);
    let ret_val = memory.read_mem_value(addr as u16);
    set_flags(ret_val, status_flag);
    ret_val
}

pub fn store_zero_page(to_store: u8, operand: u8, offset: u8, memory: &mut RAM){
    let addr = operand.wrapping_add(offset);
    memory.write_mem_value(addr as u16, to_store);
}

pub fn store_absolute(to_store: u8, operand: u16, offset: u8, memory: &mut RAM){
    let addr = operand + offset as u16;
    memory.write_mem_value(addr as u16, to_store);
}

pub fn store_indirect_x(to_store: u8, operand: u8, offset: u8, memory: &mut RAM){
    let addr = operand.wrapping_add(offset);
    let write_addr = memory.read_mem_address(addr as u16);
    memory.write_mem_value(write_addr, to_store);
}

pub fn store_indirect_y(to_store: u8, operand: u16, offset: u8, memory: &mut RAM){
    let write_addr = memory.read_mem_address(operand) + offset as u16;
    memory.write_mem_value(write_addr, to_store);
}

pub fn absolute_load(operand: u16, offset: u16, memory: &RAM, status_flag: &mut u8) -> u8 {
    let addr = operand + offset;
    let ret_val = memory.read_mem_value(addr as u16);
    set_flags(ret_val, status_flag);
    ret_val
}

pub fn indirect_x_load(operand: u8, x_val: u8, memory: &RAM, status_flag: &mut u8) -> u8 {
    let addr = operand.wrapping_add(x_val);
    let table_addr = memory.read_mem_address(addr as u16);
    let mem_value = memory.read_mem_value(table_addr); 
    set_flags(mem_value, status_flag);
    mem_value
}

pub fn indirect_y_load(operand: u8, y_val: u8, memory: &RAM, status_flag: &mut u8) -> u8 {
    let table_addr = memory.read_mem_address(operand as u16) + y_val as u16; 
    let mem_value = memory.read_mem_value(table_addr); 
    set_flags(mem_value, status_flag);
    mem_value
}