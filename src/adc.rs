// adc.rs - contains all 8 functions to support adc instructions

// immediate

// TODO change mem reads! so that they use mem control / mapper correctly

pub fn adc_immediate(operand : u8, pc_reg : &mut u16, accumulator: &mut u8, status_flags: &mut u8){
    let mut temp = *accumulator as u16;
    temp += operand as u16;
    
    if temp > 255 {
        *status_flags |= 0x1;
    }

    *accumulator = temp as u8;
    *pc_reg += 1;
}

pub fn adc_zero_page(operand : u8, pc_reg : &mut u16, accumulator: &mut u8, status_flags: &mut u8, memory : &[u8]){
    let mut temp = *accumulator as u16;
    let mem_value = memory[operand as usize];
    temp += mem_value as u16;
    
    if temp > 255 {
        *status_flags |= 0x1;
    }

    *accumulator = temp as u8;
    *pc_reg += 1;
}

pub fn adc_zero_page_x(operand : u8, x_reg : u8, pc_reg : &mut u16, accumulator: &mut u8, status_flags: &mut u8, memory : &[u8]){
    let mut temp = *accumulator as u16;
    let addr :u8 = operand.wrapping_add(x_reg);
    let mem_value = memory[addr as usize];
    temp += mem_value as u16;
    
    if temp > 255 {
        *status_flags |= 0x1;
    }

    *accumulator = temp as u8;
    *pc_reg += 1;
}

pub fn adc_absolute(operand : u16, pc_reg : &mut u16, accumulator: &mut u8, status_flags: &mut u8, memory : &[u8]){
    let mut temp = *accumulator as u16;
    let addr :u8 = operand.wrapping_add(x_reg);
    let mem_value = memory[addr as usize];
    temp += mem_value as u16;
    
    if temp > 255 {
        *status_flags |= 0x1;
    }

    *accumulator = temp as u8;
    *pc_reg += 2;
}