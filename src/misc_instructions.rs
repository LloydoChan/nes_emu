// misc_instructions.rs for BIT instruction, NOP and more
use crate::memory::{RAM, *};
use crate::flags::*;
use crate::mem_map;

pub fn bittest_zero_page(pc_reg : &mut u16, accumulator : u8, operand: u8,  mem : &mut RAM, status_flags : &mut u8, cycles : &mut u8){
    let value = mem.read_mem_value(operand as u16);
    let result = accumulator & value;

    if result == 0 {
        *status_flags |= ZERO_BIT;
    }

    if (value & 0x40) != 0 {
        *status_flags |= OVERFLOW_BIT;
    }else{
        *status_flags &= !OVERFLOW_BIT;
    }

    if (value & 0x80) != 0 {
        *status_flags |= NEGATIVE_BIT;
    }else{
        *status_flags &= !NEGATIVE_BIT;
    }

    *cycles = 3;
    *pc_reg += 2;
}

pub fn bittest_absolute(pc_reg : &mut u16, accumulator : u8, operand: u16,  mem : &mut RAM, status_flags : &mut u8, cycles : &mut u8){
    let value = mem.read_mem_value(operand);
    let result = accumulator & value;

    if result == 0 {
        *status_flags |= ZERO_BIT;
    }

    if (value & 0x40) != 0 {
        *status_flags |= OVERFLOW_BIT;
    }else{
        *status_flags &= !OVERFLOW_BIT;
    }

    if (value & 0x80) != 0 {
        *status_flags |= NEGATIVE_BIT;
    }else{
        *status_flags &= !NEGATIVE_BIT;
    }

    *cycles = 4;
    *pc_reg += 3;
}

pub fn NOP(pc_reg: &mut u16, cycles : &mut u8){
    *pc_reg += 1;
    *cycles = 2;
}

pub fn break_force_interrupt(pc_reg: &mut u16, status: &mut u8, stack_ptr: &mut u8, test_ram : &mut RAM, cycles : &mut u8){
    *pc_reg += 1;
    *cycles = 7;

    test_ram.push_address_on_stack(stack_ptr, *pc_reg);
    test_ram.push_value_on_stack(stack_ptr, *status);
    
    //TODO push pc and status onto stack, load IRQ into PC
    *status |= BREAK_CMD_BIT;
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_misc(){
        use super::*;
        use crate::memory;

        let operand = 7;
        let mut pc_reg  = 0;
        let mut accumulator = 7;
        let mut status : u8 = 0;
        let mut test_memory  : memory::RAM = memory::RAM::new();
        let mut cycles = 0;

        let mut stack = 0;

        // init mem
        for i in 0..2048 {
            test_memory.write_mem_value(i, i  as u8);
        }

        bittest_zero_page(&mut pc_reg, accumulator, 7, &mut test_memory, &mut status, &mut cycles);

        assert_eq!(pc_reg, 2);
        assert_eq!(accumulator, 7);
        assert_eq!(status, 0);

        accumulator = 192;

        bittest_zero_page(&mut pc_reg, accumulator, 194, &mut test_memory, &mut status, &mut cycles);

        assert_eq!(pc_reg, 4);
        assert_eq!(accumulator, 192);
        assert_eq!(status, 96);

        bittest_zero_page(&mut pc_reg, accumulator, 1, &mut test_memory, &mut status, &mut cycles);

        assert_eq!(pc_reg, 6);
        assert_eq!(accumulator, 192);
        assert_eq!(status, 2);

        bittest_absolute(&mut pc_reg, accumulator, 290, &mut test_memory, &mut status, &mut cycles);

        assert_eq!(pc_reg, 9);
        assert_eq!(accumulator, 192);
        assert_eq!(status, 2);

        NOP(&mut pc_reg, &mut cycles);

        assert_eq!(pc_reg, 10);
        
        break_force_interrupt(&mut pc_reg, &mut status, &mut stack, &mut test_memory, &mut cycles);

        assert_eq!(stack, 3);
        assert_eq!(status, 18);
    }
}