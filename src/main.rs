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
extern crate image;

use glium::DisplayBuild;
use glium::Surface;
use std::time::{Duration, Instant};
use conrod::{color, widget};
use conrod::{Colorable, Positionable, Widget, Sizeable};

mod cpu;
mod memory;
mod rom;
mod util;
mod ppu;
mod debug;

widget_ids!(
  struct Ids {
    tabs, tab_game, tab_debugger, game_screen, background
  }
);

fn main() {
  let display = glium::glutin::WindowBuilder::new()
  .with_title(option_env!("CARGO_PKG_NAME").unwrap_or("unknown"))
  .with_dimensions(800, 600)
  .build_glium().unwrap();

  let mut ui = conrod::UiBuilder::new([800.0, 600.0]).build();

  let ids = Ids::new(ui.widget_id_generator());
  ui.fonts.insert_from_file("resource/font/PXSansRegular.ttf").unwrap();

  let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

  let mut image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

  let mut cpu = cpu::CPU::new();
  let mut ppu = ppu::PPU::new();

  let rom_path = std::env::args().nth(1).expect("Gameboy ROM expected as argument");
  let mut memory = rom::load_rom(&rom_path).unwrap();
  // BIOS would set these values
  {
    memory.memory[0xff05] = 0x00; // TIMA
    memory.memory[0xff06] = 0x00; // TMA
    memory.memory[0xff07] = 0x00; // TAC
    memory.memory[0xff10] = 0x80; // NR10
    memory.memory[0xff11] = 0xbf; // NR11
    memory.memory[0xff12] = 0xf3; // NR12
    memory.memory[0xff14] = 0xbf; // NR14
    memory.memory[0xff16] = 0x3f; // NR21
    memory.memory[0xff17] = 0x00; // NR22
    memory.memory[0xff19] = 0xbf; // NR24
    memory.memory[0xff1a] = 0x7f; // NR30
    memory.memory[0xff1b] = 0xff; // NR31
    memory.memory[0xff1c] = 0x9f; // NR32
    memory.memory[0xff1e] = 0xbf; // NR33
    memory.memory[0xff20] = 0xff; // NR41
    memory.memory[0xff21] = 0x00; // NR42
    memory.memory[0xff22] = 0x00; // NR43
    memory.memory[0xff23] = 0xbf; // NR30
    memory.memory[0xff24] = 0x77; // NR50
    memory.memory[0xff25] = 0xf3; // NR51
    memory.memory[0xff26] = 0xf1; // NR52 // 0xf0 on SGB @Future
    memory.memory[0xff40] = 0x91; // LCDC
    memory.memory[0xff42] = 0x00; // SCY
    memory.memory[0xff43] = 0x00; // SCX
    memory.memory[0xff44] = 0x00; // Start at scanline 0
    memory.memory[0xff45] = 0x00; // LYC
    memory.memory[0xff47] = 0xfc; // BGP
    memory.memory[0xff48] = 0xff; // OBP0
    memory.memory[0xff49] = 0xff; // OBP1
    memory.memory[0xff4a] = 0x00; // WY
    memory.memory[0xff4b] = 0x00; // WX
    memory.memory[0xffff] = 0x00; // IE
  }

  let mut last_time = Instant::now();
  let mut cpu_acc = 0;
  let mut ppu_acc = 0;

  let game_screen = {
    let draw_results = ppu.draw(&memory);
    let texture = glium::texture::Texture2d::new(&display, draw_results.0).unwrap();
    image_map.insert(texture)
  };
  'game: loop {
    let mut elapsed = Instant::now().duration_since(last_time);
    if elapsed > Duration::from_millis(100) {
      elapsed = Duration::from_millis(100);
    };
    cpu_acc += elapsed.subsec_nanos() as i64;
    ppu_acc += elapsed.subsec_nanos() as i64;
    last_time = Instant::now();

    for event in display.poll_events() {
      // Use the `winit` backend feature to convert the winit event to a conrod one.
      if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
        ui.handle_event(event);
      }

      match event {
        glutin::Event::Closed => break 'game,
        _ => (),
      }
    }


    // TODO:
    // I think it would be cool to emulate the next step and see how long that took
    // then only follow through on it if we banked enough time
    // right now we are going too fast
    let mut did_something = true;
    while did_something {
      did_something = false;
      if cpu_acc > 952 {
        cpu_acc -= cpu.step(&mut memory) * 238;
        did_something = true;
      }
      if ppu_acc > ppu.estimate_clock_cycles() * 238 {
        ppu_acc -= ppu.step(&mut memory) * 238;
        did_something = true;
      }
    }

    {
      let draw_results = ppu.draw(&memory);
      let texture = glium::texture::Texture2d::new(&display, draw_results.0).unwrap();
      let _ = image_map.replace(game_screen, texture);
      ui.needs_redraw();

      // Instantiate all widgets in the GUI.
      {
        let ui = &mut ui.set_widgets();

        widget::Tabs::new(&[(ids.tab_game, "Gameboy"), (ids.tab_debugger, "Debugger")])
        .middle_of(ui.window)
        .color(color::BLUE)
        .label_color(color::WHITE)
        .set(ids.tabs, ui);

        widget::Image::new(game_screen).w_h(256.0f64, 256.0f64).middle_of(ids.tab_game).set(ids.game_screen, ui);
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
}
