extern crate glutin;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate log;
extern crate log_panics;

use glium::DisplayBuild;
use glium::Surface;

mod cpu;

fn main() {
    let display = glium::glutin::WindowBuilder::new()
    .with_title(option_env!("CARGO_PKG_NAME").unwrap_or("unknown"))
    .with_dimensions(160, 144)
    .build_glium().unwrap();

    'game: loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => break 'game,
                glutin::Event::Resized(width, height) => {
                    // Doo dad
                }
                _ => (),
            }
        }
    }
}