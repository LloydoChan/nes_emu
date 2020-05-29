//compare.rs - all compare instructions

use crate::memory::RAM;
use crate::flags::*;

fn set_flags(status_flags : &mut u8, reg_acc : u8, comp_value: u8){

    if reg_acc == comp_value {
        *status_flags |= ZERO_BIT;
    }

    if reg_acc >= comp_value {
        *status_flags |= CARRY_BIT;
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

    let mem_val = mem.read_mem_value(abs_addr);
    set_flags(status_flags, reg_acc, mem_val);
    *pc_reg += 3;
    *cycles = 4;
}

pub fn comp_value_absolute_reg(pc_reg : &mut u16, acc: u8, abs_addr : u16, reg: u8, mem : &mut RAM, status_flags: &mut u8, cycles : &mut u8){

    let mem_val = mem.read_mem_value(abs_addr + reg as u16);
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

    }
}