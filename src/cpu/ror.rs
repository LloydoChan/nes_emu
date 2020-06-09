//ror.rs rotate right

use crate::memory::*;
use super::shift_addr::*;
use super::flags::*;

pub fn ror_accumulator(pc_reg : &mut u16, accumulator: &mut u8, status_flags: &mut u8, cycles_until_next : &mut u8){
    let current_carry = *status_flags & 0x1;
    let old_bit_1 = *accumulator & 0x1;

    if *accumulator & 0x01 != 0 {
        *status_flags |= CARRY_BIT;
    } else{
        *status_flags &= !CARRY_BIT;
    }

    *accumulator >>= 1;
    
    if current_carry != 0 {
        *accumulator |= 0x80;
    }

    if (*accumulator & 0x80) != 0 {
        *status_flags |= NEGATIVE_BIT;
    } 

    *cycles_until_next = 2;
    *pc_reg += 1;
}

pub fn ror_zero_page(pc_reg : &mut u16, operand: u8, status_flags: &mut u8, memory: &mut RAM, cycles_until_next : &mut u8){
    let current_carry = *status_flags & 0x1;

    let mut value = shift_zero_page(operand, memory);

    if value & 0x1 != 0 {
        *status_flags |= CARRY_BIT;
    } else{
        *status_flags &= !CARRY_BIT;
    }

    value >>= 1;

    if current_carry != 0 {
        value |= 0x80;
    }

    if (value & 0x80) != 0 {
        *status_flags |= NEGATIVE_BIT;
    } 

    memory.write_mem_value(operand as u16, value);

    *cycles_until_next = 5;
    *pc_reg += 2;
}

pub fn ror_zero_page_x(pc_reg : &mut u16, x_val : u8, operand: u8, status_flags: &mut u8, memory: &mut RAM, cycles_until_next : &mut u8){
    let current_carry = *status_flags & 0x1;

    let mut value = shift_zero_page_x(x_val, operand, memory);

    if value & 0x1 != 0 {
        *status_flags |= CARRY_BIT;
    } else{
        *status_flags &= !CARRY_BIT;
    }
    
    value >>= 1;

    if current_carry != 0 {
        value |= 0x80;
    }

    if (value & 0x80) != 0 {
        *status_flags |= NEGATIVE_BIT;
    } 

    memory.write_mem_value(operand.wrapping_add(x_val) as u16, value);

    *cycles_until_next = 6;
    *pc_reg += 2;
}

pub fn ror_absolute(pc_reg : &mut u16, operand: u16, status_flags: &mut u8, memory: &mut RAM, cycles_until_next : &mut u8){
    let current_carry = *status_flags & 0x1;

    let mut value = shift_absolute(operand, memory);

    if value & 0x1 != 0 {
        *status_flags |= CARRY_BIT;
    } else{
        *status_flags &= !CARRY_BIT;
    }

    value >>= 1;

    if current_carry != 0 {
        value |= 0x80;
    }

    if (value & 0x80) != 0 {
        *status_flags |= NEGATIVE_BIT;
    } 

    memory.write_mem_value(operand, value);

    *cycles_until_next = 6;
    *pc_reg += 3;
}

pub fn ror_absolute_x(pc_reg : &mut u16, x_val : u8, operand: u16, status_flags: &mut u8, memory: &mut RAM, cycles_until_next : &mut u8){
    let current_carry = *status_flags & 0x1;

    let mut value = shift_absolute_x(x_val, operand, memory);

    if value & 0x1 != 0 {
        *status_flags |= CARRY_BIT;
    } else{
        *status_flags &= !CARRY_BIT;
    }

    value >>= 1;

    if current_carry != 0 {
        value |= 0x80;
    }

    if (value & 0x80) != 0 {
        *status_flags |= NEGATIVE_BIT;
    } 

    memory.write_mem_value(operand + x_val as u16, value);

    *cycles_until_next = 7;
    *pc_reg += 3;
}

#[cfg(test)]
mod tests{
    #[test]
    pub fn tests(){
        use super::*;
        use crate::memory;

        let mut pc_reg  = 0;
        let mut accumulator = 128;
        let mut status : u8 = 0;
        let mut test_memory  : memory::RAM = memory::RAM::new();
        let mut cycles = 0;

        for i in 0..2048 {
            test_memory.write_mem_value(i, i as u8);
        }

        ror_accumulator(&mut pc_reg, &mut accumulator, &mut status, &mut cycles);

        assert_eq!(accumulator, 64);
        assert_eq!(pc_reg, 1);
        assert_eq!(cycles, 2);
        assert_eq!(status, 0);

        let mut accumulator = 65;
        ror_accumulator(&mut pc_reg, &mut accumulator, &mut status, &mut cycles);

        assert_eq!(accumulator, 32);
        assert_eq!(pc_reg, 2);
        assert_eq!(cycles, 2);
        assert_eq!(status, 1);

        ror_zero_page(&mut pc_reg, 19, &mut status, &mut test_memory, &mut cycles);

        assert_eq!(test_memory.read_mem_value(19 as u16), 9 | 0x80);
        assert_eq!(pc_reg, 4);
        assert_eq!(cycles, 5);
        assert_eq!(status, 65);

        ror_zero_page(&mut pc_reg, 128, &mut status, &mut test_memory, &mut cycles);

        assert_eq!(test_memory.read_mem_value(128), 64 + 128);
        assert_eq!(pc_reg, 6);
        assert_eq!(cycles, 5);
        assert_eq!(status, 64);

        status = 0;
        ror_zero_page_x(&mut pc_reg, 3, 254, &mut status, &mut test_memory, &mut cycles);

        assert_eq!(test_memory.read_mem_value(1), 0);
        assert_eq!(pc_reg, 8);
        assert_eq!(cycles, 6);
        assert_eq!(status, 1);

        ror_zero_page_x(&mut pc_reg, 3, 251, &mut status, &mut test_memory, &mut cycles);

        assert_eq!(test_memory.read_mem_value(254), 127 + 128);
        assert_eq!(pc_reg, 10);
        assert_eq!(cycles, 6);
        assert_eq!(status, 64);

        status = 0;
        ror_absolute(&mut pc_reg, swap_bytes(264), &mut status, &mut test_memory, &mut cycles);

        assert_eq!(test_memory.read_mem_value(264), 4);
        assert_eq!(pc_reg, 13);
        assert_eq!(cycles, 6);
        assert_eq!(status, 0);

        ror_absolute(&mut pc_reg, swap_bytes(265), &mut status, &mut test_memory, &mut cycles);

        assert_eq!(test_memory.read_mem_value(264), 4);
        assert_eq!(pc_reg, 16);
        assert_eq!(cycles, 6);
        assert_eq!(status, 1);

        status = 0;

        ror_absolute_x(&mut pc_reg, 13, swap_bytes(285), &mut status, &mut test_memory, &mut cycles);

        assert_eq!(test_memory.read_mem_value(298), 21);
        assert_eq!(pc_reg, 19);
        assert_eq!(cycles, 7);
        assert_eq!(status, 0);
    }
}