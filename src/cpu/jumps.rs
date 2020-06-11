//jumps.rs - jump instructions
use crate::memory::{RAM, *};
use crate::mem_map;

pub fn jump_absolute(pc_reg : &mut u16, absolute_addr: u16, cycles : &mut u8){
    *pc_reg = absolute_addr;
    *cycles = 3;
}

pub fn jump_indirect(pc_reg : &mut u16, indirect_addr: u16, ram: &mut RAM, cycles : &mut u8){
    let mut addr = ram.read_mem_address(indirect_addr);

    // is the indirect addr on a page boundary?
    if indirect_addr & 0x00FF != 0 {
        // need to get msb from xx00 where xx is the first byte in the indirect address
        let msb_addr = (indirect_addr >> 8) << 8;
        let msb = (ram.read_mem_value(msb_addr) as u16) << 8;
        addr = (addr << 8 ) >> 8;
        addr = msb | addr;
    }
    
    
    *pc_reg = addr;
    *cycles = 5;
}

pub fn jump_subroutine(pc_reg : &mut u16, absolute_addr: u16, stack_ptr : &mut u8, ram: &mut RAM, cycles : &mut u8){
    ram.push_address_on_stack(stack_ptr, *pc_reg + 2);
    *pc_reg = absolute_addr;
    *cycles = 6;
}

pub fn return_from_subroutine(pc_reg : &mut u16, stack_ptr : &mut u8, ram: &mut RAM, cycles : &mut u8){
    let addr = ram.pop_address_off_stack(stack_ptr);
    *pc_reg = addr;
    *pc_reg += 1;
    *cycles = 6;
}

pub fn return_from_interrupt(pc_reg : &mut u16, stack_ptr : &mut u8, status_flags: &mut u8, ram: &mut RAM, cycles : &mut u8){
    *status_flags = ram.pop_value_off_stack(stack_ptr);
    // set bit 5
    *status_flags |= 0b0010_0000;
    *pc_reg = ram.pop_address_off_stack(stack_ptr);
    *cycles = 6;
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_jumps(){
        use super::*;
        use crate::memory;

        let operand = 7;
        let mut pc_reg  = 0;
        let mut accumulator = 15;
        let mut status : u8 = 0;
        let mut test_memory  : memory::RAM = memory::RAM::new();
        let mut cycles = 0;
        let mut stack_ptr = 0;

        // init mem
        for i in 0..2048 {
            test_memory.write_mem_value(i, i as u8);
        }

        jump_absolute(&mut pc_reg, 0xFFF, &mut cycles);

        assert_eq!(pc_reg, 0xFFF);

        let indirect_addr = swap_bytes(test_memory.read_mem_address(0x07));

        jump_indirect(&mut pc_reg, 0x07, &mut test_memory, &mut cycles);

        assert_eq!(pc_reg, indirect_addr);

        pc_reg = 256;

        jump_subroutine(&mut pc_reg, 0x0100, &mut stack_ptr, &mut test_memory, &mut cycles);
        let absolute_addr = swap_bytes(0x0100);
        assert_eq!(pc_reg, absolute_addr);
        assert_eq!(stack_ptr, 2);

        let stack_addr = test_memory.read_mem_address(mem_map::STACK_START as u16);
        assert_eq!(stack_addr, 255);

        return_from_subroutine(&mut pc_reg, &mut stack_ptr, &mut test_memory, &mut cycles);
        assert_eq!(stack_ptr, 0);
        assert_eq!(pc_reg, 256);

    }
}