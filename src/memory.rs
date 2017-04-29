use std;

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
      memory: Box::new(unsafe { std::mem::uninitialized() })
    }
  }

  // @Performance Read and write can use unsafe operations to index

  pub fn write(&mut self, address: u16, value: u8) {
    self.memory[translate(address)] = value;
  }

  pub fn read(&self, address: u16) -> u8 {
    self.memory[translate(address)]
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