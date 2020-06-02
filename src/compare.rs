//compare.rs - all compare instructions

use crate::memory::{RAM, *};
use crate::flags::*;

fn set_flags(status_flags : &mut u8, reg_acc : u8, comp_value: u8){

    if reg_acc == comp_value {
        *status_flags |= ZERO_BIT;
    } else {
        *status_flags &= !ZERO_BIT;
    }

    if reg_acc >= comp_value {
        *status_flags |= CARRY_BIT;
    } else {
        *status_flags &= !CARRY_BIT;
    }

    let result = reg_acc.wrapping_sub(comp_value);

    if result & 0x80 != 0 {
        *status_flags |= NEGATIVE_BIT;
    }else{
        *status_flags &= !NEGATIVE_BIT;
    }

}

pub fn comp_value_immediate(pc_reg : &mut u16, reg_acc: u8, imm_value : u8, status_flags: &mut u8, cycles : &mut u8){
    set_flags(status_flags, reg_acc, imm_value);
    *pc_reg += 2;
    *cycles = 2;
}

pub fn comp_value_zero_page(pc_reg : &mut u16, reg_acc: u8, page_addr : u8, mem : &mut RAM, status_flags: &mut u8, cycles : &mut u8){

    let mem_val = mem.read_mem_value(page_addr as u16);
    set_flags(status_flags, reg_acc, mem_val);
    *pc_reg += 2;
    *cycles = 3;
}

pub fn comp_value_zero_page_x(pc_reg : &mut u16, acc: u8, page_addr : u8, x_val : u8, mem : &mut RAM, status_flags: &mut u8, cycles : &mut u8){
    let addr = page_addr.wrapping_add(x_val);
    let mem_val = mem.read_mem_value(addr as u16);
    set_flags(status_flags, acc, mem_val);
    *pc_reg += 2;
    *cycles = 4;
}

pub fn comp_value_absolute(pc_reg : &mut u16, reg_acc: u8, abs_addr : u16, mem : &mut RAM, status_flags: &mut u8, cycles : &mut u8){

    let mem_val = mem.read_mem_value(swap_bytes(abs_addr));
    set_flags(status_flags, reg_acc, mem_val);
    *pc_reg += 3;
    *cycles = 4;
}

pub fn comp_value_absolute_reg(pc_reg : &mut u16, acc: u8, abs_addr : u16, reg: u8, mem : &mut RAM, status_flags: &mut u8, cycles : &mut u8){

    let mem_val = mem.read_mem_value(swap_bytes(abs_addr) + reg as u16);
    set_flags(status_flags, acc, mem_val);
    *pc_reg += 3;
    *cycles = 4;
}

pub fn comp_value_indexed_indirect(pc_reg : &mut u16, acc: u8, addr: u8, x_reg: u8, mem : &mut RAM, status_flags: &mut u8, cycles : &mut u8){
    let mem_addr = mem.read_mem_address((addr + x_reg) as u16);
    let mem_val = mem.read_mem_value(mem_addr);
    set_flags(status_flags, acc, mem_val);
    *pc_reg += 2;
    *cycles = 6;
}

pub fn comp_value_indirect_indexed(pc_reg : &mut u16, acc: u8, addr: u8, y_reg: u8, mem : &mut RAM, status_flags: &mut u8, cycles : &mut u8){
    let mem_addr = mem.read_mem_address(addr as u16);
    let mem_val = mem.read_mem_value(mem_addr + y_reg as u16);
    set_flags(status_flags, acc, mem_val);
    *pc_reg += 2;
    *cycles = 5;
}



#[cfg(test)]
mod tests {
    #[test]
    pub fn test_compare(){
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

        comp_value_immediate(&mut pc_reg, 128, 128, &mut status, &mut cycles);
        assert_eq!(status, 3);
        assert_eq!(pc_reg, 2);
        assert_eq!(cycles, 2);

        status = 0;
        comp_value_zero_page(&mut pc_reg, 128, 128, &mut test_memory, &mut status, &mut cycles);
        assert_eq!(status, 3);
        assert_eq!(pc_reg, 4);
        assert_eq!(cycles, 3);

        status = 0;
        comp_value_zero_page_x(&mut pc_reg, 128, 125, 3, &mut test_memory, &mut status, &mut cycles);
        assert_eq!(status, 3);
        assert_eq!(pc_reg, 6);
        assert_eq!(cycles, 4);

        status = 0;
        comp_value_absolute(&mut pc_reg, 128, 384, &mut test_memory, &mut status, &mut cycles);
        assert_eq!(status, 3);
        assert_eq!(pc_reg, 9);
        assert_eq!(cycles, 4);

        status = 0;
        comp_value_immediate(&mut pc_reg, 132, 128, &mut status, &mut cycles);
        assert_eq!(status, 1);
        assert_eq!(pc_reg, 11);
        assert_eq!(cycles, 2);

        status = 0;
        comp_value_absolute_reg(&mut pc_reg, 128, 383, 1, &mut test_memory, &mut status, &mut cycles);
        assert_eq!(status, 3);
        assert_eq!(pc_reg, 14);
        assert_eq!(cycles, 4);

        status = 0;
        comp_value_indexed_indirect(&mut pc_reg, 5, 2, 3, &mut test_memory, &mut status, &mut cycles);
        assert_eq!(status, 3);
        assert_eq!(pc_reg, 16);
        assert_eq!(cycles, 6);

        status = 0;
        comp_value_indirect_indexed(&mut pc_reg, 10, 6, 3, &mut test_memory, &mut status, &mut cycles);
        assert_eq!(status, 1);
        assert_eq!(pc_reg, 18);
        assert_eq!(cycles, 5);
    }
}