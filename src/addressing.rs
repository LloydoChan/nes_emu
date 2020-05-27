// addressing.rs - contains some simple addressing mode functions 
// so that they don't have to be repeated in all of the different operation types

//TODO page crossing and extra cycles?

use crate::memory::RAM;
use crate::flags::*;

pub enum Operation{
    Add,
    And,
}

pub fn swap_bytes(in_val : u16) -> u16 {
    let out_val = ( in_val << 8 ) | (in_val >> 8);
    out_val
}

fn match_on_op(mut in_val : u16, operand: u16, op : Operation) -> u16{

    match op{
        Operation::Add => in_val += operand,
        Operation::And => in_val &= operand,
        _=> panic!()
    }

    in_val
}

fn set_flags(in_val: u16, status_flag: Option<&mut u8>){
   
    match status_flag{
        Some(regs) => {
            if in_val == 0 { 
                set_zero(regs) 
            }
            if in_val > 256 {
                set_carry(regs)
            }
            if (in_val & 0x80) != 0 {
                set_negative(regs)
            }
            // TODO check "if sign bit is incorrect" for overflow
        },
         None => {}
    }
}

pub fn immediate(mut in_val : u16, operand: u16, status_flag: Option<&mut u8>, op : Operation) -> u8 {

    in_val = match_on_op(in_val, operand, op);
    set_flags(in_val, status_flag);
    in_val as u8
}

pub fn zero_page(mut in_val : u16, operand: u8, memory: &RAM, status_flag: Option<&mut u8>, op : Operation) -> u8 {
    let mem_value = memory.read_mem_value(operand as u16);

    in_val = match_on_op(in_val, mem_value as u16, op);
    set_flags(in_val, status_flag);
    in_val as u8
}

pub fn zero_page_x(mut in_val : u16, x_val : u8, operand: u8, memory: &RAM, status_flag: Option<&mut u8>, op : Operation) -> u8 {
    let addr = operand.wrapping_add(x_val);
    let mem_value = memory.read_mem_value(addr as u16);

    in_val = match_on_op(in_val, mem_value as u16, op);
    set_flags(in_val, status_flag);
    in_val as u8    
}

pub fn absolute(mut in_val : u16, operand: u16, memory: &RAM, status_flag: Option<&mut u8>, op : Operation) -> u8 {
    // little endian so swap around the operand bytes
    let addr_one : u16 = (operand >> 8) & 0xFF;
    let addr_two : u16 = operand & 0xFF;

    let addr : u16 = (addr_two << 8 ) | addr_one;
    let mem_value = memory.read_mem_value(addr as u16);

    in_val = match_on_op(in_val, mem_value as u16, op);
    set_flags(in_val, status_flag);

    in_val as u8    
}

// used for both x and y variants
pub fn absolute_reg(mut in_val : u16, reg : u16, operand: u16, memory: &RAM, status_flag: Option<&mut u8>, op : Operation) -> u8 {
    // little endian so swap around the operand bytes
    let addr = swap_bytes(operand);
    let mem_value = memory.read_mem_value(addr + reg as u16);

    in_val = match_on_op(in_val, mem_value as u16, op);
    set_flags(in_val, status_flag);

    in_val as u8    
}

pub fn indexed_indirect(mut in_val : u16, x_val : u8, operand: u8, memory: &RAM, status_flag: Option<&mut u8>, op : Operation) -> u8 {
    let table_addr = memory.read_mem_value(operand as u16);
    let addr = table_addr.wrapping_add(x_val);
    let mem_value = memory.read_mem_value(addr as u16);

    in_val = match_on_op(in_val, mem_value as u16, op);
    set_flags(in_val, status_flag);

    in_val as u8    
}

pub fn indirect_indexed(mut in_val : u16, y_val : u8, operand: u16, memory: &RAM, status_flag: Option<&mut u8>, op : Operation) -> u8 {
    let table_addr = memory.read_mem_value(operand as u16);
    let addr : u16 = table_addr as u16 + y_val as u16;
    let mem_value = memory.read_mem_value(addr as u16);

    in_val = match_on_op(in_val, mem_value as u16, op);
    set_flags(in_val, status_flag);

    in_val as u8    
}