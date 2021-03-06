use std;
use util::LoHi;

/* 
Helpful reference!
struct Memory {
  // 0x00 - 0x3fff
  bank_one: [u8; 16384],
  // 0x4000 - 0x7FFF
  bank_two: [u8; 16384],
  // 0x8000 - 0x9FFF,
  graphics: [u8; 8192],
  // 0xA000 - 0xBFFF
  external: [u8; 8192],
  // 0xC000 - 0xDFFF
  working: [u8; 8192],
  // 0xE000 - 0xFDFF
  // This mirrors working (except for the last 512 bytes)
  working_copy: [u8; 7680],
  // 0xFE00 - 0xFE9F
  sprites: [u8; 160],
  // 0xFF00 - 0xFF7F
  mmap: [u8; 128],
  // 0xFF80 - 0xFFFF
  zero_page: [u8; 128]
}
*/

pub struct Memory {
  pub memory: Box<[u8; 65536]>
}

impl Memory {
  pub fn new() -> Memory {
    Memory {
      memory: Box::new(unsafe { std::mem::zeroed() })
    }
  }

  // @Performance Read and write can use unsafe operations to index

  pub fn write_byte(&mut self, address: u16, value: u8) {
    self.memory[translate(address)] = value;
  }

  pub fn write_short(&mut self, address: u16, value: u16) {
    // This is basically un-needed because rust does this in debug mode already
    // but I just want to remind myself
    debug_assert!(address != 65535);
    self.memory[translate(address)] = value.lo();
    self.memory[translate(address + 1)] = value.hi();
  }

  pub fn read_byte(&self, address: u16) -> u8 {
    if address >= 0xFEA0 && address <= 0xFEFF {
      0xff
    } else if address == 0xFF0F {
      0b11100000 | self.memory[0xff0f]
    } else {
      self.memory[translate(address)]
    }
  }
  
  pub fn read_signed_byte(&self, address: u16) -> i8 {
    self.read_byte(address) as i8
  }

  pub fn read_short(&self, address: u16) -> u16 {
    // This is basically un-needed because rust does this in debug mode already
    // but I just want to remind myself
    debug_assert!(address != 65535);
    (self.read_byte(address + 1) as u16) << 8 | self.read_byte(address) as u16
  }
}

// Translates from virtual gameboy addresses to our array indexing
fn translate(address: u16) -> usize {
  // If it's in the working memory "shadow" just index the working memory
  if address >= 0xE000 && address <= 0xFDFF {
    address as usize - 0x2000
  } else {
    address as usize
  }
}