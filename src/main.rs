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
use sdl2::Sdl;
use sdl2::video::{self, Window, WindowBuilder, WindowContext, WindowBuildError};
use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioDevice};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::pixels::{Color, PixelFormatEnum};
use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use nes_emu::cpu::nes_6502::Nes6502;
use nes_emu::ppu::ppu::PPU;
use nes_emu::memory::RAM;

const WIDTH : u32 = 64;
const HEIGHT: u32 = 32;

const SECONDS_PER_CLOCK : f32 = 1.0 / 1_790_000.0; // 1.79 MHz freq
const NANOS_PER_CLOCK: u128 = (SECONDS_PER_CLOCK * 1_000_000_000_000.0) as u128;

fn main() {
    // todo 
    // get env args
    // read rom into mem
    // start cpu execution
    let args : Vec<String> = env::args().collect();
    let rom_path = &args[1];

    let rom_data = load_binary(rom_path);
    let mut ram : RAM = RAM::new();
    ram.load_rom(rom_data);

     // do SDL init stuff
     let mut sdl_context = sdl2::init().unwrap();
     let win = init_window(&mut sdl_context, WIDTH, HEIGHT);
     let unrapped = win.unwrap();
 
     let mut canvas = unrapped.into_canvas().build().unwrap();
 
     let texture_creator = canvas.texture_creator();
 
     let mut frameBuffers : Vec<Texture> = vec![];

    for i in 0..2{
        frameBuffers.push(texture_creator
            .create_texture_target(PixelFormatEnum::RGB888, WIDTH, HEIGHT)
            .unwrap());
    }
    
    let mut cpu : Nes6502 = Nes6502::new();
    let mut ppu : PPU = PPU::default();


    // create pixel data
    let mut pixData : Box<[u8]> = vec![0; WIDTH as usize * HEIGHT as usize * 4 ].into_boxed_slice();
    let mut frame_index = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut timeTaken : u32 = 0;
    'running: loop {
        //beginning of loop
      
        let start = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                _ => {}
            }
        }

        cpu.run(&mut ram);
        ppu.run(&mut ram);

        expand_vram();

        frame_index = ( frame_index + 1 ) % 2;
        let frameTime = start.elapsed().as_nanos();
        timeTaken = frameTime as u32;
        if(frameTime > NANOS_PER_CLOCK){
            continue;
        }
       
        let sleepAmount = (NANOS_PER_CLOCK - frameTime) as u32;
        //println!("{}", sleepAmount);
        ::std::thread::sleep(Duration::new(0, sleepAmount)); // 1.79 MHz freq
    };
}

fn load_binary<P: AsRef<Path>>(path : P) -> Box<[u8]> {
    let mut file = fs::File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf);
    file_buf.into_boxed_slice()
}

fn init_window(context : &mut Sdl, width : u32, height : u32) -> Result<Window, WindowBuildError> {
    let video_subsystem = context.video().unwrap();
    let window = video_subsystem.window("nes demo", width * 8, height * 8)
        .position_centered()
        .build();

    window
}

fn expand_vram(){
    
}