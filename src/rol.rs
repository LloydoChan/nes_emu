// rol.rs - rotate left

use crate::memory::*;
use crate::shift_addr::*;
use crate::flags::*;

pub fn rol_accumulator(pc_reg : &mut u16, accumulator: &mut u8, status_flags: &mut u8, cycles_until_next : &mut u8){
    let current_carry = *status_flags & 0x1;

    if (*accumulator & 0x80) != 0 {
        *status_flags |= CARRY_BIT;
    } else{
        *status_flags &= !CARRY_BIT;
    }

    *accumulator <<= 1;
    *accumulator |= current_carry;

    if (*accumulator & 0x80) != 0 {
        *status_flags |= NEGATIVE_BIT;
    } else {
        *status_flags &= !NEGATIVE_BIT;
    }

    if *accumulator == 0 {
        *status_flags |= ZERO_BIT;
    } else {
        *status_flags &= !ZERO_BIT;
    }

    *cycles_until_next = 2;
    *pc_reg += 1;
}

pub fn rol_zero_page(pc_reg : &mut u16, operand: u8, status_flags: &mut u8, memory: &mut RAM, cycles_until_next : &mut u8){
    let current_carry = *status_flags & 0x1;

    let mut value = shift_zero_page(operand, memory);

    let old_bit_7 = (value & 0x80) >> 7;
    *status_flags |= old_bit_7;

    value <<= 1;
    value |= current_carry;

    memory.write_mem_value(operand as u16, value);

    if (value & 0x80) != 0 {
        *status_flags |= NEGATIVE_BIT;
    } else {
        *status_flags &= !NEGATIVE_BIT;
    }

    *cycles_until_next = 5;
    *pc_reg += 2;
}

pub fn rol_zero_page_x(pc_reg : &mut u16, x_val : u8, operand: u8, status_flags: &mut u8, memory: &mut RAM, cycles_until_next : &mut u8){
    let current_carry = *status_flags & 0x1;

    let mut value = shift_zero_page_x(x_val, operand, memory);

    if (value & 0x80) != 0 {
        *status_flags |= CARRY_BIT;
    } else{
        *status_flags &= !CARRY_BIT;
    }
    
    value <<= 1;
    value |= current_carry;
    memory.write_mem_value(operand.wrapping_add(x_val) as u16, value);

    
    if (value & 0x80) != 0 {
        *status_flags |= NEGATIVE_BIT;
    } else {
        *status_flags &= !NEGATIVE_BIT;
    }

    // if value == 0 {
    //     *status_flags |= ZERO_BIT;
    // } else {
    //     *status_flags &= !ZERO_BIT;
    // }

    *cycles_until_next = 6;
    *pc_reg += 2;
}

pub fn rol_absolute(pc_reg : &mut u16, operand: u16, status_flags: &mut u8, memory: &mut RAM, cycles_until_next : &mut u8){
    let current_carry = *status_flags & 0x1;

    let mut value = shift_absolute(operand, memory);

    if (value & 0x80) != 0 {
        *status_flags |= CARRY_BIT;
    } else{
        *status_flags &= !CARRY_BIT;
    }

    value <<= 1;
    value |= current_carry;
    memory.write_mem_value(operand, value);
      
    if (value & 0x80) != 0 {
        *status_flags |= NEGATIVE_BIT;
    } else {
        *status_flags &= !NEGATIVE_BIT;
    } 

    // if value == 0 {
    //     *status_flags |= ZERO_BIT;
    // } else {
    //     *status_flags &= !ZERO_BIT;
    // }

    *cycles_until_next = 6;
    *pc_reg += 3;
}

pub fn rol_absolute_x(pc_reg : &mut u16, x_val : u8, operand: u16, status_flags: &mut u8, memory: &mut RAM, cycles_until_next : &mut u8){
    let current_carry = *status_flags & 0x1;

    let mut value = shift_absolute_x(x_val, operand, memory);

    if (value & 0x80) != 0 {
        *status_flags |= CARRY_BIT;
    } else{
        *status_flags &= !CARRY_BIT;
    }

    value <<= 1;
    value |= current_carry;

    memory.write_mem_value(operand + x_val as u16, value);

    if (value & 0x80) != 0 {
        *status_flags |= NEGATIVE_BIT;
    } else {
        *status_flags &= !NEGATIVE_BIT;
    }

    // if value == 0 {
    //     *status_flags |= ZERO_BIT;
    // } else {
    //     *status_flags &= !ZERO_BIT;
    // }


    *cycles_until_next = 7;
    *pc_reg += 3;
}

#[cfg(test)]
mod tests{
    #[test]
    pub fn tests(){
        use super::*;
        use crate::memory;

        let operand = 7;
        let mut pc_reg  = 0;
        let mut accumulator = 128;
        let mut status : u8 = 0;
        let mut test_memory  : memory::RAM = memory::RAM::new();
        let mut cycles = 0;

        for i in 0..2048 {
            test_memory.write_mem_value(i, i as u8);
        }

        rol_accumulator(&mut pc_reg, &mut accumulator, &mut status, &mut cycles);

        assert_eq!(accumulator, 0);
        assert_eq!(pc_reg, 1);
        assert_eq!(cycles, 2);
        assert_eq!(status, 1);

        rol_accumulator(&mut pc_reg, &mut accumulator, &mut status, &mut cycles);

        assert_eq!(accumulator, 1);
        assert_eq!(pc_reg, 2);
        assert_eq!(cycles, 2);
        assert_eq!(status, 0);

        rol_zero_page(&mut pc_reg, operand, &mut status, &mut test_memory, &mut cycles);

        assert_eq!(test_memory.read_mem_value(operand as u16), 14);
        assert_eq!(pc_reg, 4);
        assert_eq!(cycles, 5);
        assert_eq!(status, 0);

        rol_zero_page(&mut pc_reg, 128, &mut status, &mut test_memory, &mut cycles);

        assert_eq!(test_memory.read_mem_value(128), 0);
        assert_eq!(pc_reg, 6);
        assert_eq!(cycles, 5);
        assert_eq!(status, 1);

        rol_zero_page_x(&mut pc_reg, 3, 254, &mut status, &mut test_memory, &mut cycles);

        assert_eq!(test_memory.read_mem_value(1), 3);
        assert_eq!(pc_reg, 8);
        assert_eq!(cycles, 6);
        assert_eq!(status, 0);

        rol_zero_page_x(&mut pc_reg, 3, 251, &mut status, &mut test_memory, &mut cycles);

        assert_eq!(test_memory.read_mem_value(254), 252);
        assert_eq!(pc_reg, 10);
        assert_eq!(cycles, 6);
        assert_eq!(status, 65);

        rol_absolute(&mut pc_reg, swap_bytes(264), &mut status, &mut test_memory, &mut cycles);

        assert_eq!(test_memory.read_mem_value(264), 17);
        assert_eq!(pc_reg, 13);
        assert_eq!(cycles, 6);
        assert_eq!(status, 64);

        status = 0;

        rol_absolute_x(&mut pc_reg, 13, swap_bytes(264), &mut status, &mut test_memory, &mut cycles);

        assert_eq!(test_memory.read_mem_value(277), 42);
        assert_eq!(pc_reg, 16);
        assert_eq!(cycles, 7);
        assert_eq!(status, 0);
    }
}