//increment_decrement.rs - source for inc and dec isntructions

use crate::memory::RAM;
use crate::flags::*;

fn set_flags(status_flags : &mut u8, new_value : u8){

    if new_value == 0 {
        *status_flags |= ZERO_BIT;
    }

    if (new_value & 0x80) != 0 {
        *status_flags |= NEGATIVE_BIT;
    }

}

pub enum Operation{
    Inc,
    Dec,
}

fn match_on_op(mut in_val : u8, op : Operation) -> u8{

    match op{
        Operation::Inc => in_val = in_val.wrapping_add(1),
        Operation::Dec => in_val = in_val.wrapping_sub(1),
        _=> panic!()
    }

    in_val
}

pub fn incdec_memory_zero_page(pc_reg : &mut u16, operand : u8, status_flags: &mut u8, memory : &mut RAM, cycles : &mut u8, op : Operation){

    let mut mem_value = memory.read_mem_value(operand as u16);
    mem_value = match_on_op(mem_value, op);
    memory.write_mem_value(operand as u16, mem_value);

    set_flags(status_flags, mem_value);

    *pc_reg +=2;
    *cycles = 5;
}

pub fn incdec_memory_zero_page_x(pc_reg : &mut u16, operand : u8, x_val : u8, status_flags: &mut u8, memory : &mut RAM, cycles : &mut u8, op : Operation){

    let address = operand.wrapping_add(x_val);
    let mut mem_value = memory.read_mem_value(address as u16);
    mem_value = match_on_op(mem_value, op);
    memory.write_mem_value(address as u16, mem_value);

    set_flags(status_flags, mem_value);

    *pc_reg +=2;
    *cycles = 6;
}

pub fn incdec_memory_absolute(pc_reg : &mut u16, operand : u16, status_flags: &mut u8, memory : &mut RAM, cycles : &mut u8, op : Operation) {
    let mut mem_value = memory.read_mem_value(operand as u16);
    mem_value = match_on_op(mem_value, op);
    memory.write_mem_value(operand, mem_value);

    set_flags(status_flags, mem_value);

    *pc_reg += 3;
    *cycles =  6;
}

pub fn incdec_memory_absolute_x(pc_reg : &mut u16, operand : u16, x_val : u8, status_flags: &mut u8, memory : &mut RAM, cycles : &mut u8, op : Operation) {
    let mut mem_value = memory.read_mem_value(operand + x_val as u16);
    mem_value = match_on_op(mem_value, op);
    memory.write_mem_value(operand + x_val as u16, mem_value);

    set_flags(status_flags, mem_value);

    *pc_reg += 3;
    *cycles =  7;
}

pub fn incdec_reg(pc_reg : &mut u16, reg_val : &mut u8, status_flags: &mut u8, cycles : &mut u8, op : Operation){
    *reg_val = match_on_op(*reg_val, op);
    set_flags(status_flags, *reg_val);
    *pc_reg += 1;
    *cycles =  2;
}

#[cfg(test)]
mod tests{
    #[test]
    pub fn test_inc_dec(){

        use super::*;
        use crate::memory::RAM;

        let mut test_memory  : RAM = RAM::new();
        let mut val = 0;
        let mut stack_ptr = 0;
        let mut pc_reg : u16 = 0;
        let mut status = 0;
        let mut cycles = 0;

        // init mem
        for i in 0..2048 {
            test_memory.write_mem_value(i, i as u8);
        }

        incdec_reg(&mut pc_reg, &mut val, &mut status, &mut cycles, Operation::Inc);

        assert_eq!(pc_reg, 1);
        assert_eq!(cycles, 2);
        assert_eq!(val, 1);
        assert_eq!(status, 0);

        incdec_reg(&mut pc_reg, &mut val, &mut status, &mut cycles, Operation::Dec);

        assert_eq!(pc_reg, 2);
        assert_eq!(cycles, 2);
        assert_eq!(val, 0);
        assert_eq!(status, 2);

        incdec_memory_zero_page(&mut pc_reg, 230, &mut status, &mut test_memory, &mut cycles, Operation::Dec);

        assert_eq!(pc_reg, 4);
        assert_eq!(cycles, 5);

        let test_val = test_memory.read_mem_value(230);
        status = 0;
        assert_eq!(test_val, 229);
        assert_eq!(status, 0);

        incdec_memory_absolute(&mut pc_reg, 383, &mut status, &mut test_memory, &mut cycles, Operation::Inc);
        assert_eq!(pc_reg, 7);
        assert_eq!(cycles, 6);
        let test_val = test_memory.read_mem_value(383);
        assert_eq!(test_val, 128);
        assert_eq!(status, 0x40);

        incdec_memory_absolute_x(&mut pc_reg, 383, 12, &mut status, &mut test_memory, &mut cycles, Operation::Dec);
        assert_eq!(pc_reg, 10);
        assert_eq!(cycles, 7);
        let test_val = test_memory.read_mem_value(395);
        assert_eq!(test_val, 138);
        assert_eq!(status, 0x40);
    }
}