use crate::mem_map::*;
use crate::memory::RAM;

const NAME_TABLE_ADDRS: [usize; 4] = [0x2000, 0x2400, 0x2800, 0x2C00];
const VRAM_INCRS: [usize; 2] = [1, 32];
const PAT_TABLE_ADDR: [usize; 2] = [0, 0x1000];

#[derive(Default)]
pub struct PPU {
    PPUCTRL: ppuCtrl,
    PPUMASK: ppuMask,
    PPUSTATUS: ppuStatus, // read ONLY
    OAMADDR: u8,
    OAMDATA: u8,
    PPUSCROLL: ppuScroll,
    PPUADDR: ppuAddr,
}

fn get_bit(byte: u8, index: u8) -> u8 {
    (byte & (0x1 << index)) >> index
}

impl PPU {
    pub fn run(&mut self, mem: &mut RAM) {
        // run?
        let mut reg = PPU_REGISTERS_START;

        if mem.was_written(0) {
            let ppuCtrlVal = mem.read_mem_value(reg as u16);
            self.updatePpuCtrl(ppuCtrlVal);
        }

        reg += 1;

        if mem.was_written(1) {
            let ppuMaskVal = mem.read_mem_value(reg as u16);
            self.updatePpuMask(ppuMaskVal);
        }

        reg += 1;

        // check for ppu status read
        if mem.was_read(2) {
            self.readPpuStatus();
        }

        reg += 1;

        if mem.was_written(3) {
            let OAMaddr = mem.read_mem_value(reg as u16);
            self.OAMADDR = OAMaddr;
        }

        reg += 1;

        if mem.was_written(4) {
            let OAMdata = mem.read_mem_value(reg as u16);
            // TODO
        }

        reg += 1;

        if mem.was_written(5) {
            let scroll = mem.read_mem_value(reg as u16);
            self.updatePpuScroll(scroll);
        }

        reg += 1;

        if mem.was_written(6) {
            let addr = mem.read_mem_value(reg as u16);
            self.updatePpuAddr(addr);
        }

        reg += 1;

        if mem.was_written(7) {
            let value = mem.read_ppu_data_no_incr();
            // write value to mem address stored in ppu addr
            let addr = self.PPUADDR.address as usize;
            mem.write_vram_value(addr, value);
            self.PPUADDR.address += self.PPUCTRL.VRAM_address_increment;
        }

        if mem.was_read(7) {
            let addr = self.PPUADDR.address as u16;
            let value = mem.read_mem_value(addr);
            mem.write_ppu_data_no_incr(value);
            self.PPUADDR.address += self.PPUCTRL.VRAM_address_increment;
            // TODO potential problem with internal read buffer?
        }

        mem.clear_read_write_regs();
    }

    pub fn updatePpuCtrl(&mut self, byte_val: u8) {
        let name_table_idx = byte_val & 0x03;
        self.PPUCTRL.nametableAddress = NAME_TABLE_ADDRS[name_table_idx as usize];

        let vram_incr = get_bit(byte_val, 2);
        self.PPUCTRL.VRAM_address_increment = VRAM_INCRS[vram_incr as usize];

        let spr_pattern = get_bit(byte_val, 3);
        self.PPUCTRL.sprite_pattern_table_addr = PAT_TABLE_ADDR[spr_pattern as usize];

        let bg_pattern = get_bit(byte_val, 4);
        self.PPUCTRL.bg_pattern_table_addr = PAT_TABLE_ADDR[bg_pattern as usize];

        self.PPUCTRL.sprite_size = get_bit(byte_val, 5) as usize;
        //self.PPUCTRL. = get_bit(byte_val, 6) as usize; MASTER SLAVE? TODO!
        self.PPUCTRL.gen_nmi = get_bit(byte_val, 7) as usize;
    }

    pub fn updatePpuMask(&mut self, byte_val: u8) {
        self.PPUMASK.greyScale = get_bit(byte_val, 0);
        self.PPUMASK.show_bg_left = get_bit(byte_val, 1);
        self.PPUMASK.show_spr_left = get_bit(byte_val, 2);
        self.PPUMASK.show_bg = get_bit(byte_val, 3);
        self.PPUMASK.show_spr = get_bit(byte_val, 4);
        self.PPUMASK.emphasize_red = get_bit(byte_val, 5);
        self.PPUMASK.emphasize_green = get_bit(byte_val, 6);
        self.PPUMASK.emphasize_blue = get_bit(byte_val, 7);
    }

    pub fn readPpuStatus(&mut self) {
        self.PPUSCROLL.write_byte = 0;
        self.PPUADDR.write_byte = 0;
    }

    pub fn updatePpuScroll(&mut self, byte_val: u8) {
        // two writes to this write the address that will be read when checking the nametable to render
        // starting in top left of screen
        if self.PPUSCROLL.write_byte == 0 {
            self.PPUSCROLL.horiz_offset = byte_val;
        } else {
            self.PPUSCROLL.vert_offset = byte_val;
        }
    }

    pub fn updatePpuAddr(&mut self, byte_val: u8) {
        // two writes to this write the address any writes to updatePpuAddr will write to
        // upper byte / big endian first
        if self.PPUADDR.write_byte == 0 {
            self.PPUADDR.address = 0;
            self.PPUADDR.address = ((byte_val as u16) << 8) as usize;
            self.PPUADDR.write_byte += 1;
        } else {
            self.PPUADDR.address |= ((byte_val as u16) as usize);
        }
    }

    pub fn updateOAMAddr(&mut self, byte_val: u8) {
        todo!();
    }

    pub fn updateOAMData(&mut self, byte_val: u8) {
        todo!();
    }
}

struct ppuCtrl {
    nametableAddress: usize,
    VRAM_address_increment: usize,
    sprite_pattern_table_addr: usize,
    bg_pattern_table_addr: usize,
    sprite_size: usize,
    gen_nmi: usize,
}

impl Default for ppuCtrl {
    fn default() -> Self {
        ppuCtrl {
            nametableAddress: NAME_TABLE_ADDRS[0],
            VRAM_address_increment: VRAM_INCRS[0],
            sprite_pattern_table_addr: PAT_TABLE_ADDR[0],
            bg_pattern_table_addr: PAT_TABLE_ADDR[0],
            sprite_size: 0,
            gen_nmi: 0,
        }
    }
}

#[derive(Default)]
struct ppuMask {
    greyScale: u8,
    show_bg_left: u8,
    show_spr_left: u8,
    show_bg: u8,
    show_spr: u8,
    emphasize_red: u8,
    emphasize_green: u8,
    emphasize_blue: u8,
}

#[derive(Default)]
struct ppuStatus {
    overflow: u8,
    sprite_hit: u8,
    vert_blank_started: u8,
}

#[derive(Default)]
struct ppuScroll {
    horiz_offset: u8,
    vert_offset: u8,
    write_byte: u8,
}

#[derive(Default)]
struct ppuAddr {
    address: usize,
    write_byte: u8,
}

#[cfg(test)]
pub mod Test {

    use super::*;
    use crate::memory::RAM;

    #[test]
    pub fn ppu_tests() {
        let mut test_memory: RAM = RAM::new();
        let mut ppu: PPU = PPU::default();

        // let's test ppu address writes...
        test_memory.write_mem_value(PPUADDR as u16, 0x01);
        ppu.run(&mut test_memory);
        test_memory.write_mem_value(PPUADDR as u16, 0x02);
        ppu.run(&mut test_memory);

        test_memory.write_mem_value(PPUDATA as u16, 255);
        ppu.run(&mut test_memory);
        let test_val = test_memory.read_vram_value(0x0102);
        ppu.run(&mut test_memory);
        assert_eq!(test_val, 255);

        test_memory.write_mem_value(PPUDATA as u16, 255);
        ppu.run(&mut test_memory);
        let test_val = test_memory.read_vram_value(0x0103);
        ppu.run(&mut test_memory);
        assert_eq!(test_val, 255);

        // test the y increment mode
        let status = 0b00000100;
        test_memory.write_mem_value(PPUCTRL as u16, status);
        ppu.run(&mut test_memory);

        test_memory.write_mem_value(PPUDATA as u16, 255);
        ppu.run(&mut test_memory);
        let test_val = test_memory.read_vram_value(0x0104);
        ppu.run(&mut test_memory);
        assert_eq!(test_val, 255);
        test_memory.write_mem_value(PPUDATA as u16, 255);
        ppu.run(&mut test_memory);
        let test_val = test_memory.read_vram_value(0x0124);

        assert_eq!(test_val, 255);
    }
}
