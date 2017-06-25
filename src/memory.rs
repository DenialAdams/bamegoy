use std;
use util::LoHi;
use rom::{Cart, MBCMode};

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
  // This is wasteful... ROM Bank memory is duplicated. Should probably split this up TODO
  pub memory: Box<[u8; 65536]>,
  pub cart: Cart
}

impl Memory {
  pub fn write_byte(&mut self, address: u16, value: u8) {
    if address <= 0x7fff {
      match self.cart {
        Cart::RomOnly(_) => {
          // Nothing
        },
        Cart::MBC1(ref mut data) => {
          if address <= 0x1fff {
            data.ram_enabled = value & 0x0f == 0x0a;
          } else if address <= 0x3fff {
            data.bank_lo = if value == 0x00 {
              1
            } else {
              value & 0x1f
            }
          } else if address <= 0x5fff {
            data.bank_hi = value & 0b0110_0000;
          } else {
            if value & 0x01 == 0x01 {
              data.mode = MBCMode::Ram;
            } else {
              data.mode = MBCMode::Rom;
            }
          }
        }
      }
    } else if address >= 0xa000 && address <= 0xbfff {
      match self.cart {
        Cart::RomOnly(ref data) => {
          unimplemented!()
        },
        Cart::MBC1(ref mut data) => {
          match data.mode {
            MBCMode::Rom => {
              data.ram[address as usize - 0xa000] = value;
            },
            MBCMode::Ram => {
              unimplemented!()
            }
          }
        }
      }
    } else if address >= 0xe000 && address <= 0xfdff {
      self.memory[address as usize - 0x2000] = value;
    } else if address == 0xff41 {
      // bits 0-2 are read only
      self.memory[0xff41] = (value & 0x1111_1000) | (self.memory[0xff41] & 0x0000_0111);
    } else {
      self.memory[address as usize] = value;
    }
  }

  pub fn write_short(&mut self, address: u16, value: u16) {
    // TODO is this right behavior? (wrap)
    self.write_byte(address, value.lo());
    self.write_byte(address.wrapping_add(1), value.hi());
  }

  pub fn read_byte(&self, address: u16) -> u8 {
    if address >= 0x4000 && address <= 0x7fff {
      match self.cart {
        Cart::RomOnly(ref data) => {
          data[address as usize - 0x4000]
        },
        Cart::MBC1(ref data) => {
          match data.mode {
            MBCMode::Rom => {
              data.rom[(data.bank_hi | data.bank_lo) as usize * 16384 + (address as usize - 0x4000)]
            },
            MBCMode::Ram => {
              data.rom[data.bank_lo as usize * 16384 + (address as usize - 0x4000)]
            }
          }
        }
      }
    } else if address >= 0xa000 && address <= 0xbfff {
      match self.cart {
        Cart::RomOnly(ref data) => {
          unimplemented!()
        },
        Cart::MBC1(ref data) => {
          match data.mode {
            MBCMode::Rom => {
              data.ram[address as usize - 0xa000]
            },
            MBCMode::Ram => {
              unimplemented!()
            }
          }
        }
      }
    } else if address >= 0xe000 && address <= 0xfdff {
      self.memory[address as usize - 0x2000]
    } else if address >= 0xfea0 && address <= 0xfeff {
      0xff
    } else if address == 0xff0f {
      0b1110_0000 | self.memory[0xff0f]
    } else {
      self.memory[address as usize]
    }
  }
  
  pub fn read_signed_byte(&self, address: u16) -> i8 {
    self.read_byte(address) as i8
  }

  pub fn read_short(&self, address: u16) -> u16 {
    // TODO is this right behavior? (wrap)
    (self.read_byte(address.wrapping_add(1)) as u16) << 8 | self.read_byte(address) as u16
  }
}

