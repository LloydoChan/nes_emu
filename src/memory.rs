// memory access - uses values in mem_map to check what address being passed actually is before 
// returning value
use crate::mem_map::*;

pub struct RAM{
    ram : [u8; 2048],
}

impl RAM {
    pub fn new() -> RAM{
        RAM{
            ram : [0; 2048]
        }
    }

    pub fn read_mem_value(&self, addr: u16) -> u8 {
        check_address(addr as usize);
        self.ram[addr as usize]
    }

    pub fn read_mem_address(&self, addr: u16) -> u16 {
        check_address(addr as usize);
        ((self.ram[(addr + 1) as usize] as u16) << 8) |
        (self.ram[addr as usize] as u16)
    }

    pub fn write_mem_value(&mut self, addr: u16, value : u8){
        check_address(addr as usize);
        self.ram[addr as usize] = value;
    }

    pub fn write_mem_address(&mut self, addr: u16, new_addr : u16){
        check_address(addr as usize);
        self.ram[addr as usize] = (new_addr) as u8;
        self.ram[(addr + 1) as usize] = (new_addr >> 8) as u8; 
    }

    pub fn push_address_on_stack(&mut self, stack_ptr : &mut u8, push_address : u16){
        if *stack_ptr == 254 {
            panic!("stack overflow")
        }

        let addr = STACK_START + *stack_ptr as usize;
        self.ram[addr] = push_address  as u8;
        self.ram[addr + 1] = (push_address >> 8) as u8; 
        //println!("{:#x}, {:#x}, {:#x}", push_address, self.ram[addr], self.ram[addr + 1] );
        *stack_ptr += 2;
    }

    pub fn push_value_on_stack(&mut self, stack_ptr : &mut u8, push_value : u8){
        if *stack_ptr == 255 {
            panic!("stack overflow")
        }

        let addr = STACK_START + *stack_ptr as usize;
        self.ram[addr] = push_value;
        *stack_ptr += 1;
    }

    pub fn pop_address_off_stack(&mut self, stack_ptr : &mut u8) -> u16{
        if *stack_ptr == 0 {
            panic!("stack underflow")
        }

        *stack_ptr -= 2;
        let addr = STACK_START + *stack_ptr as usize;
        let pop_addr = (self.ram[addr + 1] as u16) << 8 | 
                       self.ram[addr] as u16; 
        
        pop_addr
    }

    pub fn pop_value_off_stack(&mut self, stack_ptr : &mut u8) -> u8{

        *stack_ptr -= 1;
        let addr = STACK_START + *stack_ptr as usize;
        self.ram[addr] 
    }
   
}

fn check_address(address: usize){
    match address{
        INTERNAL_RAM_START..=INTERNAL_RAM_END =>{
            println!("ram access {:#x}", address);
        },
        _=> {panic!("{:#x}", address);}
    }
}

pub fn swap_bytes(in_val : u16) -> u16 {
    let out_val = ( in_val << 8 ) | (in_val >> 8);
    out_val
}

#[cfg(test)]
mod tests{
    #[test]
    fn mem_tests(){
        use super::*;

        let mut test_memory  : RAM = RAM::new();
        let mut stack_ptr = 0;

        // init mem
        for i in 0..2048 {
            test_memory.write_mem_value(i, i as u8);
        }

        let byteVal = 0x1FF1;

        let newBytes = swap_bytes(byteVal);
        assert_eq!(newBytes, 0xF11F);

        let value = test_memory.read_mem_value(18);
        assert_eq!(value, 18);

        test_memory.write_mem_value(0x10, 128);
        let value = test_memory.read_mem_value(0x10);
        assert_eq!(value, 128);

        let address = test_memory.read_mem_address(0x4);
        assert_eq!(address, 0x504);

        test_memory.write_mem_address(0x20, 0x3FFF);
        let new_address =  test_memory.read_mem_address(0x20);
        assert_eq!(new_address, 0x3FFF);

        test_memory.push_address_on_stack(&mut stack_ptr, 0x286);
        assert_eq!(stack_ptr, 2);

        let stack_addr = test_memory.pop_address_off_stack(&mut stack_ptr);
        println!("{:#x}", stack_addr);
        assert_eq!(stack_ptr, 0);
        assert_eq!(stack_addr, 0x286);

        test_memory.push_value_on_stack(&mut stack_ptr, 0x86);
        assert_eq!(stack_ptr, 1);

        let stack_val = test_memory.pop_value_off_stack(&mut stack_ptr);
        assert_eq!(stack_val, 0x86);
    }
}