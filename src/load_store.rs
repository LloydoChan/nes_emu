// load_store - load and store instructions on 6502
use crate::memory::*;
use crate::flags::*;

pub fn set_flags(in_val: u8, status_flag: &mut u8){
   
    if in_val == 0 { 
        set_zero(status_flag) 
    }else{
        clear_zero(status_flag)
    }

     if (in_val & 0x80) != 0 {
        set_negative(status_flag)
    }else{
        clear_negative(status_flag)
    }
}

pub fn load_zero_page(pc_reg: &mut u16, operand: u8, memory: &RAM, status_flag: &mut u8, cycles: &mut u8) -> u8 {
    let ret_val = memory.read_mem_value(operand as u16);
    set_flags(ret_val, status_flag);

    //cycles
    *cycles = 3;
    *pc_reg += 2;
    ret_val
}

pub fn load_zero_page_reg(pc_reg: &mut u16, operand: u8, offset: u8, memory: &RAM, status_flag: &mut u8, cycles: &mut u8) -> u8 {
    let addr = operand.wrapping_add(offset);
    let ret_val = memory.read_mem_value(addr as u16);
    set_flags(ret_val, status_flag);

    *cycles = 4;
    *pc_reg += 2;
    ret_val
}

pub fn store_zero_page(pc_reg: &mut u16, to_store: u8, operand: u8, offset: u8, memory: &mut RAM, cycles: &mut u8){
    let addr = operand.wrapping_add(offset);
    memory.write_mem_value(addr as u16, to_store);
    *cycles = 3;
    *pc_reg += 2; 
}


pub fn absolute_load(pc_reg: &mut u16, operand: u16, offset: u8, memory: &RAM, status_flag: &mut u8, cycles: &mut u8) -> u8 {
    let addr = operand.wrapping_add(offset as u16);
    let ret_val = memory.read_mem_value(addr as u16);
    set_flags(ret_val, status_flag);

    if offset == 0 {
        *cycles = 4;
    }else{
        *cycles = 5;
    }
   
    *pc_reg += 3;
    ret_val
}

pub fn store_absolute(pc_reg: &mut u16, to_store: u8, operand: u16, offset: u8, memory: &mut RAM, cycles: &mut u8){
    let addr = operand + offset as u16;
    memory.write_mem_value(addr as u16, to_store);
    *pc_reg += 3;
    *cycles = 4;
}

pub fn indirect_x_load(pc_reg: &mut u16, operand: u8, x_val: u8, memory: &RAM, status_flag: &mut u8, cycles: &mut u8) -> u8 {
    let addr = operand.wrapping_add(x_val);
    let table_addr = memory.read_mem_address(addr as u16);
    let mem_value = memory.read_mem_value(table_addr); 
    set_flags(mem_value, status_flag);

    *pc_reg += 2;
    *cycles = 6;

    mem_value
}


pub fn store_indirect_x(pc_reg: &mut u16, to_store: u8, operand: u8, offset: u8, memory: &mut RAM, cycles: &mut u8){
    let addr = operand.wrapping_add(offset);
    let write_addr = memory.read_mem_address(addr as u16);
    memory.write_mem_value(write_addr, to_store);
    *pc_reg += 2;
    *cycles = 6;
}

pub fn indirect_y_load(pc_reg: &mut u16, operand: u8, y_val: u8, memory: &RAM, status_flag: &mut u8, cycles: &mut u8) -> u8 {
    let table_addr = memory.read_mem_address(operand as u16);  
    let mem_value = memory.read_mem_value(table_addr.wrapping_add(y_val as u16)); 
    set_flags(mem_value, status_flag);

    *pc_reg += 2;
    *cycles = 6;

    mem_value
}

pub fn store_indirect_y(pc_reg: &mut u16, to_store: u8, operand: u8, offset: u8, memory: &mut RAM, cycles: &mut u8){
    let write_addr = memory.read_mem_address(operand as u16) + offset as u16;
    memory.write_mem_value(write_addr, to_store);
    *pc_reg += 2;
    *cycles = 6;
}

#[cfg(test)]
mod tests{
    use crate::memory;
    use super::*;

    #[test]
    fn load_store_tests(){
        let operand = 7;
        let mut pc_reg  = 0;
        let mut accumulator = 25;
        let mut status : u8 = 0;
        let mut test_memory  : memory::RAM = memory::RAM::new();
        let mut cycles = 0;

        // init mem
        for i in 0..512 {
            test_memory.write_mem_value(i*2, 0  as u8);
            test_memory.write_mem_value(i*2+1, i  as u8);
        }

        for i in 512..1024 {
            test_memory.write_mem_value(i, i  as u8);
        }

        accumulator = load_zero_page(&mut pc_reg, 5, &test_memory, &mut status, &mut cycles);
        assert_eq!(accumulator, 2);
    }
}