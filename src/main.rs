extern crate glutin;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate log;
extern crate log_panics;

use glium::DisplayBuild;
use glium::Surface;
use std::time::{Duration, Instant};

mod cpu;
mod memory;
mod rom;
mod util;

fn main() {
    let display = glium::glutin::WindowBuilder::new()
    .with_title(option_env!("CARGO_PKG_NAME").unwrap_or("unknown"))
    .with_dimensions(160, 144)
    .build_glium().unwrap();


    let mut memory = memory::Memory::new();
    let mut cpu = cpu::CPU::new();
    let rom_path = std::env::args().nth(1).unwrap();

    rom::load_rom(&mut memory, &rom_path).unwrap();

    let mut last_time = Instant::now();
    let mut acc = 0;
    'game: loop {
        let mut elapsed = Instant::now().duration_since(last_time);
        if elapsed > Duration::from_millis(100) {
            elapsed = Duration::from_millis(100);
        };
        acc += (elapsed.as_secs() as i64 * 1000000000) + elapsed.subsec_nanos() as i64;
        last_time = Instant::now();

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => break 'game,
                glutin::Event::Resized(width, height) => {
                    // Doo dad
                }
                _ => (),
            }
        }

        while acc > 238 {
            acc -= cpu.step(&mut memory) as i64 * 238;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.finish().unwrap();
    }
}