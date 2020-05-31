// 6502.rs for nes-emu
// looking to do a lot of high level stuff regarding the 6502 in here
// will put instructions in different .rs files.
use crate::memory::RAM;
use crate::adc;
use crate::and;
use crate::asl;
use crate::branch;
use crate::misc_instructions;

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
            },
            0x71 => {
               // indirect y
               let addr = ram.read_mem_value(self.pc_counter + 1);
               adc::adc_indirect_indexed(addr, self.y,  &mut self.pc_counter, &mut self.accumulator, &mut self.status_flags, ram, &mut self.cycles_until_next);
            },
            // add with carry end -----------------------------------------
            // ------------------------------------------------------------
            // logical and start-------------------------------------------
            0x29 => {
                // immediate
                let imm_value = ram.read_mem_value(self.pc_counter + 1);
                and::and_immediate(imm_value, &mut self.pc_counter, &mut self.accumulator, &mut self.status_flags,  &mut self.cycles_until_next)
            },
            0x25 => {
                // zero page
                let zero_page_addr = ram.read_mem_value(self.pc_counter + 1);
                and::and_zero_page(zero_page_addr,  &mut self.pc_counter, &mut self.accumulator, &mut self.status_flags, ram, &mut self.cycles_until_next);
            },
            0x35 => {
                // zero page x
                let zero_page_addr = ram.read_mem_value(self.pc_counter + 1);
                and::and_zero_page_x(zero_page_addr, self.x,  &mut self.pc_counter, &mut self.accumulator, &mut self.status_flags, ram, &mut self.cycles_until_next);
            },
            0x2D => {
                // absolute
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                and::and_absolute(absolute_addr,  &mut self.pc_counter, &mut self.accumulator, &mut self.status_flags, ram, &mut self.cycles_until_next);
            },
            0x3D => {
                // absolute x
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                and::and_absolute_reg(absolute_addr, self.x, &mut self.pc_counter, &mut self.accumulator, &mut self.status_flags, ram, &mut self.cycles_until_next);
            },
            0x39 => {
                // absolute y
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                and::and_absolute_reg(absolute_addr, self.y, &mut self.pc_counter, &mut self.accumulator, &mut self.status_flags, ram, &mut self.cycles_until_next);
            },
            0x21 => {
                // indirect x
                let addr = ram.read_mem_value(self.pc_counter + 1);
                and::and_indexed_indirect(addr, self.x,  &mut self.pc_counter, &mut self.accumulator, &mut self.status_flags, ram, &mut self.cycles_until_next);
            },
            0x31 => {
                // indirect y
               let addr = ram.read_mem_value(self.pc_counter + 1);
               and::and_indirect_indexed(addr, self.y,  &mut self.pc_counter, &mut self.accumulator, &mut self.status_flags, ram, &mut self.cycles_until_next);
            },
            // logical and end---------------------------------------------
            // ------------------------------------------------------------
            // arithmetic shift left start
            0x0A => {
                 // absolute
                asl::asl_accumulator(&mut self.pc_counter, &mut self.accumulator, &mut self.status_flags, &mut self.cycles_until_next);
            },
            0x06 => {
                // zero page
                let zero_page_addr = ram.read_mem_value(self.pc_counter + 1);
                asl::asl_zero_page(&mut self.pc_counter, zero_page_addr, ram, &mut self.status_flags, &mut self.cycles_until_next);
            },
            0x16 => {
                // zero page x
                let zero_page_addr = ram.read_mem_value(self.pc_counter + 1);
                asl::asl_zero_page_x(&mut self.pc_counter, self.x, zero_page_addr, ram, &mut self.status_flags, &mut self.cycles_until_next);
            },
            0x0E => {
                // absolute
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                asl::asl_absolute( &mut self.pc_counter, absolute_addr,  ram, &mut self.status_flags, &mut self.cycles_until_next);
            },
            0x1E => {
                // absolute x
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                asl::asl_absolute_x( &mut self.pc_counter, self.x, absolute_addr, ram, &mut self.status_flags, &mut self.cycles_until_next);
            },
            // arithmetic shift left end
            // ------------------------------------------------------------
            // branch instructions start
            0x90 => {
                // branch if carry clear
                let branch_addr = ram.read_mem_value(self.pc_counter + 1);
                branch::branch_if_carry_clear(self.status_flags, &mut self.pc_counter, branch_addr as i8, &mut self.cycles_until_next);
            },
            0xB0 => {
                //branch if carry set
                let branch_addr = ram.read_mem_value(self.pc_counter + 1);
                branch::branch_if_carry_set(self.status_flags, &mut self.pc_counter, branch_addr as i8, &mut self.cycles_until_next);
            },
            0xF0 => {
                //branch if equal
                let branch_addr = ram.read_mem_value(self.pc_counter + 1);
                branch::branch_if_equal(self.status_flags, &mut self.pc_counter, branch_addr as i8, &mut self.cycles_until_next);
            },
            0x30 => {
                // branch if minus
                let branch_addr = ram.read_mem_value(self.pc_counter + 1);
                branch::branch_if_minus(self.status_flags, &mut self.pc_counter, branch_addr as i8, &mut self.cycles_until_next);
            },
            0xD0 => {
                //branch if not equal
                let branch_addr = ram.read_mem_value(self.pc_counter + 1);
                branch::branch_if_not_equal(self.status_flags, &mut self.pc_counter, branch_addr as i8, &mut self.cycles_until_next);
            },
            0x10 => {
                //branch if positive
                let branch_addr = ram.read_mem_value(self.pc_counter + 1);
                branch::branch_if_positive(self.status_flags, &mut self.pc_counter, branch_addr as i8, &mut self.cycles_until_next);
            },
            0x50 => {
                //branch if overflow clear
                let branch_addr = ram.read_mem_value(self.pc_counter + 1);
                branch::branch_if_overflow_clear(self.status_flags, &mut self.pc_counter, branch_addr as i8, &mut self.cycles_until_next);
            },
            0x70 => {
                //branch if overflow set
                let branch_addr = ram.read_mem_value(self.pc_counter + 1);
                branch::branch_if_overflow_set(self.status_flags, &mut self.pc_counter, branch_addr as i8, &mut self.cycles_until_next);
            },
            // branch instruction end
            // -----------------------------------------------------------
            // bit test
            0x24 => {
                // bit test zero page
                let zero_page_addr = ram.read_mem_value(self.pc_counter + 1);
                misc_instructions::bittest_zero_page(&mut self.pc_counter, self.accumulator, zero_page_addr, ram, &mut self.status_flags, &mut self.cycles_until_next);
            },
            0x2C => {
                // bit test absolute
                let addr = ram.read_mem_address(self.pc_counter + 1);
                misc_instructions::bittest_absolute(&mut self.pc_counter, self.accumulator, addr, ram, &mut self.status_flags, &mut self.cycles_until_next);
            },
            0x00 => {
                misc_instructions::break_force_interrupt(&mut self.pc_counter, &mut self.status_flags, &mut self.stack_pointer, ram, &mut self.cycles_until_next);
            }
            //-----------------------------
            // clear flag instructions start
            0x18 => {

            },
            0xD8 => {

            },
            0x58 => {

            },
            0xB8 => {

            },
            // clear flag instructions end
            //-----------------------------
            // set flag instructions start
            0x38 => {

            },
            0xF8 => {

            },
            0x78 => {

            },
            // set flag instructions end
            //----------------------------
            // compare instructions start
            0xC9 => {

            },
            0xC5 => {

            },
            0xD5 => {

            },
            0xCD => {

            },
            0xDD => {

            },
            0xD9 => {

            },
            0xC1 => {

            },
            0xD1 => {

            },
            0xE0 => {

            },
            0xE4 => {

            },
            0xEC => {

            },
            0xC0 => {

            },
            0xC4 => {

            },
            0xCC => {

            },
            // compare instructions end
            // --------------------------
            // decrement instructions start
            0xC6 => {

            },
            0xD6 => {

            },
            0xCE => {

            },
            0xDE => {

            },
            0xCA => {

            },
            0x88 => {

            },
            // decrement instructions end
            // -------------------------
            // increment instructions start
            0xE6 => {

            },
            0xF6 => {

            },
            0xEE => {

            },
            0xFE => {

            },
            0xE8 => {

            },
            0xC8 => {

            },
            // increment instructions end
            // -------------------------
            // jump and return instructions start
            0x4C => {

            },
            0x6C => {

            },
            0x20 => {

            },
            0x40 => {

            },
            0x60 => {

            },
            // jump and return instructions end
            // -------------------------
            // load instructions start
            0xA9 => {

            },
            0xA5 => {

            },
            0xB5 => {

            },
            0xAD => {

            },
            0xBD => {

            },
            0xB9 => {

            },
            0xA1 => {

            },
            0xB1 => {

            },
            0xA2 => {

            },
            0xA6 => {

            },
            0xB6 => {

            },
            0xAE => {

            },
            0xBE => {

            },
            0xA0 => {

            },
            0xA4 => {

            },
            0xB4 => {

            },
            0xAC => {

            },
            0xBC => {

            },
            // load instructions end
            // -------------------------
            // store instructions start
            0x85 => {

            },
            0x95 => {

            },
            0x8D => {

            },
            0x9D => {

            },
            0x99 => {

            },
            0x81 => {

            },
            0x91 => {

            },
            0x86 => {

            },
            0x96 => {

            },
            0x8E => {

            },
            0x84 => {

            },
            0x94 => {

            },
            0x8C => {

            },
            // store instructions end
            // -------------------------
            // logical shift right start
            0x4A => {

            },
            0x46 => {

            },
            0x56 => {

            },
            0x4E => {

            },
            0x5E => {

            },
            // logical shift right end
            // -------------------------
            // exclusive or instructions start
            0x49 => {

            },
            0x45 => {

            },
            0x55 => {

            },
            0x4D => {

            },
            0x5D => {

            },
            0x59 => {

            },
            0x41 => {

            },
            0x51 => {

            },
            // exclusive or instructions end
            // -------------------------
            // inclusive or instruction start
            0x09 => {

            },
            0x05 => {

            },
            0x15 => {

            },
            0x0D => {

            },
            0x1D => {

            },
            0x19 => {

            },
            0x01 => {

            },
            0x11 => {

            },
            // inclusive or instruction end
            // -------------------------
            // push and pull instructions start
            0x48 => {

            },
            0x08 => {

            },
            0x68 => {

            },
            0x28 => {

            },
            // push and pull instructions end
            // -------------------------
            // rotate instructions start
            0x2A => {

            },
            0x26 => {

            },
            0x36 => {

            },
            0x2E => {

            },
            0x3E => {

            },
            0x6A => {

            },
            0x66 => {

            },
            0x76 => {

            },
            0x6E => {

            },
            0x7E => {

            },
            // rotate instructions end
            // -------------------------
            // transfer instructions start
            0xAA => {

            },
            0xA8 => {

            },
            0x8A => {

            },
            0x9A => {

            },
            0x98 => {

            }
            // transfer instructions end
            //-------------------------
            //subc instructions start
            0xE9 => {

            },
            0xE5 => {

            },
            0xF5 => {

            },
            0xED => {

            },
            0xFD => {

            },
            0xF9 => {

            },
            0xE1 => {

            },
            0xF1 => {
                
            }
            //subc instructions end
            //------------------------
            //nop
            0xEA => {

            },
        }
    }
    

}