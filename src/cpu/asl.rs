//asl.rs arithmetic shift left

use crate::memory::*;
use super::shift_addr::*;
use super::flags::*;


pub fn asl_accumulator(pc_reg : &mut u16, accumulator: &mut u8, status_flags: &mut u8, cycles_until_next : &mut u8){
    
    if (*accumulator & 0x80) != 0 {
        set_carry(status_flags);
    } else {
        clear_carry(status_flags);
    }

    *accumulator <<= 1;
   
    if (*accumulator & 0x80) != 0 {
        set_negative(status_flags);
    } else {
        clear_negative(status_flags);
    }

    if *accumulator == 0 {
        set_zero(status_flags);
    } else {
        clear_zero(status_flags);
    }

    *pc_reg += 1;
    *cycles_until_next = 2;
 }

 pub fn asl_zero_page(pc_reg : &mut u16, operand: u8, memory: &mut RAM, status_flags: &mut u8, cycles_until_next : &mut u8){

    let mut value = shift_zero_page(operand, memory);

    if (value & 0x80) != 0 {
        set_carry(status_flags);
    } else {
        clear_carry(status_flags);
    }

    value <<= 1;
    memory.write_mem_value(operand as u16, value);

    if (value & 0x80) != 0 {
        set_negative(status_flags);
    } else {
        clear_negative(status_flags);
    }

    if value == 0 {
        set_zero(status_flags);
    } else {
        clear_zero(status_flags);
    }

    *pc_reg += 2;
    *cycles_until_next = 5;
 }

 pub fn asl_zero_page_x(pc_reg : &mut u16, x_val : u8, operand: u8, memory: &mut RAM, status_flags: &mut u8, cycles_until_next : &mut u8){

    let mut value = shift_zero_page_x(x_val, operand, memory);

    if (value & 0x80) != 0 {
        set_carry(status_flags);
    } else {
        clear_carry(status_flags);
    }

    value <<= 1;
    memory.write_mem_value(operand.wrapping_add(x_val) as u16, value);

    if (value & 0x80) != 0 {
        set_negative(status_flags);
    } else {
        clear_negative(status_flags);
    }

    if value == 0 {
        set_zero(status_flags);
    } else {
        clear_zero(status_flags);
    }

    *pc_reg += 2;
    *cycles_until_next = 6;
 }

 pub fn asl_absolute(pc_reg : &mut u16, operand: u16, memory: &mut RAM, status_flags: &mut u8, cycles_until_next : &mut u8){
    let mut value = shift_absolute(operand, memory);

    if (value & 0x80) != 0 {
        set_carry(status_flags);
    } else {
        clear_carry(status_flags);
    }

    value <<= 1;
    let addr = operand;
    memory.write_mem_value(addr as u16, value);

    if (value & 0x80) != 0 {
        set_negative(status_flags);
    } else {
        clear_negative(status_flags);
    }

    if value == 0 {
        set_zero(status_flags);
    } else {
        clear_zero(status_flags);
    }

    *pc_reg += 3;
    *cycles_until_next = 6;
 }

 pub fn asl_absolute_x(pc_reg : &mut u16, x_reg : u8, operand: u16, memory: &mut RAM, status_flags: &mut u8, cycles_until_next : &mut u8){
    let mut value = shift_absolute_x(x_reg, operand, memory);

    if (value & 0x80) != 0 {
        set_carry(status_flags);
    } else {
        clear_carry(status_flags);
    }

    value <<= 1;
    let addr = operand + x_reg as u16;
    memory.write_mem_value(addr, value);

    if (value & 0x80) != 0 {
        set_negative(status_flags);
    } else {
        clear_negative(status_flags);
    }

    if value == 0 {
        set_zero(status_flags);
    } else {
        clear_zero(status_flags);
    }

    *pc_reg += 3;
    *cycles_until_next = 7;
 }

 // tests
 #[cfg(test)]
 mod tests{
    use super::*;
    use crate::memory;

     #[test]
     fn tests(){
         
        let operand = 7;
        let mut pc_reg  = 0;
        let mut accumulator = 15;
        let mut status : u8 = 0;
        let mut test_memory  : memory::RAM = memory::RAM::new();
        let mut cycles = 0;

        // init mem
        for i in 0..2048 {
            test_memory.write_mem_value(i, (i * 2) as u8);
        }

        asl_accumulator(&mut pc_reg, &mut accumulator, &mut status, &mut cycles);

        assert_eq!(accumulator, 30);
        assert_eq!(pc_reg, 1);
        assert_eq!(status, 0);

        asl_zero_page(&mut pc_reg, 24, &mut test_memory, &mut status, &mut cycles);

        assert_eq!(test_memory.read_mem_value(24), 96);
        assert_eq!(pc_reg, 3);
        assert_eq!(status, 0);

        asl_zero_page_x(&mut pc_reg, 130, 151, &mut test_memory, &mut status, &mut cycles);

        assert_eq!(test_memory.read_mem_value(25), 100);
        assert_eq!(pc_reg, 5);
        assert_eq!(status, 0);

        asl_absolute(&mut pc_reg, 0x3700, &mut test_memory, &mut status, &mut cycles);

        assert_eq!(test_memory.read_mem_value(127), 254);
        assert_eq!(pc_reg, 8);
        assert_eq!(status, 0x40);

        status = 0;
        asl_absolute_x(&mut pc_reg, 16, 0x1200, &mut test_memory, &mut status, &mut cycles);

        assert_eq!(test_memory.read_mem_value(34), 136);
        assert_eq!(pc_reg, 11);
        assert_eq!(status, 0x40);
     }
 }

 // TODO change to read mem-address?