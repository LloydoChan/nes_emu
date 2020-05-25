//and.rs - for logical AND instructions

use crate::addressing::{self, Operation};
use crate::memory::RAM;

pub fn and_immediate(operand : u8, pc_reg : &mut u16, accumulator: &mut u8, mut status_flags: &mut u8){
    let temp = *accumulator as u16;
    *accumulator = addressing::immediate(temp as u16, operand as u16, None, Operation::And);
    *pc_reg += 2;
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::memory;

    #[test]
    fn tests() {
        let operand = 7;
        let mut pc_reg  = 0;
        let mut accumulator = 15;
        let mut status : u8 = 0;
        let mut test_memory  : memory::RAM = memory::RAM::new();

        // init mem
        for i in 0..2048 {
            test_memory.write_mem_value(i, (i * 2) as u8);
        }

        and_immediate(operand, &mut pc_reg, &mut accumulator,  &mut status);
        assert_eq!(pc_reg, 2);
        assert_eq!(accumulator, 7);
    }   
}
