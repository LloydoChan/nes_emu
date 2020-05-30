//or.rs - opcodes for xor and inclusive or dealt with!

use crate::addressing::{self, Operation};
use crate::memory::RAM;


pub fn xor_immediate(operand : u8, pc_reg : &mut u16, accumulator: u8, mut status_flags: &mut u8, cycles_until_next : &mut u8){
    addressing::immediate(accumulator as u16, operand as u16, &mut status_flags, Operation::Eor);
    *pc_reg += 2;
    *cycles_until_next = 2;
}

pub fn xor_zero_page(operand : u8, pc_reg : &mut u16, accumulator: u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    addressing::zero_page(accumulator as u16, operand, memory, &mut status_flags, Operation::Eor);
    *pc_reg += 2;
    *cycles_until_next = 3;
}

pub fn xor_zero_page_x(operand : u8, x_reg : u8, pc_reg : &mut u16, accumulator: u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    addressing::zero_page_x(accumulator as u16, x_reg, operand, memory, &mut status_flags, Operation::Eor);
    *pc_reg += 2;
    *cycles_until_next = 4;
}

pub fn xor_absolute(operand : u16, pc_reg : &mut u16, accumulator: u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    addressing::absolute(accumulator as u16, operand, memory, &mut status_flags, Operation::Eor);
    *pc_reg += 3;
    *cycles_until_next = 4;
}

pub fn xor_absolute_reg(operand : u16, reg : u8, pc_reg : &mut u16, accumulator: u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    addressing::absolute_reg(accumulator as u16, reg as u16, operand, memory, &mut status_flags, Operation::Eor);
    *pc_reg += 3;
    *cycles_until_next = 4;
}

pub fn xor_indexed_indirect(operand : u8, x_val : u8, pc_reg : &mut u16, accumulator: u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    addressing::indexed_indirect(accumulator, x_val, operand, memory, &mut status_flags, Operation::Eor);
    *pc_reg += 2;
    *cycles_until_next = 6;
}

pub fn xor_indirect_indexed(operand : u16, y_val : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    let temp = *accumulator as u16;
    *accumulator = addressing::indirect_indexed(temp, y_val, operand, memory, &mut status_flags, Operation::Eor);
    *pc_reg += 2;
    *cycles_until_next = 5;
}

pub fn ior_immediate(operand : u8, pc_reg : &mut u16, accumulator: u8, mut status_flags: &mut u8, cycles_until_next : &mut u8){
    addressing::immediate(accumulator as u16, operand as u16, &mut status_flags, Operation::Ior);
    *pc_reg += 2;
    *cycles_until_next = 2;
}

pub fn ior_zero_page(operand : u8, pc_reg : &mut u16, accumulator: u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    addressing::zero_page(accumulator as u16, operand, memory, &mut status_flags, Operation::Ior);
    *pc_reg += 2;
    *cycles_until_next = 3;
}

pub fn ior_zero_page_x(operand : u8, x_reg : u8, pc_reg : &mut u16, accumulator: u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    addressing::zero_page_x(accumulator as u16, x_reg, operand, memory, &mut status_flags, Operation::Ior);
    *pc_reg += 2;
    *cycles_until_next = 4;
}

pub fn ior_absolute(operand : u16, pc_reg : &mut u16, accumulator: u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    addressing::absolute(accumulator as u16, operand, memory, &mut status_flags, Operation::Ior);
    *pc_reg += 3;
    *cycles_until_next = 4;
}

pub fn ior_absolute_reg(operand : u16, reg : u8, pc_reg : &mut u16, accumulator: u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    addressing::absolute_reg(accumulator as u16, reg as u16, operand, memory, &mut status_flags, Operation::Ior);
    *pc_reg += 3;
    *cycles_until_next = 4;
}

pub fn ior_indexed_indirect(operand : u8, x_val : u8, pc_reg : &mut u16, accumulator: u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    addressing::indexed_indirect(accumulator, x_val, operand, memory, &mut status_flags, Operation::Ior);
    *pc_reg += 2;
    *cycles_until_next = 6;
}

pub fn ior_indirect_indexed(operand : u16, y_val : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    let temp = *accumulator as u16;
    *accumulator = addressing::indirect_indexed(temp, y_val, operand, memory, &mut status_flags, Operation::Ior);
    *pc_reg += 2;
    *cycles_until_next = 5;
}

#[cfg(test)]
mod tests{
    #[test]
    pub fn or_tests(){

    }
}