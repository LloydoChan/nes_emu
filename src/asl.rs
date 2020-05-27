// //asl.rs arithmetic shift left

use crate::memory::RAM;
use crate::shift_addr::*;
use crate::flags::*;


fn post_op_set_flags(in_val: u8, status_flags: &mut u8){
    if in_val == 0 {
        set_zero(status_flags);
    }

    if (in_val & 0x80) != 0 {
        set_negative(status_flags);
    }
}


pub fn asl_accumulator(pc_reg : &mut u16, accumulator: &mut u8, status_flags: &mut u8){
    
    if (*accumulator & 0x80) != 0 {
        set_carry(status_flags);
    }

    *accumulator <<= 1;
    post_op_set_flags(*accumulator, status_flags);
    *pc_reg += 1;
 }

 pub fn asl_zero_page(pc_reg : &mut u16, operand: u8, memory: &RAM, status_flags: &mut u8){

    let mut value = shift_zero_page(operand, memory);

    if (value & 0x80) != 0 {
        set_carry(status_flags);
    }

    value <<= 1;

    if (value & 0x80) != 0 {
        set_negative(status_flags);
    }

    *pc_reg += 2;
 }

 pub fn asl_zero_page_x(pc_reg : &mut u16, x_val : u8, operand: u8, memory: &RAM, status_flags: &mut u8){

    let mut value = shift_zero_page_x(x_val, operand, memory);

    if (value & 0x80) != 0 {
        set_carry(status_flags);
    }

    value <<= 1;

    if (value & 0x80) != 0 {
        set_negative(status_flags);
    }

    *pc_reg += 2;
 }

 pub fn asl_absolute(pc_reg : &mut u16, operand: u16, memory: &RAM, status_flags: &mut u8){
    let mut value = shift_absolute(operand, memory);

    if (value & 0x80) != 0 {
        set_carry(status_flags);
    }

    value <<= 1;

    if (value & 0x80) != 0 {
        set_negative(status_flags);
    }

    *pc_reg += 3;
 }

 pub fn asl_absolute_x(pc_reg : &mut u16, x_reg : u8, operand: u16, memory: &RAM, status_flags: &mut u8){
    let mut value = shift_absolute_x(x_reg, operand, memory);

    if (value & 0x80) != 0 {
        set_carry(status_flags);
    }

    value <<= 1;

    if (value & 0x80) != 0 {
        set_negative(status_flags);
    }

    *pc_reg += 3;
 }