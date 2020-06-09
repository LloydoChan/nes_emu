//or.rs - opcodes for xor and inclusive or dealt with!

use super::addressing::{self, Operation};
use crate::memory::{RAM, *};


pub fn xor_immediate(operand : u8, pc_reg : &mut u16, accumulator:  &mut u8, mut status_flags: &mut u8, cycles_until_next : &mut u8){
    *accumulator = addressing::immediate(*accumulator, operand, &mut status_flags, Operation::Eor);
    *pc_reg += 2;
    *cycles_until_next = 2;
}

pub fn xor_zero_page(operand : u8, pc_reg : &mut u16, accumulator:  &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    *accumulator = addressing::zero_page(*accumulator, operand, memory, &mut status_flags, Operation::Eor);
    *pc_reg += 2;
    *cycles_until_next = 3;
}

pub fn xor_zero_page_x(operand : u8, x_reg : u8, pc_reg : &mut u16, accumulator:  &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    *accumulator = addressing::zero_page_x(*accumulator, x_reg, operand, memory, &mut status_flags, Operation::Eor);
    *pc_reg += 2;
    *cycles_until_next = 4;
}

pub fn xor_absolute(operand : u16, pc_reg : &mut u16, accumulator:  &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    *accumulator = addressing::absolute(*accumulator, operand, memory, &mut status_flags, Operation::Eor);
    *pc_reg += 3;
    *cycles_until_next = 4;
}

pub fn xor_absolute_reg(operand : u16, reg : u8, pc_reg : &mut u16, accumulator:  &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    *accumulator = addressing::absolute_reg(*accumulator, reg, operand, memory, &mut status_flags, Operation::Eor);
    *pc_reg += 3;
    *cycles_until_next = 4;
}

pub fn xor_indexed_indirect(operand : u8, x_val : u8, pc_reg : &mut u16, accumulator:  &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    *accumulator = addressing::indexed_indirect(*accumulator, x_val, operand, memory, &mut status_flags, Operation::Eor);
    *pc_reg += 2;
    *cycles_until_next = 6;
}

pub fn xor_indirect_indexed(operand : u8, y_val : u8, pc_reg : &mut u16, accumulator:  &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    *accumulator = addressing::indirect_indexed(*accumulator, y_val, operand, memory, &mut status_flags, Operation::Eor);
    *pc_reg += 2;
    *cycles_until_next = 5;
}

pub fn ior_immediate(operand : u8, pc_reg : &mut u16, accumulator:  &mut u8, mut status_flags: &mut u8, cycles_until_next : &mut u8){
    *accumulator = addressing::immediate(*accumulator, operand, &mut status_flags, Operation::Ior);
    *pc_reg += 2;
    *cycles_until_next = 2;
}

pub fn ior_zero_page(operand : u8, pc_reg : &mut u16, accumulator:  &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    *accumulator = addressing::zero_page(*accumulator, operand, memory, &mut status_flags, Operation::Ior);
    *pc_reg += 2;
    *cycles_until_next = 3;
}

pub fn ior_zero_page_x(operand : u8, x_reg : u8, pc_reg : &mut u16, accumulator:  &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    *accumulator = addressing::zero_page_x(*accumulator, x_reg, operand, memory, &mut status_flags, Operation::Ior);
    *pc_reg += 2;
    *cycles_until_next = 4;
}

pub fn ior_absolute(operand : u16, pc_reg : &mut u16, accumulator:  &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    *accumulator = addressing::absolute(*accumulator, operand, memory, &mut status_flags, Operation::Ior);
    *pc_reg += 3;
    *cycles_until_next = 4;
}

pub fn ior_absolute_reg(operand : u16, reg : u8, pc_reg : &mut u16, accumulator:  &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    *accumulator = addressing::absolute_reg(*accumulator, reg, operand, memory, &mut status_flags, Operation::Ior);
    *pc_reg += 3;
    *cycles_until_next = 4;
}

pub fn ior_indexed_indirect(operand : u8, x_val : u8, pc_reg : &mut u16, accumulator:  &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    *accumulator = addressing::indexed_indirect(*accumulator, x_val, operand, memory, &mut status_flags, Operation::Ior);
    *pc_reg += 2;
    *cycles_until_next = 6;
}

pub fn ior_indirect_indexed(operand : u8, y_val : u8, pc_reg : &mut u16, accumulator:  &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    *accumulator = addressing::indirect_indexed(*accumulator, y_val, operand, memory, &mut status_flags, Operation::Ior);
    *pc_reg += 2;
    *cycles_until_next = 5;
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::memory;

    #[test]
    pub fn test_or(){
        let operand = 7;
        let mut pc_reg  = 0;
        let mut accumulator = 2;
        let mut status : u8 = 0;
        let mut test_memory  : memory::RAM = memory::RAM::new();
        let mut cycles = 0;

        // init mem
        for i in 0..512 {
            test_memory.write_mem_value(i*2, 0  as u8);
            test_memory.write_mem_value(i*2+1, i  as u8);
        }

        for i in 512..1024 {
            test_memory.write_mem_value(i, i  as u8);
        }

        ior_immediate(5, &mut pc_reg, &mut accumulator, &mut status, &mut cycles);

        assert_eq!(pc_reg, 2);
        assert_eq!(accumulator, 7);

        xor_immediate(7, &mut pc_reg, &mut accumulator, &mut status, &mut cycles);

        assert_eq!(pc_reg, 4);
        assert_eq!(accumulator, 0);
        assert_eq!(status, 2);

        status = 0;
        ior_zero_page(11, &mut pc_reg, &mut accumulator, &mut status, &mut test_memory, &mut cycles);

        assert_eq!(pc_reg, 6);
        assert_eq!(accumulator, 5);
        assert_eq!(status, 0);

        xor_zero_page(5, &mut pc_reg, &mut accumulator, &mut status, &mut test_memory, &mut cycles);

        assert_eq!(pc_reg, 8);
        assert_eq!(accumulator, 7);
        assert_eq!(status, 0);

        status = 0;
        ior_zero_page_x(11, 2, &mut pc_reg, &mut accumulator, &mut status, &mut test_memory, &mut cycles);

        assert_eq!(pc_reg, 10);
        assert_eq!(accumulator, 7);
        assert_eq!(status, 0);

        status = 0;
        accumulator = 0;
        xor_zero_page_x(11, 2, &mut pc_reg, &mut accumulator, &mut status, &mut test_memory, &mut cycles);

        assert_eq!(pc_reg, 12);
        assert_eq!(accumulator, 6);
        assert_eq!(status, 0);

        xor_zero_page_x(255, 14, &mut pc_reg, &mut accumulator, &mut status, &mut test_memory, &mut cycles);

        assert_eq!(pc_reg, 14);
        assert_eq!(accumulator, 0);
        assert_eq!(status, 2);

        status = 0; 
        accumulator = 0;
        ior_absolute(259, &mut pc_reg, &mut accumulator, &mut status, &mut test_memory, &mut cycles);

        assert_eq!(pc_reg, 17);
        assert_eq!(accumulator, 129);
        assert_eq!(status, 64);

        status = 0;
        xor_absolute(259, &mut pc_reg, &mut accumulator, &mut status, &mut test_memory, &mut cycles);

        assert_eq!(pc_reg, 20);
        assert_eq!(accumulator, 0);
        assert_eq!(status, 2);
    }
}