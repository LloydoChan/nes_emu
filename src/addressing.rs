// addressing.rs - contains some simple addressing mode functions 
// so that they don't have to be repeated in all of the different operation types

use crate::memory::RAM;

pub enum Operation{
    Add,
    And
}

fn set_carry(in_val : u16, status_flag: Option<&mut u8>) {
    match status_flag {
        Some(carry) => {
            if in_val > 255 {
                *carry |= 1;
            }
        },
        None => {}
    }
}

fn set_zero(in_val : u16, status_flag: Option<&mut u8>) {
    match status_flag {
        Some(zero) => {
            if in_val == 0 {
                *zero |= 2;
            }
        },
        None => {}
    }
}

fn set_negative(in_val : u16, status_flag: Option<&mut u8>) {
    match status_flag {
        Some(negative) => {
            if in_val & 128 != 0 {
                *negative |= 64;
            }
        },
        None => {}
    }
}

fn match_on_op(mut in_val : u16, operand: u16, op : Operation) -> u16{
    match op{
        Operation::Add => in_val += operand,
        Operation::And => in_val &= operand,
        _=> panic!()
    }

    in_val
}

pub fn immediate(mut in_val : u16, operand: u16, status_flag: Option<&mut u8>, op : Operation) -> u8 {

    in_val = match_on_op(in_val, operand, op);
    set_carry(in_val, status_flag);
    in_val as u8
}

pub fn zero_page(mut in_val : u16, operand: u8, memory: &RAM, status_flag: Option<&mut u8>, op : Operation) -> u8 {
    let mem_value = memory.read_mem_value(operand as u16);

    in_val = match_on_op(in_val, mem_value as u16, op);
    set_carry(in_val, status_flag);
    in_val as u8
}

pub fn zero_page_x(mut in_val : u16, x_val : u8, operand: u8, memory: &RAM, status_flag: Option<&mut u8>, op : Operation) -> u8 {
    let addr = operand.wrapping_add(x_val);
    let mem_value = memory.read_mem_value(addr as u16);

    in_val = match_on_op(in_val, mem_value as u16, op);
    set_carry(in_val, status_flag);
    in_val as u8    
}

pub fn absolute(mut in_val : u16, operand: u16, memory: &RAM, status_flag: Option<&mut u8>, op : Operation) -> u8 {
    // little endian so swap around the operand bytes
    let addr_one : u16 = (operand >> 8) & 0xFF;
    let addr_two : u16 = operand & 0xFF;
    let addr : u16 = addr_one | addr_two;
    let mem_value = memory.read_mem_value(addr as u16);

    in_val = match_on_op(in_val, mem_value as u16, op);
    set_carry(in_val, status_flag);

    in_val as u8    
}

// used for both x and y variants
pub fn absolute_reg(mut in_val : u16, reg : u16, operand: u16, memory: &RAM, status_flag: Option<&mut u8>, op : Operation) -> u8 {
    // little endian so swap around the operand bytes
    let addr_one : u16 = (operand >> 8) & 0xFF;
    let addr_two : u16 = operand & 0xFF;
    let addr : u16 = addr_one | addr_two;
    let mem_value = memory.read_mem_value(addr + reg as u16);

    in_val = match_on_op(in_val, mem_value as u16, op);
    set_carry(in_val, status_flag);

    in_val as u8    
}

pub fn indexed_indirect(mut in_val : u16, x_val : u8, operand: u8, memory: &RAM, status_flag: Option<&mut u8>, op : Operation) -> u8 {
    let table_addr = memory.read_mem_value(operand as u16);
    let addr = table_addr.wrapping_add(x_val);
    let mem_value = memory.read_mem_value(addr as u16);

    in_val = match_on_op(in_val, mem_value as u16, op);
    set_carry(in_val, status_flag);

    in_val as u8    
}

pub fn indirect_indexed(mut in_val : u16, y_val : u8, operand: u16, memory: &RAM, status_flag: Option<&mut u8>, op : Operation) -> u8 {
    let table_addr = memory.read_mem_value(operand as u16);
    let addr : u16 = table_addr as u16 + y_val as u16;
    let mem_value = memory.read_mem_value(addr as u16);

    in_val = match_on_op(in_val, mem_value as u16, op);
    set_carry(in_val, status_flag);

    in_val as u8    
}