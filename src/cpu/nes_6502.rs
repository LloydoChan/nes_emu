// 6502.rs for nes-emu
// looking to do a lot of high level stuff regarding the 6502 in here
// will put instructions in different .rs files.
use super::adc;
use super::and;
use super::asl;
use super::branch;
use super::compare;
use super::flags;
use super::increment_decrement;
use super::jumps;
use super::load_store;
use super::lsr;
use super::misc_instructions;
use super::or;
use super::rol;
use super::ror;
use super::sub;
use crate::memory::RAM;

#[derive(Debug)]
pub struct Nes6502 {
    accumulator: u8,
    x: u8,
    y: u8,
    status_flags: u8, // only need 7 bits of this
    stack_pointer: u8,
    pc_counter: u16,
    cycles_until_next: u8,
    total_cycles: u16,
}

impl Nes6502 {
    pub fn new() -> Self {
        Nes6502 {
            accumulator: 0,
            x: 0,
            y: 0,
            status_flags: 0x24, // only need 7 bits of this
            stack_pointer: 0xFD,
            pc_counter: 0xc000,
            cycles_until_next: 0,
            total_cycles: 7,
        }
    }

    pub fn run(&mut self, ram: &mut RAM) {
        if self.cycles_until_next == 0 {
            // get next opcode
            let opcode = ram.read_mem_value(self.pc_counter);
            self.decode_instruction(opcode, ram);
        } else {
                self.cycles_until_next -= 1;
                self.total_cycles += 1;
        }
    }

    fn decode_instruction(&mut self, opcode: u8, ram: &mut RAM) {
        println!("{:#x} {:#04x} A:{:#04x} X:{:#04x} Y:{:#04x} P:{:#04x} SP:{:#04x} cycles:{}", self.pc_counter, opcode, self.accumulator, self.x, self.y, self.status_flags, self.stack_pointer, self.total_cycles);
        match opcode {
            // -------------------------------------------------------------------
            // add with carry start ----------------------------------------------
            0x69 => {
                // immediate
                let imm_value = ram.read_mem_value(self.pc_counter + 1);
                adc::adc_immediate(
                    imm_value,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0x65 => {
                // zero page
                let zero_page_addr = ram.read_mem_value(self.pc_counter + 1);
                adc::adc_zero_page(
                    zero_page_addr,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x75 => {
                // zero page x
                let zero_page_addr = ram.read_mem_value(self.pc_counter + 1);
                adc::adc_zero_page_x(
                    zero_page_addr,
                    self.x,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x6D => {
                // absolute
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                adc::adc_absolute(
                    absolute_addr,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x7D => {
                // absolute x
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                adc::adc_absolute_reg(
                    absolute_addr,
                    self.x,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x79 => {
                // absolute y
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                adc::adc_absolute_reg(
                    absolute_addr,
                    self.y,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x61 => {
                // indirect x
                let addr = ram.read_mem_value(self.pc_counter + 1);
                adc::adc_indexed_indirect(
                    addr,
                    self.x,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x71 => {
                // indirect y
                let addr = ram.read_mem_value(self.pc_counter + 1);
                adc::adc_indirect_indexed(
                    addr,
                    self.y,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            // add with carry end -----------------------------------------
            // ------------------------------------------------------------
            // logical and start-------------------------------------------
            0x29 => {
                // immediate
                let imm_value = ram.read_mem_value(self.pc_counter + 1);
                and::and_immediate(
                    imm_value,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                )
            }
            0x25 => {
                // zero page
                let zero_page_addr = ram.read_mem_value(self.pc_counter + 1);
                and::and_zero_page(
                    zero_page_addr,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x35 => {
                // zero page x
                let zero_page_addr = ram.read_mem_value(self.pc_counter + 1);
                and::and_zero_page_x(
                    zero_page_addr,
                    self.x,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x2D => {
                // absolute
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                and::and_absolute(
                    absolute_addr,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x3D => {
                // absolute x
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                and::and_absolute_reg(
                    absolute_addr,
                    self.x,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x39 => {
                // absolute y
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                and::and_absolute_reg(
                    absolute_addr,
                    self.y,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x21 => {
                // indirect x
                let addr = ram.read_mem_value(self.pc_counter + 1);
                and::and_indexed_indirect(
                    addr,
                    self.x,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x31 => {
                // indirect y
                let addr = ram.read_mem_value(self.pc_counter + 1);
                and::and_indirect_indexed(
                    addr,
                    self.y,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            // logical and end---------------------------------------------
            // ------------------------------------------------------------
            // arithmetic shift left start
            0x0A => {
                // absolute
                asl::asl_accumulator(
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0x06 => {
                // zero page
                let zero_page_addr = ram.read_mem_value(self.pc_counter + 1);
                asl::asl_zero_page(
                    &mut self.pc_counter,
                    zero_page_addr,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0x16 => {
                // zero page x
                let zero_page_addr = ram.read_mem_value(self.pc_counter + 1);
                asl::asl_zero_page_x(
                    &mut self.pc_counter,
                    self.x,
                    zero_page_addr,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0x0E => {
                // absolute
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                asl::asl_absolute(
                    &mut self.pc_counter,
                    absolute_addr,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0x1E => {
                // absolute x
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                asl::asl_absolute_x(
                    &mut self.pc_counter,
                    self.x,
                    absolute_addr,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            // arithmetic shift left end
            // ------------------------------------------------------------
            // branch instructions start
            0x90 => {
                // branch if carry clear
                let branch_addr = ram.read_mem_value(self.pc_counter + 1);
                branch::branch_if_carry_clear(
                    self.status_flags,
                    &mut self.pc_counter,
                    branch_addr as i8,
                    &mut self.cycles_until_next,
                );
            }
            0xB0 => {
                //branch if carry set
                let branch_addr = ram.read_mem_value(self.pc_counter + 1);
                branch::branch_if_carry_set(
                    self.status_flags,
                    &mut self.pc_counter,
                    branch_addr as i8,
                    &mut self.cycles_until_next,
                );
            }
            0xF0 => {
                //branch if equal
                let branch_addr = ram.read_mem_value(self.pc_counter + 1);
                branch::branch_if_equal(
                    self.status_flags,
                    &mut self.pc_counter,
                    branch_addr as i8,
                    &mut self.cycles_until_next,
                );
            }
            0x30 => {
                // branch if minus
                let branch_addr = ram.read_mem_value(self.pc_counter + 1);
                branch::branch_if_minus(
                    self.status_flags,
                    &mut self.pc_counter,
                    branch_addr as i8,
                    &mut self.cycles_until_next,
                );
            }
            0xD0 => {
                //branch if not equal
                let branch_addr = ram.read_mem_value(self.pc_counter + 1);
                branch::branch_if_not_equal(
                    self.status_flags,
                    &mut self.pc_counter,
                    branch_addr as i8,
                    &mut self.cycles_until_next,
                );
            }
            0x10 => {
                //branch if positive
                let branch_addr = ram.read_mem_value(self.pc_counter + 1);
                branch::branch_if_positive(
                    self.status_flags,
                    &mut self.pc_counter,
                    branch_addr as i8,
                    &mut self.cycles_until_next,
                );
            }
            0x50 => {
                //branch if overflow clear
                let branch_addr = ram.read_mem_value(self.pc_counter + 1);
                branch::branch_if_overflow_clear(
                    self.status_flags,
                    &mut self.pc_counter,
                    branch_addr as i8,
                    &mut self.cycles_until_next,
                );
            }
            0x70 => {
                //branch if overflow set
                let branch_addr = ram.read_mem_value(self.pc_counter + 1);
                branch::branch_if_overflow_set(
                    self.status_flags,
                    &mut self.pc_counter,
                    branch_addr as i8,
                    &mut self.cycles_until_next,
                );
            }
            // branch instruction end
            // -----------------------------------------------------------
            // bit test
            0x24 => {
                // bit test zero page
                let zero_page_addr = ram.read_mem_value(self.pc_counter + 1);
                misc_instructions::bittest_zero_page(
                    &mut self.pc_counter,
                    self.accumulator,
                    zero_page_addr,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0x2C => {
                // bit test absolute
                let addr = ram.read_mem_address(self.pc_counter + 1);
                misc_instructions::bittest_absolute(
                    &mut self.pc_counter,
                    self.accumulator,
                    addr,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0x00 => {
                misc_instructions::break_force_interrupt(
                    &mut self.pc_counter,
                    &mut self.status_flags,
                    &mut self.stack_pointer,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            //-----------------------------
            // clear flag instructions start
            0x18 => {
                flags::clear_carry(&mut self.status_flags);
                self.pc_counter += 1;
                self.cycles_until_next = 2;
            }
            0x58 => {
                flags::clear_interrupt_disable(&mut self.status_flags);
                self.pc_counter += 1;
                self.cycles_until_next = 2;
            }
            0xB8 => {
                flags::clear_overflow(&mut self.status_flags);
                self.pc_counter += 1;
                self.cycles_until_next = 2;
            }
            // clear flag instructions end
            //-----------------------------
            // set flag instructions start
            0x38 => {
                flags::set_carry(&mut self.status_flags);
                self.pc_counter += 1;
                self.cycles_until_next = 2;
            }
            0x78 => {
                flags::set_interrupt_disable(&mut self.status_flags);
                self.pc_counter += 1;
                self.cycles_until_next = 2;
            }
            0xf8 => {
                flags::set_decimal(&mut self.status_flags);
                self.pc_counter += 1;
                self.cycles_until_next = 2;
            }
            0xd8 => {
                flags::clear_decimal(&mut self.status_flags);
                self.pc_counter += 1;
                self.cycles_until_next = 2;
            }
            // set flag instructions end
            //----------------------------
            // compare instructions start
            0xC9 => {
                // immediate
                let imm_value = ram.read_mem_value(self.pc_counter + 1);
                compare::comp_value_immediate(
                    &mut self.pc_counter,
                    self.accumulator,
                    imm_value,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xC5 => {
                // zero page
                let operand = ram.read_mem_value(self.pc_counter + 1);
                compare::comp_value_zero_page(
                    &mut self.pc_counter,
                    self.accumulator,
                    operand,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xD5 => {
                // zero page x
                let operand = ram.read_mem_value(self.pc_counter + 1);
                compare::comp_value_zero_page_x(
                    &mut self.pc_counter,
                    self.accumulator,
                    operand,
                    self.x,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xCD => {
                // absolute
                let operand = ram.read_mem_address(self.pc_counter + 1);
                compare::comp_value_absolute(
                    &mut self.pc_counter,
                    self.accumulator,
                    operand,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xDD => {
                // absolute x
                let operand = ram.read_mem_address(self.pc_counter + 1);
                compare::comp_value_absolute_reg(
                    &mut self.pc_counter,
                    self.accumulator,
                    operand,
                    self.x,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xD9 => {
                // absolute y
                let operand = ram.read_mem_address(self.pc_counter + 1);
                compare::comp_value_absolute_reg(
                    &mut self.pc_counter,
                    self.accumulator,
                    operand,
                    self.y,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xC1 => {
                // indirect x
                let operand = ram.read_mem_value(self.pc_counter + 1);
                compare::comp_value_indexed_indirect(
                    &mut self.pc_counter,
                    self.accumulator,
                    operand,
                    self.x,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xD1 => {
                // indirect y
                let operand = ram.read_mem_value(self.pc_counter + 1);
                compare::comp_value_indirect_indexed(
                    &mut self.pc_counter,
                    self.accumulator,
                    operand,
                    self.y,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xE0 => {
                // compare x reg immediate
                let imm_value = ram.read_mem_value(self.pc_counter + 1);
                compare::comp_value_immediate(
                    &mut self.pc_counter,
                    self.x,
                    imm_value,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xE4 => {
                // compare x reg zero page
                let operand = ram.read_mem_value(self.pc_counter + 1);
                compare::comp_value_zero_page(
                    &mut self.pc_counter,
                    self.x,
                    operand,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xEC => {
                // compare x reg absolute
                let operand = ram.read_mem_address(self.pc_counter + 1);
                compare::comp_value_absolute(
                    &mut self.pc_counter,
                    self.x,
                    operand,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xC0 => {
                // compare y reg immediate
                let imm_value = ram.read_mem_value(self.pc_counter + 1);
                compare::comp_value_immediate(
                    &mut self.pc_counter,
                    self.y,
                    imm_value,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xC4 => {
                // compare y reg zero page
                let operand = ram.read_mem_value(self.pc_counter + 1);
                compare::comp_value_zero_page(
                    &mut self.pc_counter,
                    self.y,
                    operand,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xCC => {
                // compare y reg absolute
                let operand = ram.read_mem_address(self.pc_counter + 1);
                compare::comp_value_absolute(
                    &mut self.pc_counter,
                    self.y,
                    operand,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            // compare instructions end
            // --------------------------
            // decrement instructions start
            0xC6 => {
                // zero page
                let page = ram.read_mem_value(self.pc_counter + 1);
                increment_decrement::incdec_memory_zero_page(
                    &mut self.pc_counter,
                    page,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                    increment_decrement::Operation::Dec,
                );
            }
            0xD6 => {
                //zero page x
                let page = ram.read_mem_value(self.pc_counter + 1);
                increment_decrement::incdec_memory_zero_page_x(
                    &mut self.pc_counter,
                    page,
                    self.x,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                    increment_decrement::Operation::Dec,
                );
            }
            0xCE => {
                //absolute
                let addr = ram.read_mem_address(self.pc_counter + 1);
                increment_decrement::incdec_memory_absolute(
                    &mut self.pc_counter,
                    addr,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                    increment_decrement::Operation::Dec,
                );
            }
            0xDE => {
                // absolute x
                let addr = ram.read_mem_address(self.pc_counter + 1);
                increment_decrement::incdec_memory_absolute_x(
                    &mut self.pc_counter,
                    addr,
                    self.x,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                    increment_decrement::Operation::Dec,
                );
            }
            0xCA => {
                // dec x
                increment_decrement::incdec_reg(
                    &mut self.pc_counter,
                    &mut self.x,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                    increment_decrement::Operation::Dec,
                );
            }
            0x88 => {
                //dec y
                increment_decrement::incdec_reg(
                    &mut self.pc_counter,
                    &mut self.y,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                    increment_decrement::Operation::Dec,
                );
            }
            // decrement instructions end
            // -------------------------
            // increment instructions start
            0xE6 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                increment_decrement::incdec_memory_zero_page(
                    &mut self.pc_counter,
                    zero_page,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                    increment_decrement::Operation::Inc,
                )
            }
            0xF6 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                increment_decrement::incdec_memory_zero_page_x(
                    &mut self.pc_counter,
                    zero_page,
                    self.x,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                    increment_decrement::Operation::Inc,
                )
            }
            0xEE => {
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                increment_decrement::incdec_memory_absolute(
                    &mut self.pc_counter,
                    absolute_addr,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                    increment_decrement::Operation::Inc,
                );
            }
            0xFE => {
                let addr = ram.read_mem_address(self.pc_counter + 1);
                increment_decrement::incdec_memory_absolute_x(
                    &mut self.pc_counter,
                    addr,
                    self.x,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                    increment_decrement::Operation::Inc,
                );
            }
            0xE8 => {
                increment_decrement::incdec_reg(
                    &mut self.pc_counter,
                    &mut self.x,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                    increment_decrement::Operation::Inc,
                );
            }
            0xC8 => {
                increment_decrement::incdec_reg(
                    &mut self.pc_counter,
                    &mut self.y,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                    increment_decrement::Operation::Inc,
                );
            }
            // increment instructions end
            // -------------------------
            // jump and return instructions start
            0x4C => {
                // jump absolute
                let mem_addr = ram.read_mem_address(self.pc_counter + 1);
                jumps::jump_absolute(&mut self.pc_counter, mem_addr, &mut self.cycles_until_next);
            }
            0x6C => {
                // jump indirect
                let mem_addr = ram.read_mem_address(self.pc_counter + 1);
                jumps::jump_indirect(
                    &mut self.pc_counter,
                    mem_addr,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x20 => {
                // jump subroutine absolute
                let mem_addr = ram.read_mem_address(self.pc_counter + 1);
                jumps::jump_subroutine(
                    &mut self.pc_counter,
                    mem_addr,
                    &mut self.stack_pointer,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x40 => {
                jumps::return_from_interrupt(
                    &mut self.pc_counter,
                    &mut self.stack_pointer,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x60 => {
                //return from subroutine
                jumps::return_from_subroutine(
                    &mut self.pc_counter,
                    &mut self.stack_pointer,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            // jump and return instructions end
            // -------------------------
            // load instructions start
            0xA9 => {
                let immediate = ram.read_mem_value(self.pc_counter + 1);
                self.accumulator = immediate;
                load_store::set_flags(immediate, &mut self.status_flags);
                self.pc_counter += 2;
                self.cycles_until_next = 2;
            }
            0xA5 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                self.accumulator = load_store::load_zero_page(
                    &mut self.pc_counter,
                    zero_page,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xB5 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                self.accumulator = load_store::load_zero_page_reg(
                    &mut self.pc_counter,
                    zero_page,
                    self.x,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xAD => {
                let absolute = ram.read_mem_address(self.pc_counter + 1);
                self.accumulator = load_store::absolute_load(
                    &mut self.pc_counter,
                    absolute,
                    0,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xBD => {
                let absolute = ram.read_mem_address(self.pc_counter + 1);
                self.accumulator = load_store::absolute_load(
                    &mut self.pc_counter,
                    absolute,
                    self.x,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xB9 => {
                let absolute = ram.read_mem_address(self.pc_counter + 1);
                self.accumulator = load_store::absolute_load(
                    &mut self.pc_counter,
                    absolute,
                    self.y,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xA1 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                self.accumulator = load_store::indirect_x_load(
                    &mut self.pc_counter,
                    zero_page,
                    self.x,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                )
            }
            0xB1 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                self.accumulator = load_store::indirect_y_load(
                    &mut self.pc_counter,
                    zero_page,
                    self.y,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                )
            }
            0xA2 => {
                let immediate = ram.read_mem_value(self.pc_counter + 1);
                self.x = immediate;
                load_store::set_flags(immediate, &mut self.status_flags);
                self.pc_counter += 2;
                self.cycles_until_next = 2;
            }
            0xA6 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                self.x = load_store::load_zero_page(
                    &mut self.pc_counter,
                    zero_page,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xB6 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                self.x = load_store::load_zero_page_reg(
                    &mut self.pc_counter,
                    zero_page,
                    self.y,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xAE => {
                let absolute = ram.read_mem_address(self.pc_counter + 1);
                self.x = load_store::absolute_load(
                    &mut self.pc_counter,
                    absolute,
                    0,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xBE => {
                let absolute = ram.read_mem_address(self.pc_counter + 1);
                self.x = load_store::absolute_load(
                    &mut self.pc_counter,
                    absolute,
                    self.y,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xA0 => {
                let immediate = ram.read_mem_value(self.pc_counter + 1);
                self.y = immediate;
                load_store::set_flags(immediate, &mut self.status_flags);
                self.pc_counter += 2;
                self.cycles_until_next = 2;
            }
            0xA4 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                self.y = load_store::load_zero_page(
                    &mut self.pc_counter,
                    zero_page,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xB4 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                self.y = load_store::load_zero_page_reg(
                    &mut self.pc_counter,
                    zero_page,
                    self.x,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xAC => {
                let absolute = ram.read_mem_address(self.pc_counter + 1);
                self.y = load_store::absolute_load(
                    &mut self.pc_counter,
                    absolute,
                    0,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xBC => {
                let absolute = ram.read_mem_address(self.pc_counter + 1);
                self.y = load_store::absolute_load(
                    &mut self.pc_counter,
                    absolute,
                    self.x,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            // load instructions end
            // -------------------------
            // store instructions start
            0x85 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                load_store::store_zero_page(
                    &mut self.pc_counter,
                    self.accumulator,
                    zero_page,
                    0,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x95 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                load_store::store_zero_page(
                    &mut self.pc_counter,
                    self.accumulator,
                    zero_page,
                    self.x,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x8D => {
                let absolute = ram.read_mem_address(self.pc_counter + 1);
                load_store::store_absolute(
                    &mut self.pc_counter,
                    self.accumulator,
                    absolute,
                    0,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x9D => {
                let absolute = ram.read_mem_address(self.pc_counter + 1);
                load_store::store_absolute(
                    &mut self.pc_counter,
                    self.accumulator,
                    absolute,
                    self.x,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x99 => {
                let absolute = ram.read_mem_address(self.pc_counter + 1);
                load_store::store_absolute(
                    &mut self.pc_counter,
                    self.accumulator,
                    absolute,
                    self.y,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x81 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                load_store::store_indirect_x(
                    &mut self.pc_counter,
                    self.accumulator,
                    zero_page,
                    self.x,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x91 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                load_store::store_indirect_y(
                    &mut self.pc_counter,
                    self.accumulator,
                    zero_page,
                    self.y,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x86 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                load_store::store_zero_page(
                    &mut self.pc_counter,
                    self.x,
                    zero_page,
                    0,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x96 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                load_store::store_zero_page(
                    &mut self.pc_counter,
                    self.x,
                    zero_page,
                    self.y,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x8E => {
                let absolute = ram.read_mem_address(self.pc_counter + 1);
                load_store::store_absolute(
                    &mut self.pc_counter,
                    self.x,
                    absolute,
                    0,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x84 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                load_store::store_zero_page(
                    &mut self.pc_counter,
                    self.y,
                    zero_page,
                    0,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x94 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                load_store::store_zero_page(
                    &mut self.pc_counter,
                    self.y,
                    zero_page,
                    self.x,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x8C => {
                let absolute = ram.read_mem_address(self.pc_counter + 1);
                load_store::store_absolute(
                    &mut self.pc_counter,
                    self.y,
                    absolute,
                    0,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            // store instructions end
            // -------------------------
            // logical shift right start
            0x4A => lsr::lsr_accumulator(
                &mut self.pc_counter,
                &mut self.accumulator,
                &mut self.status_flags,
                &mut self.cycles_until_next,
            ),
            0x46 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                lsr::lsr_zero_page(
                    &mut self.pc_counter,
                    zero_page,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0x56 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                lsr::lsr_zero_page_x(
                    &mut self.pc_counter,
                    self.x,
                    zero_page,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0x4E => {
                let absolute = ram.read_mem_address(self.pc_counter + 1);
                lsr::lsr_absolute(
                    &mut self.pc_counter,
                    absolute,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0x5E => {
                let absolute = ram.read_mem_address(self.pc_counter + 1);
                lsr::lsr_absolute_x(
                    &mut self.pc_counter,
                    self.x,
                    absolute,
                    ram,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            // logical shift right end
            // -------------------------
            // exclusive or instructions start
            0x49 => {
                let immediate = ram.read_mem_value(self.pc_counter + 1);
                or::xor_immediate(
                    immediate,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0x45 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                or::xor_zero_page(
                    zero_page,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x55 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                or::xor_zero_page_x(
                    zero_page,
                    self.x,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x4D => {
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                or::xor_absolute(
                    absolute_addr,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x5D => {
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                or::xor_absolute_reg(
                    absolute_addr,
                    self.x,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x59 => {
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                or::xor_absolute_reg(
                    absolute_addr,
                    self.y,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x41 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                or::xor_indexed_indirect(
                    zero_page,
                    self.x,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x51 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                or::xor_indirect_indexed(
                    zero_page,
                    self.y,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            // exclusive or instructions end
            // -------------------------
            // inclusive or instruction start
            0x09 => {
                let immediate = ram.read_mem_value(self.pc_counter + 1);
                or::ior_immediate(
                    immediate,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0x05 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                or::ior_zero_page(
                    zero_page,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x15 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                or::ior_zero_page_x(
                    zero_page,
                    self.x,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x0D => {
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                or::ior_absolute(
                    absolute_addr,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x1D => {
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                or::ior_absolute_reg(
                    absolute_addr,
                    self.x,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x19 => {
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                or::ior_absolute_reg(
                    absolute_addr,
                    self.y,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x01 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                or::ior_indexed_indirect(
                    zero_page,
                    self.x,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x11 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                or::ior_indirect_indexed(
                    zero_page,
                    self.y,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            // inclusive or instruction end
            // -------------------------
            // push and pull instructions start
            0x48 => {
                misc_instructions::push_acc_on_stack(
                    &mut self.pc_counter,
                    self.accumulator,
                    &mut self.stack_pointer,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x08 => {
                misc_instructions::push_status_on_stack(
                    &mut self.pc_counter,
                    self.status_flags,
                    &mut self.stack_pointer,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x68 => {
                misc_instructions::pull_acc_from_stack(
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    &mut self.stack_pointer,
                    ram,
                    &mut self.cycles_until_next,
                );
                //println!("{:#b} {:#b}", self.accumulator, self.status_flags);
            }
            0x28 => {
                misc_instructions::pull_status_from_stack(
                    &mut self.pc_counter,
                    &mut self.status_flags,
                    &mut self.stack_pointer,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            // push and pull instructions end
            // -------------------------
            // rotate instructions start
            0x2A => {
                rol::rol_accumulator(
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0x26 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                rol::rol_zero_page(
                    &mut self.pc_counter,
                    zero_page,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x36 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                rol::rol_zero_page_x(
                    &mut self.pc_counter,
                    self.x,
                    zero_page,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x2E => {
                let absolute = ram.read_mem_address(self.pc_counter + 1);
                rol::rol_absolute(
                    &mut self.pc_counter,
                    absolute,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x3E => {
                let absolute = ram.read_mem_address(self.pc_counter + 1);
                rol::rol_absolute_x(
                    &mut self.pc_counter,
                    self.x,
                    absolute,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x6A => {
                ror::ror_accumulator(
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0x66 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                ror::ror_zero_page(
                    &mut self.pc_counter,
                    zero_page,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x76 => {
                let zero_page = ram.read_mem_value(self.pc_counter + 1);
                ror::ror_zero_page_x(
                    &mut self.pc_counter,
                    self.x,
                    zero_page,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x6E => {
                let absolute = ram.read_mem_address(self.pc_counter + 1);
                ror::ror_absolute(
                    &mut self.pc_counter,
                    absolute,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0x7E => {
                let absolute = ram.read_mem_address(self.pc_counter + 1);
                ror::ror_absolute_x(
                    &mut self.pc_counter,
                    self.x,
                    absolute,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            // rotate instructions end
            // -------------------------
            // transfer instructions start
            0xAA => {
                // transfer acc to x
                misc_instructions::transfer_source_to_dest(
                    &mut self.pc_counter,
                    self.accumulator,
                    &mut self.x,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xA8 => {
                // transfer acc to y
                misc_instructions::transfer_source_to_dest(
                    &mut self.pc_counter,
                    self.accumulator,
                    &mut self.y,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0x8A => {
                // transfer x to acc
                misc_instructions::transfer_source_to_dest(
                    &mut self.pc_counter,
                    self.x,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xBA => {
                // transfer stack pointer to x
                misc_instructions::transfer_source_to_dest(
                    &mut self.pc_counter,
                    self.stack_pointer,
                    &mut self.x,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0x9A => {
                // transfer x to stack pointer
                misc_instructions::transfer_x_to_stack_pointer(
                    &mut self.pc_counter,
                    self.x,
                    &mut self.stack_pointer,
                    &mut self.cycles_until_next,
                );
            }
            0x98 => {
                // transfer y to acc
                misc_instructions::transfer_source_to_dest(
                    &mut self.pc_counter,
                    self.y,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            // transfer instructions end
            //-------------------------
            //subc instructions start
            0xE9 => {
                let imm_value = ram.read_mem_value(self.pc_counter + 1);
                sub::sbc_immediate(
                    imm_value,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    &mut self.cycles_until_next,
                );
            }
            0xE5 => {
                // zero page
                let zero_page_addr = ram.read_mem_value(self.pc_counter + 1);
                sub::sbc_zero_page(
                    zero_page_addr,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0xF5 => {
                // zero page x
                let zero_page_addr = ram.read_mem_value(self.pc_counter + 1);
                sub::sbc_zero_page_x(
                    zero_page_addr,
                    self.x,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0xED => {
                // absolute
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                sub::sbc_absolute(
                    absolute_addr,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0xFD => {
                // absolute
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                sub::sbc_absolute_reg(
                    absolute_addr,
                    self.x,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0xF9 => {
                // absolute
                let absolute_addr = ram.read_mem_address(self.pc_counter + 1);
                sub::sbc_absolute_reg(
                    absolute_addr,
                    self.y,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0xE1 => {
                // indirect x
                let addr = ram.read_mem_value(self.pc_counter + 1);
                sub::sbc_indexed_indirect(
                    addr,
                    self.x,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            0xF1 => {
                let addr = ram.read_mem_value(self.pc_counter + 1);
                sub::sbc_indirect_indexed(
                    addr,
                    self.y,
                    &mut self.pc_counter,
                    &mut self.accumulator,
                    &mut self.status_flags,
                    ram,
                    &mut self.cycles_until_next,
                );
            }
            //subc instructions end
            //------------------------
            //nop
            0xEA | 0xFA | 0x1A | 0x5A | 0x7A | 0xDA | 0x3A | 0xFA => {
                misc_instructions::NOP(&mut self.pc_counter, &mut self.cycles_until_next);
            }
            // unofficial IGN nop 3 byte
            0x0C | 0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => {
                self.pc_counter += 3;
                self.cycles_until_next = 5;
            }
            // unofficial IGN nop 2 bytes
            0x04 | 0x44 | 0x64 | 0x14 | 0x34 | 0x54 | 0x74 | 0xD4 | 0xF4 | 0x80 => {
                self.pc_counter += 2;
                self.cycles_until_next = 4;
            }
            // misc
            0xD8 | 0xB8 | 0xF8 => {
                self.pc_counter += 1;
                self.cycles_until_next = 2;
            }
            _ => panic!("{:#x}", opcode),
        }
    }
}
