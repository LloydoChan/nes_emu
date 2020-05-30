// adc.rs - contains all 8 functions to support adc instructions

use crate::addressing::{self, Operation};
use crate::memory::RAM;


pub fn adc_immediate(operand : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, cycles_until_next : &mut u8){
    let temp = *accumulator as u16;
    *accumulator = addressing::immediate(temp as u16, operand as u16, &mut status_flags, Operation::Add);
    *pc_reg += 2;
    *cycles_until_next = 2;
}

pub fn adc_zero_page(operand : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    let temp = *accumulator as u16;
    *accumulator = addressing::zero_page(temp, operand, memory, &mut status_flags, Operation::Add);
    *pc_reg += 2;
    *cycles_until_next = 3;
}

pub fn adc_zero_page_x(operand : u8, x_reg : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    let temp = *accumulator as u16;
    *accumulator = addressing::zero_page_x(temp, x_reg, operand, memory, &mut status_flags, Operation::Add);
    *pc_reg += 2;
    *cycles_until_next = 4;
}

pub fn adc_absolute(operand : u16, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    let temp = *accumulator as u16;
    *accumulator = addressing::absolute(temp, operand, memory, &mut status_flags, Operation::Add);
    *pc_reg += 3;
    *cycles_until_next = 4;
}

pub fn adc_absolute_reg(operand : u16, reg : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    let temp = *accumulator as u16;
    *accumulator = addressing::absolute_reg(temp, reg as u16, operand, memory, &mut status_flags, Operation::Add);
    *pc_reg += 3;
    *cycles_until_next = 4;
}

pub fn adc_indexed_indirect(operand : u8, x_val : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    *accumulator = addressing::indexed_indirect(*accumulator, x_val, operand, memory, &mut status_flags, Operation::Add);
    *pc_reg += 2;
    *cycles_until_next = 6;
}

pub fn adc_indirect_indexed(operand : u16, y_val : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8, memory : &RAM, cycles_until_next : &mut u8){
    let temp = *accumulator as u16;
    *accumulator = addressing::indirect_indexed(temp, y_val, operand, memory, &mut status_flags, Operation::Add);
    *pc_reg += 2;
    *cycles_until_next = 5;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory;

    #[test]
    fn adc_tests() {
        let operand = 12;
        let mut pc_reg  = 0;
        let mut accumulator = 0;
        let mut status : u8 = 0;
        let mut test_memory  : memory::RAM = memory::RAM::new();

        let mut cycles = 0;

        // init mem
        for i in 0..2048 {
            test_memory.write_mem_value(i, (i * 2) as u8);
        }

        adc_immediate(operand, &mut pc_reg, &mut accumulator,  &mut status, &mut cycles);

        assert_eq!(pc_reg, 2);
        assert_eq!(accumulator, 12);
        assert_eq!(status, 0);

        let operand2 = 230;
        adc_immediate(operand2, &mut pc_reg, &mut accumulator,  &mut status, &mut cycles);

        assert_eq!(pc_reg, 4);
        assert_eq!(accumulator, 242);

        //-ve should be set
        assert_eq!(status, 0x40);

        accumulator = 0;
        status = 0;

        adc_zero_page(12, &mut pc_reg, &mut accumulator,  &mut status, &test_memory, &mut cycles);

        assert_eq!(pc_reg, 6);
        assert_eq!(accumulator, 24);
        assert_eq!(status, 0);

        adc_zero_page(120, &mut pc_reg, &mut accumulator,  &mut status, &test_memory, &mut cycles);

        assert_eq!(pc_reg, 8);
        assert_eq!(accumulator, 8);
        assert_eq!(status, 1);

        accumulator = 0;
        status = 0;

        adc_zero_page(0, &mut pc_reg, &mut accumulator,  &mut status, &test_memory, &mut cycles);

        assert_eq!(pc_reg, 10);
        assert_eq!(accumulator, 0);
        assert_eq!(status, 2);

        accumulator = 0;
        status = 0;

        adc_zero_page_x(255, 2, &mut pc_reg, &mut accumulator, &mut status, &test_memory, &mut cycles);

        assert_eq!(pc_reg, 12);
        assert_eq!(accumulator, 2);

        adc_absolute(257, &mut pc_reg, &mut accumulator, &mut status, &test_memory, &mut cycles);

        assert_eq!(pc_reg, 15);
        assert_eq!(accumulator, 4);

        adc_absolute_reg(257, 2, &mut pc_reg, &mut accumulator, &mut status, &test_memory, &mut cycles);

        assert_eq!(pc_reg, 18);
        assert_eq!(accumulator, 10);
        accumulator = 0;
        adc_indexed_indirect(127, 3, &mut pc_reg, &mut accumulator, &mut status, &test_memory, &mut cycles);

        assert_eq!(pc_reg, 20);
        assert_eq!(accumulator, 8);

        accumulator = 0;
        adc_indirect_indexed(1, 3, &mut pc_reg, &mut accumulator, &mut status, &test_memory, &mut cycles);

        assert_eq!(pc_reg, 22);
        assert_eq!(accumulator, 10);
    }
}