// 6502.rs for nes-emu
// looking to do a lot of high level stuff regarding the 6502 in here
// will put instructions in different .rs files.
use crate::memory::RAM;
use crate::adc;

#[derive(Debug)]
struct Nes6502{
    accumulator : u8,
    x : u8,
    y : u8,
    status_flags : u8, // only need 7 bits of this
    stack_pointer : u8,
    pc_counter : u16,
    cycles_until_next : u8
}

impl Nes6502 {
    pub fn new() -> Self{
        Nes6502{
            accumulator : 0,
            x : 0,
            y : 0,
            status_flags : 0xFD, // only need 7 bits of this
            stack_pointer : 0,
            pc_counter : 0x34,
            cycles_until_next : 0
        }
    }

    pub fn run(&mut self, ram : &mut RAM) {
        if self.cycles_until_next == 0 {
            // get next opcode
            let opcode = ram.read_mem_value(self.pc_counter);
            self.decode_instruction(opcode, ram);
        }
    }

    fn decode_instruction(&mut self, opcode : u8, ram : &mut RAM){
        match opcode{
            _=> panic!(),
            // -------------------------------------------------------------------
            // add with carry start ---------------------------------------------- 
            0x69 => {
                // immediate
                let imm_value = ram.read_mem_value(self.pc_counter + 1);
                adc::adc_immediate(imm_value, &mut self.pc_counter, &mut self.accumulator, &mut self.status_flags,  &mut self.cycles_until_next);
            },
            0x65 => {
                // zero page
                let zero_page_addr = ram.read_mem_value(self.pc_counter + 1);
                adc::adc_zero_page(zero_page_addr,  &mut self.pc_counter, &mut self.accumulator, &mut self.status_flags, ram, &mut self.cycles_until_next);
            },
            0x75 => {
                // zero page x
                let zero_page_addr = ram.read_mem_value(self.pc_counter + 1);
                adc::adc_zero_page_x(zero_page_addr, self.x,  &mut self.pc_counter, &mut self.accumulator, &mut self.status_flags, ram, &mut self.cycles_until_next);
            },
            0x6D => {
                // absolute
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                adc::adc_absolute(absolute_addr,  &mut self.pc_counter, &mut self.accumulator, &mut self.status_flags, ram, &mut self.cycles_until_next);
            },
            0x7D => {
                 // absolute x
                 let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                 adc::adc_absolute_reg(absolute_addr, self.x,  &mut self.pc_counter, &mut self.accumulator, &mut self.status_flags, ram, &mut self.cycles_until_next);
            },
            0x79 => {
                // absolute y
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                adc::adc_absolute_reg(absolute_addr, self.y,  &mut self.pc_counter, &mut self.accumulator, &mut self.status_flags, ram, &mut self.cycles_until_next);
           },
           0x61 => {
               // indirect x
               let addr = ram.read_mem_value(self.pc_counter + 1);
               adc::adc_indexed_indirect(addr, self.x,  &mut self.pc_counter, &mut self.accumulator, &mut self.status_flags, ram, &mut self.cycles_until_next);
           }
           0x71 => {
               // indirect y
               let addr = ram.read_mem_value(self.pc_counter + 1);
               adc::adc_indirect_indexed(addr, self.y,  &mut self.pc_counter, &mut self.accumulator, &mut self.status_flags, ram, &mut self.cycles_until_next);
           }
            // add with carry end ----------------------------------------
            // ------------------------------------------------------------
        }
    }
    

}