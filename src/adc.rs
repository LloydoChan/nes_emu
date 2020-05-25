// adc.rs - contains all 8 functions to support adc instructions

// immediate

// TODO change mem reads! so that they use mem control / mapper correctly

use crate::addressing::{self, Operation};
use crate::memory::RAM;


pub fn adc_immediate(operand : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8){
    let temp = *accumulator as u16;
    *accumulator = addressing::immediate(temp as u16, operand as u16, Some(&mut status_flags), Operation::Add);
    *pc_reg += 1;
}

pub fn adc_zero_page(operand : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM){
    let temp = *accumulator as u16;
    *accumulator = addressing::zero_page(temp, operand, memory, Some(&mut status_flags), Operation::Add);
    *pc_reg += 1;
}

pub fn adc_zero_page_x(operand : u8, x_reg : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM){
    let temp = *accumulator as u16;
    *accumulator = addressing::zero_page_x(temp, x_reg, operand, memory, Some(&mut status_flags), Operation::Add);
    *pc_reg += 1;
}

pub fn adc_absolute(operand : u16, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM){
    let temp = *accumulator as u16;
    *accumulator = addressing::absolute(temp, operand, memory, Some(&mut status_flags), Operation::Add);
    *pc_reg += 3;
}

pub fn adc_absolute_reg(operand : u16, reg : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM){
    let temp = *accumulator as u16;
    *accumulator = addressing::absolute_reg(temp, reg as u16, operand, memory, Some(&mut status_flags), Operation::Add);
    *pc_reg += 3;
}

pub fn adc_indexed_indirect(operand : u8, x_val : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM){
    let temp = *accumulator as u16;
    *accumulator = addressing::indexed_indirect(temp, x_val, operand, memory, Some(&mut status_flags), Operation::Add);
    *pc_reg += 2;
}

pub fn adc_indirect_indexed(operand : u16, y_val : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM){
    let temp = *accumulator as u16;
    *accumulator = addressing::indirect_indexed(temp, y_val, operand, memory, Some(&mut status_flags), Operation::Add);
    *pc_reg += 2;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory;

    #[test]
    fn adc_tests() {
        let operand = 12;
        let mut pc_reg  = 1;
        let mut accumulator = 0;
        let mut status : u8 = 0;
        let mut test_memory  : memory::RAM = memory::RAM::new();

        // init mem
        for i in 0..2048 {
            test_memory.write_mem_value(i, (i * 2) as u8);
        }

        adc_immediate(operand, &mut pc_reg, &mut accumulator,  &mut status);

        assert_eq!(pc_reg, 2);
        assert_eq!(accumulator, 12);
        assert_eq!(status, 0);

        let operand2 = 255;
        adc_immediate(operand2, &mut pc_reg, &mut accumulator,  &mut status);

        assert_eq!(pc_reg, 3);
        assert_eq!(accumulator, 11);
        assert_eq!(status, 1);

        status = 0;

        adc_zero_page(12, &mut pc_reg, &mut accumulator,  &mut status, &test_memory);

        assert_eq!(pc_reg, 4);
        assert_eq!(accumulator, 35);
        assert_eq!(status, 0);

        adc_zero_page(111, &mut pc_reg, &mut accumulator,  &mut status, &test_memory);

        assert_eq!(pc_reg, 5);
        assert_eq!(accumulator, 1);
        assert_eq!(status, 1);

        accumulator = 0;
        status = 0;

        adc_zero_page_x(255, 2, &mut pc_reg, &mut accumulator, &mut status, &test_memory);

        assert_eq!(pc_reg, 6);
        assert_eq!(accumulator, 2);

        adc_absolute(257, &mut pc_reg, &mut accumulator, &mut status, &test_memory);

        assert_eq!(pc_reg, 9);
        assert_eq!(accumulator, 4);

        adc_absolute_reg(257, 2, &mut pc_reg, &mut accumulator, &mut status, &test_memory);

        assert_eq!(pc_reg, 12);
        assert_eq!(accumulator, 10);

        adc_indexed_indirect(127, 3, &mut pc_reg, &mut accumulator, &mut status, &test_memory);

        assert_eq!(pc_reg, 14);
        assert_eq!(accumulator, 12);

        adc_indirect_indexed(127, 3, &mut pc_reg, &mut accumulator, &mut status, &test_memory);

        assert_eq!(pc_reg, 16);
        assert_eq!(accumulator, 14);
    }
}