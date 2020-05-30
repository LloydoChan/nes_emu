//and.rs - for logical AND instructions

use crate::addressing::{self, Operation};
use crate::memory::*;
use crate::flags;

pub fn and_immediate(operand : u8, pc_reg : &mut u16, accumulator: u8, status_flags: &mut u8, cycles_until_next : &mut u8){
    addressing::immediate(accumulator as u16, operand as u16, status_flags, Operation::And);
    *pc_reg += 2;
    *cycles_until_next = 2;
}

pub fn and_zero_page(operand : u8, pc_reg : &mut u16, accumulator: u8, status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    addressing::zero_page(accumulator as u16, operand, memory, status_flags, Operation::And);
    *pc_reg += 2;
    *cycles_until_next = 3;
}

pub fn and_zero_page_x(operand : u8, x_reg : u8, pc_reg : &mut u16, accumulator: u8, status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    addressing::zero_page_x(accumulator as u16, x_reg, operand, memory,  status_flags, Operation::And);
    *pc_reg += 2;
    *cycles_until_next = 4;
}

pub fn and_absolute(operand : u16, pc_reg : &mut u16, accumulator: u8, status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    addressing::absolute(accumulator as u16, operand, memory, status_flags, Operation::And);
    *pc_reg += 3;
    *cycles_until_next = 4;
}

pub fn and_absolute_reg(operand : u16, reg : u8, pc_reg : &mut u16, accumulator: u8, status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    addressing::absolute_reg(accumulator as u16, reg as u16, operand, memory, status_flags, Operation::And);
    *pc_reg += 3;
    *cycles_until_next = 4;
}

pub fn and_indexed_indirect(operand : u8, x_val : u8, pc_reg : &mut u16, accumulator: u8, status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    addressing::indexed_indirect(accumulator, x_val, operand, memory, status_flags, Operation::And);
    *pc_reg += 2;
    *cycles_until_next = 6;
}

pub fn and_indirect_indexed(operand : u16, y_val : u8, pc_reg : &mut u16, accumulator:  u8, status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    addressing::indirect_indexed(accumulator as u16, y_val, operand, memory, status_flags, Operation::And);
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
        let mut status : u8 = 0;
        let mut test_memory  : memory::RAM = memory::RAM::new();
        let mut cycles = 0;

        // init mem
        for i in 0..2048 {
            test_memory.write_mem_value(i, i  as u8);
        }

        and_immediate(15, &mut pc_reg, 15,  &mut status, &mut cycles);
        assert_eq!(pc_reg, 2);
        assert_eq!(status , 0);

        and_immediate(0, &mut pc_reg, 15,  &mut status, &mut cycles);
        assert_eq!(pc_reg, 4);
        assert_eq!(status , 2);

        status = 0;
        and_zero_page(128, &mut pc_reg, 128, &mut status, &mut test_memory, &mut cycles);
        assert_eq!(pc_reg, 6);
        assert_eq!(status , 64);

        status = 0;
        and_zero_page_x(6, 122, &mut pc_reg, 128, &mut status, &mut test_memory, &mut cycles);
        assert_eq!(pc_reg, 8);
        assert_eq!(status, 64);

        status = 0;
        and_zero_page_x(3, 254, &mut pc_reg, 1, &mut status, &mut test_memory, &mut cycles);
        assert_eq!(pc_reg, 10);
        assert_eq!(status, 0);

        status = 0;
        and_absolute(0x01, &mut pc_reg, 1, &mut status, &test_memory, &mut cycles);

        assert_eq!(pc_reg, 13);
        assert_eq!(status, 0);

        and_absolute_reg(255, 2, &mut pc_reg, 1, &mut status, &test_memory, &mut cycles);

        assert_eq!(pc_reg, 16);
        assert_eq!(status, 0);

        status = 0;
        and_indexed_indirect(1, 3, &mut pc_reg, 4, &mut status, &test_memory, &mut cycles);

        assert_eq!(pc_reg, 18);
        assert_eq!(status, 0);

        // and_indirect_indexed(127, 3, &mut pc_reg, accumulator, &mut status, &test_memory, &mut cycles);

        // assert_eq!(pc_reg, 16);
        // assert_eq!(accumulator, 2);
    }   
}
