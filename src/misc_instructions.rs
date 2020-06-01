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

pub fn push_acc_on_stack(pc_reg: &mut u16, accumulator_or_status : u8, stack_ptr: &mut u8, test_ram: &mut RAM, cycles : & mut u8){
    *pc_reg += 1;
    *cycles = 3;
    test_ram.push_value_on_stack(stack_ptr, accumulator_or_status);
}

pub fn pull_acc_from_stack(pc_reg: &mut u16, accumulator : &mut u8, status: &mut u8, stack_ptr: &mut u8, test_ram: &mut RAM, cycles : & mut u8){
    *pc_reg += 1;
    *cycles = 4;

    *accumulator = test_ram.pop_value_off_stack(stack_ptr);

    if *accumulator == 0 {
        *status |= ZERO_BIT;
    }

    if (*accumulator & 0x8) != 0 {
        *status |= NEGATIVE_BIT;
    }
}

pub fn pull_status_from_stack(pc_reg: &mut u16, status: &mut u8, stack_ptr: &mut u8, test_ram: &mut RAM, cycles : & mut u8){
    *pc_reg += 1;
    *cycles = 4;

    *status = test_ram.pop_value_off_stack(stack_ptr);
}

// transfer_source_to_dest is intended for the many variants of transfer functions, like TAY Transfer Accumulator to Y
// only expception is transfer x to stack pointer, as there are no flags set
pub fn transfer_source_to_dest(pc_reg: &mut u16, source : u8, dest : &mut u8, status: &mut u8, cycles : & mut u8){
    *pc_reg += 1;
    *cycles = 2;

    *dest = source;

    if *dest == 0 {
        *status |= ZERO_BIT;
    }

    if (*dest & 0x8) != 0 {
        *status |= NEGATIVE_BIT;
    }
}

pub fn transfer_x_to_stack_pointer(pc_reg: &mut u16, x : u8, stack_ptr : &mut u8, cycles : & mut u8){
    *pc_reg += 1;
    *cycles = 2;

    *stack_ptr = x;
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

        status = 0;

        pc_reg = 0;
        push_acc_status_on_stack(&mut pc_reg, 128, &mut stack, &mut test_memory, &mut cycles);
        assert_eq!(stack, 4);
        assert_eq!(pc_reg, 1);

        pull_acc_from_stack(&mut pc_reg, &mut accumulator, &mut status, &mut stack, &mut test_memory, &mut cycles);
        assert_eq!(stack, 3);
        assert_eq!(accumulator, 128);

        push_acc_status_on_stack(&mut pc_reg, 0, &mut stack, &mut test_memory, &mut cycles);
        pull_acc_from_stack(&mut pc_reg, &mut accumulator, &mut status, &mut stack, &mut test_memory, &mut cycles);
        assert_eq!(status, 2);

        transfer_x_to_stack_pointer(&mut pc_reg, 244, &mut stack, &mut cycles);
        assert_eq!(stack, 244);
    }
}