pub mod nes_6502;
pub mod mem_map;
pub mod adc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adc_tests() {
        let operand = 12;
        let mut pc_reg  = 1;
        let mut accumulator = 0;
        let mut status : u8 = 0;
        let mut memory  : [u8; 4096] = [0; 4096];

        // init mem
        for i in 0..4096 {
            memory[i as usize] = (i * 2) as u8;
        }

        adc::adc_immediate(operand, &mut pc_reg, &mut accumulator,  &mut status);

        assert_eq!(pc_reg, 2);
        assert_eq!(accumulator, 12);
        assert_eq!(status, 0);

        let operand2 = 255;
        adc::adc_immediate(operand2, &mut pc_reg, &mut accumulator,  &mut status);

        assert_eq!(pc_reg, 3);
        assert_eq!(accumulator, 11);
        assert_eq!(status, 1);

        status = 0;

        adc::adc_zero_page(12, &mut pc_reg, &mut accumulator,  &mut status, &memory);

        assert_eq!(pc_reg, 4);
        assert_eq!(accumulator, 35);
        assert_eq!(status, 0);
    }
}