#[macro_use]
extern crate conrod;
extern crate glutin;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate log;
extern crate log_panics;
#[macro_use]
extern crate bitflags;

use glium::DisplayBuild;
use glium::Surface;
use std::time::{Duration, Instant};
use conrod::{color, widget};
use conrod::{Colorable, Positionable, Widget};

mod cpu;
mod memory;
mod rom;
mod util;

widget_ids!(
    struct Ids {
        tabs, tab_game, tab_debugger
    }
);

fn main() {
    let display = glium::glutin::WindowBuilder::new()
    .with_title(option_env!("CARGO_PKG_NAME").unwrap_or("unknown"))
    .with_dimensions(800, 600)
    .build_glium().unwrap();

    let mut ui = conrod::UiBuilder::new([800.0, 600.0]).build();

    let ids = Ids::new(ui.widget_id_generator());
    ui.fonts.insert_from_file("resource/PXSansRegular.ttf").unwrap();

    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

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
            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
                ui.handle_event(event);
            }

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

        // Instantiate all widgets in the GUI.
        {
            let ui = &mut ui.set_widgets();

            widget::Tabs::new(&[(ids.tab_game, "Gameboy"), (ids.tab_debugger, "Debugger")])
            .top_left_of(ui.window)
            .color(color::BLUE)
            .label_color(color::WHITE)
            .set(ids.tabs, ui);
        }

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}