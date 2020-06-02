//branch.rs - for all branch instructions, they use implied addressing, thankfully!

// all take 8 bit signed relative offset, all branch on status of a flag...

use crate::flags;



fn flag_bit_check_set(flag_val : u8, flag_bit : u8) -> bool{
    (flag_val & flag_bit) != 0
}

fn branch_on_flag_set(flag_val : u8, pc_reg : &mut u16, relative_addr : i8, flag_bit: u8){

    if flag_bit_check_set(flag_val, flag_bit) {
        let mut temp_reg = *pc_reg as i16;
        temp_reg += relative_addr as i16;
        *pc_reg = temp_reg as u16;
    }
        
    *pc_reg += 2;
    
}

fn branch_on_flag_not_set(flag_val : u8, pc_reg : &mut u16, relative_addr : i8, flag_bit: u8){

    if !flag_bit_check_set(flag_val, flag_bit) {
        let mut temp_reg = *pc_reg as i16;
        temp_reg += relative_addr as i16;
        *pc_reg = temp_reg as u16;
    }
        
    *pc_reg += 2;
}

pub fn branch_if_carry_set(flag_val : u8, mut pc_reg : &mut u16, relative_addr : i8, cycles : &mut u8){
    branch_on_flag_set(flag_val, &mut pc_reg, relative_addr, flags::CARRY_BIT);
    *cycles = 2;
}

pub fn branch_if_carry_clear(flag_val : u8, mut pc_reg : &mut u16, relative_addr : i8, cycles : &mut u8){
    branch_on_flag_not_set(flag_val, &mut pc_reg, relative_addr, flags::CARRY_BIT);
    *cycles = 2;
}

pub fn branch_if_equal(flag_val : u8, mut pc_reg : &mut u16, relative_addr : i8, cycles : &mut u8){
    branch_on_flag_set(flag_val, &mut pc_reg, relative_addr, flags::ZERO_BIT);
    *cycles = 2;
}

pub fn branch_if_not_equal(flag_val : u8, mut pc_reg : &mut u16, relative_addr : i8, cycles : &mut u8){
    branch_on_flag_not_set(flag_val, &mut pc_reg, relative_addr, flags::ZERO_BIT);
    *cycles = 2;
}

pub fn branch_if_minus(flag_val : u8, mut pc_reg : &mut u16, relative_addr : i8, cycles : &mut u8){
    branch_on_flag_set(flag_val, &mut pc_reg, relative_addr, flags::NEGATIVE_BIT);
    *cycles = 2;
}

pub fn branch_if_positive(flag_val : u8, mut pc_reg : &mut u16, relative_addr : i8, cycles : &mut u8){
    branch_on_flag_not_set(flag_val, &mut pc_reg, relative_addr, flags::NEGATIVE_BIT);
    *cycles = 2;
}

pub fn branch_if_overflow_set(flag_val : u8, mut pc_reg : &mut u16, relative_addr : i8, cycles : &mut u8){
    branch_on_flag_set(flag_val, &mut pc_reg, relative_addr, flags::OVERFLOW_BIT);
    *cycles = 2;
}

pub fn branch_if_overflow_clear(flag_val : u8, mut pc_reg : &mut u16, relative_addr : i8, cycles : &mut u8){
    branch_on_flag_not_set(flag_val, &mut pc_reg, relative_addr, flags::OVERFLOW_BIT);
    *cycles = 2;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_branches() {
        let mut flag_bits : u8 = 0;
        flag_bits |= flags::CARRY_BIT;

        let mut pc_reg : u16 = 256;
        let mut cycles = 0;

        branch_if_carry_set(flag_bits, &mut pc_reg, -12, &mut cycles);
        assert_eq!(pc_reg, 246);

        flag_bits ^= flags::CARRY_BIT;
        branch_if_carry_set(flag_bits, &mut pc_reg, -12, &mut cycles);
        assert_eq!(pc_reg, 248);

        branch_if_carry_clear(flag_bits, &mut pc_reg, -12, &mut cycles);
        assert_eq!(pc_reg, 238);

        flag_bits |= flags::ZERO_BIT;
        branch_if_equal(flag_bits, &mut pc_reg, 28, &mut cycles);
        assert_eq!(pc_reg, 268);

        flag_bits ^= flags::ZERO_BIT;
        branch_if_equal(flag_bits, &mut pc_reg, 28, &mut cycles);
        assert_eq!(pc_reg, 270);


        flag_bits |= flags::NEGATIVE_BIT;
        branch_if_minus(flag_bits, &mut pc_reg, -38, &mut cycles);
        assert_eq!(pc_reg, 234);
    }
}