use crate::memory::RAM;
use crate::mem_map::*;

const NAME_TABLE_ADDRS : [usize; 4] = [0x2000, 0x2400, 0x2800, 0x2C00]; 
const VRAM_INCRS : [usize; 2] = [1, 32];
const PAT_TABLE_ADDR : [usize; 2] = [0, 0x1000];

#[derive(Default)]
pub struct PPU{
    PPUCTRL : ppuCtrl,
    PPUMASK : ppuMask,
    PPUSTATUS : ppuStatus, // read ONLY
    OAMADDR : u8,
    OAMDATA : u8,
    PPUSCROLL : ppuScroll,
    PPUADDR : ppuAddr,

    reg_write : [u8; 8]
}

fn get_bit(byte : u8, index: u8) -> u8 {
    (byte & (0x1 << index)) >> index
}

impl PPU {
    
    pub fn run(&mut self, mem : &RAM){
        // run?
        let mut reg = PPU_REGISTERS_START;

        if self.reg_write[0] != 0 {
            let ppuCtrlVal = mem.read_mem_value(reg as u16);
            self.updatePpuCtrl(ppuCtrlVal);
        }
        
        reg += 1;
        
        if self.reg_write[1] != 0 {
            let ppuMaskVal = mem.read_mem_value(reg as u16);
            self.updatePpuMask(ppuMaskVal);
        }

        reg += 1;
        
        if self.reg_write[2] != 0 {
            let ppuStatus = mem.read_mem_value(reg as u16);
            self.updatePpuStatus(ppuStatus);
        }


        reg += 1;
        
        if self.reg_write[3] != 0 {
            let OAMaddr = mem.read_mem_value(reg as u16);
            self.OAMADDR = OAMaddr;
        }

        reg += 1;
        
        if self.reg_write[4] != 0 {
            let OAMdata = mem.read_mem_value(reg as u16);
            // TODO
        }

        reg += 1;
        
        if self.reg_write[5] != 0 {
            let scroll = mem.read_mem_value(reg as u16);
            self.updatePpuScroll(scroll);
        }

        reg += 1;
        
        if self.reg_write[6] != 0 {
            let addr = mem.read_mem_value(reg as u16);
            self.updatePpuAddr(addr);
        }

        reg += 1;
        
        if self.reg_write[7] != 0 {
            let value = mem.read_mem_value(reg as u16);
            // write value to mem address stored in ppu addr
            // TODO
        }

        for i in 0..=7 {
            self.reg_write[i as usize] = 0;
        }
    }

    pub fn set_write(&mut self, index : usize){
        self.reg_write[index] = 1;
    }

    pub fn updatePpuCtrl(&mut self, byte_val : u8){
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

    pub fn updatePpuMask(&mut self, byte_val : u8){
        self.PPUMASK.greyScale = get_bit(byte_val, 0);
        self.PPUMASK.show_bg_left = get_bit(byte_val, 1);
        self.PPUMASK.show_spr_left = get_bit(byte_val, 2);
        self.PPUMASK.show_bg = get_bit(byte_val, 3);
        self.PPUMASK.show_spr = get_bit(byte_val, 4);
        self.PPUMASK.emphasize_red = get_bit(byte_val, 5);
        self.PPUMASK.emphasize_green=  get_bit(byte_val, 6);
        self.PPUMASK.emphasize_blue = get_bit(byte_val, 7);
    }

    pub fn updatePpuStatus(&mut self, byte_val : u8){
        todo!();
        // read only, but some effects from reading this reg
    }

    pub fn updatePpuScroll(&mut self, byte_val : u8){
        // two writes to this write the address any writes to updatePpuAddr will write to
        if self.PPUSCROLL.write_byte == 0 {
            self.PPUSCROLL.horiz_offset = byte_val;
            self.PPUADDR.write_byte += 1;
        } else {
            self.PPUSCROLL.vert_offset = byte_val;
        }
    }

    pub fn updatePpuAddr(&mut self, byte_val : u8){
        // two writes to this write the address any writes to updatePpuAddr will write to
        // upper byte / big endian first
        if self.PPUADDR.write_byte == 0 {
            self.PPUADDR.address = 0;
            self.PPUADDR.address = (byte_val as u16) << 8;
            self.PPUADDR.write_byte += 1;
        } else {
            self.PPUADDR.address |= (byte_val as u16);
        }
    }

    pub fn updateOAMAddr(&mut self, byte_val : u8){
        todo!();
    }

    pub fn updateOAMData(&mut self, byte_val : u8){
        todo!();
    }
}

#[derive(Default)]
struct ppuCtrl {
    nametableAddress : usize,
    VRAM_address_increment : usize,
    sprite_pattern_table_addr : usize,
    bg_pattern_table_addr : usize,
    sprite_size : usize,
    gen_nmi : usize
}

#[derive(Default)]
struct ppuMask {
    greyScale : u8,
    show_bg_left : u8,
    show_spr_left : u8,
    show_bg : u8,
    show_spr : u8,
    emphasize_red : u8,
    emphasize_green: u8,
    emphasize_blue: u8
}

#[derive(Default)]
struct ppuStatus {
    overflow : u8,
    sprite_hit : u8,
    vert_blank_started : u8
}

#[derive(Default)]
struct ppuScroll{
    horiz_offset : u8,
    vert_offset : u8,
    write_byte : u8
}

#[derive(Default)]
struct ppuAddr {
    address : u16,
    write_byte : u8
}
