use memory::Memory;
use std::fs::File;
use std::io;
use std::io::Read;
use enum_primitive::FromPrimitive;

const SUPPORTED_CART_TYPES: [Cart; 1] = [Cart::RomOnly];

enum_from_primitive! {
  #[derive(PartialEq)]
  enum Cart {
    RomOnly = 0x00,
    MBC1,
    BC1Ram,
    BC1RamBatt,
    MBC2,
    MBC2Batt,
    Ram = 0x08,
    RamBatt,
    MMM01 = 0x0B,
    MMM01Ram,
    MMM01RamBatt,
    MBC3TimerBatt = 0x0F,
    MBC3TimerRamBatt,
    MBC3,
    MBC3Ram,
    MBC3RamBatt = 0x13,
    MBC5 = 0x19,
    MBC5Ram,
    MBC5RamBatt,
    MBC5Rumble,
    MBC5RumbleRam,
    BC5RumbleRamBatt,
    MBC6 = 0x20,
    MBC7SensorRumbleRamBatt = 0x22,
    PocketCamera = 0xFC,
    BandaiTAMA5,
    HudsonHuC3,
    HudsonHuC1RamBatt
  }
}

#[derive(Debug)]
pub enum RomError {
  NotARom,
  Io(io::Error)
}

impl From<io::Error> for RomError {
  fn from(err: io::Error) -> RomError {
    RomError::Io(err)
  }
}

pub fn load_rom(memory: &mut Memory, path: &str) -> Result<(), RomError> {
  do_load(memory, path)?;
  let cart_val = memory.read_byte(0x1047);
  let cart_type = Cart::from_u8(cart_val);
  if let Some(cart) = cart_type {
    if !is_supported(cart) {
      println!("ROM {:02x} not supported, but carrying on", cart_val);
    }
    Ok(())
  } else {
    println!("ROM given has invalid type {:02x}!", cart_val);
    Err(RomError::NotARom)
  }
}

fn do_load(memory: &mut Memory, path: &str) -> Result<(), io::Error> {
  let mut file = File::open(path)?;
  let buf = &mut memory.memory[0..0x8000];
  file.read_exact(buf)
}

fn is_supported(cart: Cart) -> bool {
  for x in SUPPORTED_CART_TYPES.iter() {
    if cart == *x {
      return true;
    }
  }
  false
}
