//and.rs - for logical AND instructions

use super::addressing::{self, Operation};
use crate::memory::*;
use super::flags;

pub fn and_immediate(operand : u8, pc_reg : &mut u16, accumulator: &mut u8, status_flags: &mut u8, cycles_until_next : &mut u8){
    *accumulator = addressing::immediate(*accumulator, operand, status_flags, Operation::And);
    *pc_reg += 2;
    *cycles_until_next = 2;
}

pub fn and_zero_page(operand : u8, pc_reg : &mut u16, accumulator: &mut u8, status_flags: &mut u8, memory : &mut RAM, cycles_until_next : &mut u8){
    *accumulator = addressing::zero_page(*accumulator, operand, memory, status_flags, Operation::And);
    *pc_reg += 2;
    *cycles_until_next = 3;
}

pub fn and_zero_page_x(operand : u8, x_reg : u8, pc_reg : &mut u16, accumulator: &mut u8, status_flags: &mut u8, memory : &mut RAM, cycles_until_next : &mut u8){
    *accumulator = addressing::zero_page_x(*accumulator, x_reg, operand, memory,  status_flags, Operation::And);
    *pc_reg += 2;
    *cycles_until_next = 4;
}

pub fn and_absolute(operand : u16, pc_reg : &mut u16, accumulator: &mut u8, status_flags: &mut u8, memory : &mut RAM, cycles_until_next : &mut u8){
    *accumulator = addressing::absolute(*accumulator, operand, memory, status_flags, Operation::And);
    *pc_reg += 3;
    *cycles_until_next = 4;
}

pub fn and_absolute_reg(operand : u16, reg : u8, pc_reg : &mut u16, accumulator: &mut u8, status_flags: &mut u8, memory : &mut RAM, cycles_until_next : &mut u8){
    *accumulator = addressing::absolute_reg(*accumulator, reg, operand, memory, status_flags, Operation::And);
    *pc_reg += 3;
    *cycles_until_next = 4;
}

pub fn and_indexed_indirect(operand : u8, x_val : u8, pc_reg : &mut u16, accumulator: &mut u8, status_flags: &mut u8, memory : &mut RAM, cycles_until_next : &mut u8){
    *accumulator = addressing::indexed_indirect(*accumulator, x_val, operand, memory, status_flags, Operation::And);
    *pc_reg += 2;
    *cycles_until_next = 6;
}

pub fn and_indirect_indexed(operand : u8, y_val : u8, pc_reg : &mut u16, accumulator:  &mut u8, status_flags: &mut u8, memory : &mut RAM, cycles_until_next : &mut u8){
    *accumulator = addressing::indirect_indexed(*accumulator, y_val, operand, memory, status_flags, Operation::And);
    *pc_reg += 2;
    *cycles_until_next = 5;
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::memory;

    #[test]
    fn tests() {
        let operand = 7;
        let mut pc_reg  = 0;
        let mut accumulator = 25;
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

        and_immediate(15, &mut pc_reg, &mut accumulator,  &mut status, &mut cycles);
        assert_eq!(pc_reg, 2);
        assert_eq!(status , 0);

        and_immediate(0, &mut pc_reg, &mut accumulator,  &mut status, &mut cycles);
        assert_eq!(pc_reg, 4);
        assert_eq!(status , 2);

        status = 0;
        accumulator = 64;
        and_zero_page(129, &mut pc_reg, &mut accumulator, &mut status, &mut test_memory, &mut cycles);
        assert_eq!(accumulator, 64);
        assert_eq!(pc_reg, 6);
        assert_eq!(status , 0);

        status = 0;
        and_zero_page_x(126, 3, &mut pc_reg, &mut accumulator, &mut status, &mut test_memory, &mut cycles);
        assert_eq!(pc_reg, 8);
        assert_eq!(status, 0);

        status = 0;
        and_absolute(3, &mut pc_reg, &mut accumulator, &mut status, &test_memory, &mut cycles);

        assert_eq!(pc_reg, 11);
        assert_eq!(status, 2);

        status = 0;
        accumulator = 129;
        and_absolute_reg(255, 2, &mut pc_reg, &mut accumulator, &mut status, &test_memory, &mut cycles);

        assert_eq!(pc_reg, 14);
        assert_eq!(status, 64);

        status = 0;
        accumulator = 1;
        and_indexed_indirect(1, 6, &mut pc_reg, &mut accumulator, &mut status, &test_memory, &mut cycles);

        assert_eq!(pc_reg, 16);
        assert_eq!(status, 0);

        and_indirect_indexed(1, 3, &mut pc_reg, &mut accumulator, &mut status, &test_memory, &mut cycles);

        assert_eq!(pc_reg, 18);
    }   
}
