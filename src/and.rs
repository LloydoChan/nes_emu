//and.rs - for logical AND instructions

use crate::addressing::{self, Operation};
use crate::memory::*;
use crate::flags;

pub fn and_immediate(operand : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8){
    let temp = *accumulator as u16;
    *accumulator = addressing::immediate(temp as u16, operand as u16, None, Operation::And);
    *pc_reg += 2;
}

pub fn and_zero_page(operand : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM){
    let temp = *accumulator as u16;
    *accumulator = addressing::zero_page(temp, operand, memory, Some(&mut status_flags), Operation::And);
    *pc_reg += 2;
}

pub fn and_zero_page_x(operand : u8, x_reg : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM){
    let temp = *accumulator as u16;
    *accumulator = addressing::zero_page_x(temp, x_reg, operand, memory, Some(&mut status_flags), Operation::And);
    *pc_reg += 2;
}

pub fn and_absolute(operand : u16, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM){
    let temp = *accumulator as u16;
    *accumulator = addressing::absolute(temp, operand, memory, Some(&mut status_flags), Operation::And);
    *pc_reg += 3;
}

pub fn and_absolute_reg(operand : u16, reg : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM){
    let temp = *accumulator as u16;
    *accumulator = addressing::absolute_reg(temp, reg as u16, operand, memory, Some(&mut status_flags), Operation::And);
    *pc_reg += 3;
}

pub fn and_indexed_indirect(operand : u8, x_val : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM){
    let temp = *accumulator as u16;
    *accumulator = addressing::indexed_indirect(temp, x_val, operand, memory, Some(&mut status_flags), Operation::And);
    *pc_reg += 2;
}

pub fn and_indirect_indexed(operand : u16, y_val : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM){
    let temp = *accumulator as u16;
    *accumulator = addressing::indirect_indexed(temp, y_val, operand, memory, Some(&mut status_flags), Operation::And);
    *pc_reg += 2;
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::memory;

    #[test]
    fn tests() {
        let operand = 7;
        let mut pc_reg  = 0;
        let mut accumulator = 15;
        let mut status : u8 = 0;
        let mut test_memory  : memory::RAM = memory::RAM::new();

        // init mem
        for i in 0..2048 {
            test_memory.write_mem_value(i, (i * 2) as u8);
        }

        and_immediate(operand, &mut pc_reg, &mut accumulator,  &mut status);
        assert_eq!(pc_reg, 2);
        assert_eq!(accumulator, 7);

        and_zero_page(0, &mut pc_reg, &mut accumulator, &mut status, &mut test_memory);
        assert_eq!(pc_reg, 4);
        assert_eq!(accumulator, 0);
        assert_eq!(status & flags::ZERO_BIT, flags::ZERO_BIT);

        accumulator = 244;

        and_zero_page_x(0, 122, &mut pc_reg, &mut accumulator, &mut status, &mut test_memory);
        assert_eq!(pc_reg, 6);
        assert_eq!(accumulator, 244);
        assert_eq!(status & flags::NEGATIVE_BIT, flags::NEGATIVE_BIT);

        let operand = 514;
        let new_op = swap_bytes(operand);
        status = 0;

        and_absolute(new_op, &mut pc_reg, &mut accumulator, &mut status, &test_memory);

        assert_eq!(pc_reg, 9);
        assert_eq!(accumulator, 4);
        assert_eq!(status & flags::NEGATIVE_BIT, 0);
        assert_eq!(status & flags::ZERO_BIT, 0);

        and_absolute_reg(new_op, 2, &mut pc_reg, &mut accumulator, &mut status, &test_memory);

        assert_eq!(pc_reg, 12);
        assert_eq!(accumulator, 0);
        assert_eq!(status & flags::ZERO_BIT, flags::ZERO_BIT);

        status = 0;
        accumulator = 2;
        and_indexed_indirect(127, 3, &mut pc_reg, &mut accumulator, &mut status, &test_memory);

        assert_eq!(pc_reg, 14);
        assert_eq!(accumulator, 2);

        and_indirect_indexed(127, 3, &mut pc_reg, &mut accumulator, &mut status, &test_memory);

        assert_eq!(pc_reg, 16);
        assert_eq!(accumulator, 2);
    }   
}
