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
        ((self.ram[addr as usize] as u16) << 8) &
        (self.ram[(addr + 1) as usize] as u16)
    }

    pub fn write_mem_value(&mut self, addr: u16, value : u8){
        check_address(addr as usize);
        self.ram[addr as usize] = value;
    }

    pub fn write_mem_address(&mut self, addr: u16, new_addr : u16){
        check_address(addr as usize);
        self.ram[addr as usize] = (new_addr >> 8) as u8;
        self.ram[(addr + 1) as usize] = (new_addr) as u8; 
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

