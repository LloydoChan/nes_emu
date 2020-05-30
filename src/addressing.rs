// addressing.rs - contains some simple addressing mode functions 
// so that they don't have to be repeated in all of the different operation types

//TODO page crossing and extra cycles?

use crate::memory::*;
use crate::flags::*;

pub enum Operation{
    Add,
    Sub,
    And,
    Eor,
    Ior
}


fn match_on_op(mut in_val : u16, operand: u16, op : Operation, status_flag: &mut u8) -> u16{

    match op{
        Operation::Add => in_val += operand,
        Operation::And => in_val &= operand,
        Operation::Sub => in_val -= operand,
        Operation::Eor => in_val ^= operand,
        Operation::Ior => in_val |= operand,
        _=> panic!()
    }

    match op{
        Operation::And | Operation::Eor |
        Operation::Ior => {
            set_flags_or_and(in_val, status_flag)
        },
        Operation::Sub | Operation::Add => {
            set_flags_add_sub(in_val, status_flag)
        }
        _=> panic!()
    }

    in_val
}

fn set_flags_add_sub(in_val: u16, status_flag: &mut u8){
   
    if in_val == 0 { 
        set_zero(status_flag) 
    }

    if in_val > 256 {
         set_carry(status_flag)
    }

     if (in_val & 0x80) != 0 {
        set_negative(status_flag)
    }

    // TODO check "if sign bit is incorrect" for overflow
      
}

fn set_flags_or_and(in_val: u16, status_flag: &mut u8){
   
    if in_val == 0 { 
        set_zero(status_flag) 
    }

     if (in_val & 0x80) != 0 {
        set_negative(status_flag)
    }
}

pub fn immediate(mut in_val : u16, operand: u16, status_flag: &mut u8, op : Operation) -> u8 {

    in_val = match_on_op(in_val, operand, op, status_flag);
    in_val as u8
}

pub fn zero_page(mut in_val : u16, operand: u8, memory: &RAM, status_flag: &mut u8, op : Operation) -> u8 {
    let mem_value = memory.read_mem_value(operand as u16);

    in_val = match_on_op(in_val, mem_value as u16, op, status_flag);
    in_val as u8
}

pub fn zero_page_x(mut in_val : u16, x_val : u8, operand: u8, memory: &RAM, status_flag: &mut u8, op : Operation) -> u8 {
    let addr = operand.wrapping_add(x_val);
    let mem_value = memory.read_mem_value(addr as u16);

    in_val = match_on_op(in_val, mem_value as u16, op, status_flag);
    in_val as u8    
}

pub fn absolute(mut in_val : u16, operand: u16, memory: &RAM, status_flag: &mut u8, op : Operation) -> u8 {
    // little endian so swap around the operand bytes
    let mem_value = memory.read_mem_value(operand as u16);
    in_val = match_on_op(in_val, mem_value as u16, op, status_flag);
    in_val as u8    
}

// used for both x and y variants
pub fn absolute_reg(mut in_val : u16, reg : u16, operand: u16, memory: &RAM, status_flag: &mut u8, op : Operation) -> u8 {
    // little endian so swap around the operand bytes
    let mem_value = memory.read_mem_value(operand + reg as u16);
    in_val = match_on_op(in_val, mem_value as u16, op, status_flag);
    in_val as u8    
}

pub fn indexed_indirect(mut in_val : u8, x_val : u8, operand: u8, memory: &RAM, status_flag: &mut u8, op : Operation) -> u8 {
    let addr = operand.wrapping_add(x_val);
    let table_addr = memory.read_mem_address(addr as u16);
    let mem_value = memory.read_mem_value(table_addr);

    in_val = match_on_op(in_val as u16, mem_value as u16, op, status_flag) as u8;

    in_val as u8    
}

pub fn indirect_indexed(mut in_val : u16, y_val : u8, operand: u16, memory: &RAM, status_flag: &mut u8, op : Operation) -> u8 {
    let table_addr = memory.read_mem_address(operand as u16);
    let addr : u16 = table_addr as u16 + y_val as u16;
    let mem_value = memory.read_mem_value(addr as u16);

    in_val = match_on_op(in_val, mem_value as u16, op, status_flag);

    in_val as u8    
}