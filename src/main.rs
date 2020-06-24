//main.rs for nes-emu
// Copyright 2020 Lloyd Crawley

// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
//associated documentation files (the "Software"), to deal in the Software without restriction,
//including without limitation the rights to use, copy, modify, merge, publish, distribute,
//sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
// INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
// PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE
// FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

extern crate sdl2;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::{self, Window, WindowBuildError, WindowBuilder, WindowContext};
use sdl2::Sdl;
use std::time::{Duration, Instant};

use nes_emu::cpu::nes_6502::Nes6502;
use nes_emu::memory::RAM;
use nes_emu::ppu::ppu::PPU;
use nes_emu::ppu::ppu::output_image;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 240;

const SECONDS_PER_CLOCK: f32 = 1.0 / 1_790_000.0; // 1.79 MHz freq
const NANOS_PER_CLOCK: u128 = (SECONDS_PER_CLOCK * 1_000_000_000_000.0) as u128;

// NES was 60 FPS
const NANOS_PER_FRAME : u128 = 1_000_0000_000 / 60;

fn main() {
    // todo
    // get env args
    // read rom into mem
    // start cpu execution
    let args: Vec<String> = env::args().collect();
    let rom_path = &args[1];

    let rom_data = load_binary(rom_path);
    // parse header info
    let (prg_blocks, chr_blocks, has_trainer) = parse_header(&rom_data);

    let mut ram: RAM = RAM::new(prg_blocks as usize, chr_blocks as usize);
    ram.load_rom(rom_data);

    
    // do SDL init stuff
    let mut sdl_context = sdl2::init().unwrap();
    let win = init_window(&mut sdl_context, WIDTH, HEIGHT);
    let unrapped = win.unwrap();

    let mut canvas = unrapped.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();

    let mut frameBuffers: Vec<Texture> = vec![];

    for i in 0..2 {
        frameBuffers.push(
            texture_creator
                .create_texture_target(PixelFormatEnum::RGB888, WIDTH, HEIGHT)
                .unwrap(),
        );
    }

    // set ppu status
    ram.write_mem_value(0x2002, 0b10100000);

    let mut cpu: Nes6502 = Nes6502::new();
    let mut ppu: PPU = PPU::default();

    // create pixel data
    let mut pixData: Box<[u8]> = vec![0; WIDTH as usize * HEIGHT as usize * 4].into_boxed_slice();
    let mut frame_index = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut timeTaken: u32 = 0;

    let mut frame_time : u128 = 0;

    'running: loop {
        //beginning of loop

        let start = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                _ => {}
            }
        }

        cpu.run(&mut ram);

        // 3 cycles for every cpu one
        for i in 0..3 {
            ppu.run(&mut ram);
        }

       

        frame_index = (frame_index + 1) % 2;
        let cycle_time = start.elapsed().as_nanos();
        timeTaken = cycle_time as u32;

        frame_time += cycle_time;

        if frame_time > NANOS_PER_FRAME {
            frame_time = 0;
            expand_vram(ppu.get_output_image(), &mut pixData);
            let texRef = &mut frameBuffers[frame_index];

            texRef.update(None, &pixData, (WIDTH * 4) as usize);
            canvas.copy(&texRef, None, None);

            canvas.present();
            continue;
        }

        if cycle_time > NANOS_PER_CLOCK {
            continue;
        }

        let sleepAmount = (NANOS_PER_CLOCK - cycle_time) as u32;
        //println!("{}", sleepAmount);
        ::std::thread::sleep(Duration::new(0, sleepAmount)); // 1.79 MHz freq
    }
}

fn load_binary<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    let mut file = fs::File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf);
    file_buf.into_boxed_slice()
}

fn init_window(context: &mut Sdl, width: u32, height: u32) -> Result<Window, WindowBuildError> {
    let video_subsystem = context.video().unwrap();
    let window = video_subsystem
        .window("nes demo", width * 2, height * 2)
        .position_centered()
        .build();

    window
}

fn expand_vram(mem : &[(u8, u8, u8)], pixData : &mut [u8]) {
    
    for i in 0..HEIGHT {
        for j in 0..WIDTH{
            // get the relevant byte 
            let read_offset = i * WIDTH + j;
            let write_offset = read_offset * 4;
            let (r, g, b) =  mem[read_offset as usize];

            pixData[write_offset as usize] = r;
            pixData[(write_offset + 1) as usize] = g;
            pixData[(write_offset + 2) as usize] = b;
            pixData[(write_offset + 3) as usize] = 255;
        }
    }
}

fn parse_header( mem : &Box<[u8]>) -> (u8, u8, bool) {

    let num_prg_blocks = mem[4];
    let num_chr_blocks = mem[5];
    let copy_byte = mem[6];
    let has_trainer : bool = ((copy_byte >> 3) & 0x1) != 0;

    (num_prg_blocks, num_chr_blocks, has_trainer)
}
