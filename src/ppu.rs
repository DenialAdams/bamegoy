use memory::Memory;
use image::{ImageBuffer, Rgba};
use glium;

bitflags! {
    struct LCDC: u8 {
        const LCD_POWER         = 0b10000000;
        const WINDOW_TILE_MAP   = 0b01000000;
        const WINDOW_ENABLE     = 0b00100000;
        const BG_WINDOW_TILESET = 0b00010000;
        const BG_TILE_MAP       = 0b00001000;
        const SPRITE_SIZE       = 0b00000100;
        const SPRITES_ENABLED   = 0b00000010;
        const BG_ENABLED        = 0b00000001;
    }
}

pub fn draw(memory: &Memory) -> (glium::texture::RawImage2d<u8>, u8) {
  let mut img = ImageBuffer::new(256, 256);
  let control = LCDC::from_bits_truncate(memory.read_byte(0xff40));
  let tiles = if control.contains(BG_WINDOW_TILESET) {
    &memory.memory[0x8000...0x8fff]
  } else {
    &memory.memory[0x8800...0x97ff]
  };
  let bg_tile_map = if control.contains(BG_TILE_MAP) {
    &memory.memory[0x9c00...0x9fff]
  } else {
    &memory.memory[0x9800...0x9bff]
  };
  let mut cur_x = 0;
  let mut cur_y = 0;
  for row in bg_tile_map.chunks(32) {
    for index in row {
      let real_index = if control.contains(BG_WINDOW_TILESET) {
        *index as usize * 16
      } else {
        ((*index) as i8 as i16 + 128) as usize * 16
      };
      let tile = &tiles[real_index..real_index+16];
      for line in tile.chunks(2) {
        img.put_pixel(cur_x,     cur_y, to_pixel(line[0] & 0x80 >> 6 | line[1] & 0x80 >> 7));
        img.put_pixel(cur_x + 1, cur_y, to_pixel(line[0] & 0x40 >> 6 | line[1] & 0x40 >> 7));
        img.put_pixel(cur_x + 2, cur_y, to_pixel(line[0] & 0x20 >> 6 | line[1] & 0x20 >> 7));
        img.put_pixel(cur_x + 3, cur_y, to_pixel(line[0] & 0x10 >> 6 | line[1] & 0x10 >> 7));
        img.put_pixel(cur_x + 4, cur_y, to_pixel(line[0] & 0x08 >> 6 | line[1] & 0x08 >> 7));
        img.put_pixel(cur_x + 5, cur_y, to_pixel(line[0] & 0x04 >> 6 | line[1] & 0x04 >> 7));
        img.put_pixel(cur_x + 6, cur_y, to_pixel(line[0] & 0x02 >> 6 | line[1] & 0x02 >> 7));
        img.put_pixel(cur_x + 7, cur_y, to_pixel(line[0] & 0x01 >> 6 | line[1] & 0x01 >> 7));
        //println!("{} {}", cur_x, cur_y);
        cur_y += 1;
      }
      cur_x += 8;
      cur_y -= 8;
    }
    cur_y += 8;
    cur_x = 0;
  }
  let scroll_x = memory.read_byte(0xff42);
  let scroll_y = memory.read_byte(0xff43);
  (glium::texture::RawImage2d::from_raw_rgba_reversed(img.into_raw(), (256, 256)), 0)
}

fn to_pixel(bits: u8) -> Rgba<u8> {
  // TODO: do palette lookup
  match bits {
    0 => {
      Rgba([0u8, 0, 0, 255])
    },
    1 => {
      Rgba([85u8, 85, 85, 255])
    },
    2 => {
      Rgba([170u8, 170, 170, 255])
    },
    3 => {
      Rgba([255u8, 255, 255, 255])
    },
    _ => unreachable!()
  }
}