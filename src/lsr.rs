//lsr.rs - logical shift right
use crate::memory::*;
use crate::shift_addr::*;
use crate::flags::*;



pub fn lsr_accumulator(pc_reg : &mut u16, accumulator: &mut u8, status_flags: &mut u8){
    
    if (*accumulator & 0x1) != 0 {
        set_carry(status_flags);
    }

    *accumulator >>= 1;

    if *accumulator == 0 {
        set_zero(status_flags);
    }

    *pc_reg += 1;
 }

 pub fn lsr_zero_page(pc_reg : &mut u16, operand: u8, memory: &mut RAM, status_flags: &mut u8){

    let mut value = shift_zero_page(operand, memory);

    if (value & 0x1) != 0 {
        set_carry(status_flags);
    }

    value >>= 1;
    memory.write_mem_value(operand as u16, value);

    if value == 0 {
        set_zero(status_flags);
    }

    *pc_reg += 2;
 }

 pub fn lsr_zero_page_x(pc_reg : &mut u16, x_val : u8, operand: u8, memory: &mut RAM, status_flags: &mut u8){

    let mut value = shift_zero_page_x(x_val, operand, memory);

    if (value & 0x01) != 0 {
        set_carry(status_flags);
    }

    value >>= 1;
    memory.write_mem_value(operand.wrapping_add(x_val) as u16, value);

    if value == 0 {
        set_zero(status_flags);
    }

    *pc_reg += 2;
 }

 pub fn lsr_absolute(pc_reg : &mut u16, operand: u16, memory: &mut RAM, status_flags: &mut u8){
    let mut value = shift_absolute(operand, memory);

    if (value & 0x01) != 0 {
        set_carry(status_flags);
    }

    value >>= 1;
    let addr = swap_bytes(operand);
    memory.write_mem_value(addr as u16, value);

    if value == 0 {
        set_zero(status_flags);
    }

    *pc_reg += 3;
 }

 pub fn lsr_absolute_x(pc_reg : &mut u16, x_reg : u8, operand: u16, memory: &mut RAM, status_flags: &mut u8){
    let mut value = shift_absolute_x(x_reg, operand, memory);

    if (value & 0x01) != 0 {
        set_carry(status_flags);
    }

    value >>= 1;
    let addr = swap_bytes(operand) + x_reg as u16;
    memory.write_mem_value(addr, value);

    if value == 0 {
        set_zero(status_flags);
    }


    *pc_reg += 3;
 }

 #[cfg(test)]
 mod tests{
     
    #[test]
    fn lsr_tests(){
        use super::*;
        use crate::memory;

       
            let mut operand = 28;
            let mut pc_reg  = 0;
            let mut accumulator = 2;
            let mut status : u8 = 0;
            let mut test_memory : memory::RAM = memory::RAM::new();

            // init mem
            for i in 0..2048 {
                test_memory.write_mem_value(i, i as u8);
            }

            lsr_accumulator(&mut pc_reg, &mut accumulator, &mut status);

            assert_eq!(pc_reg, 1);
            assert_eq!(accumulator, 1);
            assert_eq!(status, 0);
            
            lsr_accumulator(&mut pc_reg, &mut accumulator, &mut status);

            assert_eq!(pc_reg, 2);
            assert_eq!(accumulator, 0);
            assert_eq!(status, 0x3);

            lsr_zero_page(&mut pc_reg, operand, &mut test_memory, &mut status);
            status = 0;
            assert_eq!(pc_reg, 4);
            assert_eq!(test_memory.read_mem_value(operand as u16), 14);
            assert_eq!(status, 0);

            operand = 255;

            lsr_zero_page_x(&mut pc_reg, operand, 2, &mut test_memory, &mut status);
            
            assert_eq!(pc_reg, 6);
            assert_eq!(test_memory.read_mem_value(((operand as u16 + 2) as u16 % 256) as u16), 0);
            assert_eq!(status, 3);

            status = 0;
            lsr_absolute(&mut pc_reg, 0x0201, &mut test_memory, &mut status);

            assert_eq!(pc_reg, 9);
            assert_eq!(test_memory.read_mem_value(0x102), 1);
            assert_eq!(status, 0);

            lsr_absolute_x(&mut pc_reg, 25, 0x0401, &mut test_memory, &mut status);
            assert_eq!(pc_reg, 12);
            assert_eq!(test_memory.read_mem_value(0x104 + 25), 14);
            assert_eq!(status, 1);
        
    }
 }