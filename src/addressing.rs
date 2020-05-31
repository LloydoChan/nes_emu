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
    Ior,
    Load,
    Store
}


fn match_on_op(in_val : u8, operand: u8, op : Operation, status_flag: &mut u8) -> u8{

    let result = match op{
        Operation::Add => {
                            let carry = *status_flag & CARRY_BIT;
                            let result = in_val.wrapping_add(operand + carry);
                            result
                        },
        Operation::And => in_val & operand,
        Operation::Sub => {
                            let carry = *status_flag & CARRY_BIT;
                            let mut result = in_val.wrapping_sub(operand);
                            result = result.wrapping_sub(1-carry);
                            result
                        },
        Operation::Eor => in_val ^ operand,
        Operation::Ior => in_val | operand,
        _=> panic!()
    };

    match op{
        Operation::And | Operation::Eor |
        Operation::Ior => {
            set_flags_or_and(result, status_flag)
        },
        Operation::Add => {
            set_flags_add(in_val, operand, result, status_flag)
        },
        Operation::Sub => {
            set_flags_sub(in_val, operand, result, status_flag)
        }
        _=> panic!()
    }

    result
}

fn set_flags_add(in_val: u8, operand: u8, result: u8, status_flag: &mut u8){
   
    if result == 0 { 
        set_zero(status_flag) 
    }

    if (result & 0x80) != 0 {
        set_negative(status_flag)
    }

    // will overflow happen? assume use of carry flag
    if (operand & in_val & 0x80) != 0  && (result & 0x80) == 0{
       set_overflow(status_flag)
    }
   
    if (operand & in_val & 0x40) != 0  && (result & 0x80) != 0{
        set_overflow(status_flag)
    }

    // will carry happen?
    // assume that this is lower byte to generate carry, don't add current carry
    let (_, carry) = in_val.overflowing_add(operand);

    if carry {
        set_carry(status_flag)
    }   
}

fn set_flags_sub(in_val: u8, operand: u8, result: u8, status_flag: &mut u8){
   
    if result == 0 { 
        set_zero(status_flag) 
    }

    if (result & 0x80) != 0 {
        set_negative(status_flag)
    }
                            
    if (operand & in_val & 0x80) != 0  && (result & 0x80) == 0{
        set_overflow(status_flag)
    }
    
    if (operand & in_val & 0x40) != 0  && (result & 0x80) != 0{
        set_overflow(status_flag)
    }
                            

    let (_, carry) = in_val.overflowing_sub(operand);

    //clear carry if overflow
    if carry {
        clear_carry(status_flag);
    }
}


fn set_flags_or_and(in_val: u8, status_flag: &mut u8){
   
    if in_val == 0 { 
        set_zero(status_flag) 
    }

     if (in_val & 0x80) != 0 {
        set_negative(status_flag)
    }
}

pub fn immediate(mut in_val : u8, operand: u8, status_flag: &mut u8, op : Operation) -> u8 {
    in_val = match_on_op(in_val, operand, op, status_flag);
    in_val as u8
}

pub fn zero_page(mut in_val : u8, operand: u8, memory: &RAM, status_flag: &mut u8, op : Operation) -> u8 {
    let mem_value = memory.read_mem_value(operand as u16);
    in_val = match_on_op(in_val, mem_value, op, status_flag);
    in_val as u8
}

pub fn zero_page_x(mut in_val : u8, x_val : u8, operand: u8, memory: &RAM, status_flag: &mut u8, op : Operation) -> u8 {
    let addr = operand.wrapping_add(x_val);
    let mem_value = memory.read_mem_value(addr as u16);

    in_val = match_on_op(in_val, mem_value, op, status_flag);
    in_val as u8    
}


pub fn absolute(mut in_val : u8, operand: u16, memory: &RAM, status_flag: &mut u8, op : Operation) -> u8 {
    // little endian so swap around the operand bytes
    let mem_value = memory.read_mem_value(operand);
    in_val = match_on_op(in_val, mem_value, op, status_flag);
    in_val as u8    
}

// used for both x and y variants
pub fn absolute_reg(mut in_val : u8, reg : u8, operand: u16, memory: &RAM, status_flag: &mut u8, op : Operation) -> u8 {
    // little endian so swap around the operand bytes
    let mem_value = memory.read_mem_value(operand + reg as u16);
    in_val = match_on_op(in_val, mem_value, op, status_flag);
    in_val as u8    
}

pub fn indexed_indirect(mut in_val : u8, x_val : u8, operand: u8, memory: &RAM, status_flag: &mut u8, op : Operation) -> u8 {
    let addr = operand.wrapping_add(x_val);
    let table_addr = memory.read_mem_address(addr as u16);
    let mem_value = memory.read_mem_value(table_addr);

    in_val = match_on_op(in_val, mem_value, op, status_flag) as u8;

    in_val as u8    
}

pub fn indirect_indexed(mut in_val : u8, y_val : u8, operand: u8, memory: &RAM, status_flag: &mut u8, op : Operation) -> u8 {
    let table_addr = memory.read_mem_address(operand as u16);
    let addr : u16 = table_addr + y_val as u16;
    let mem_value = memory.read_mem_value(addr as u16);

    in_val = match_on_op(in_val, mem_value, op, status_flag);

    in_val as u8    
}