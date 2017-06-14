use memory::Memory;
use std::fs::File;
use std;
use std::io;
use std::io::Read;

/*
const SUPPORTED_CART_TYPES: [Cart; 1] = [Cart::RomOnly];

#[derive(PartialEq)]
pub enum Cart {
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
} */

pub enum Cart {
  RomOnly,
  MBC1([u8; 2097152])
}

#[derive(Debug)]
pub enum RomError {
  Invalid,
  Io(io::Error)
}

impl From<io::Error> for RomError {
  fn from(err: io::Error) -> RomError {
    RomError::Io(err)
  }
}

pub fn load_rom(memory: &mut Memory, path: &str) -> Result<(), RomError> {
  let mut file = File::open(path)?;
  do_load(memory, &mut file)?;
  let cart_val = memory.read_byte(0x0147);
  match cart_val {
    0x00 => {
      // RomOnly
      // Nothing needed
      Ok(())
    },
    0x01 => {
      // MBC1
      let mut buf: [u8; 2097152] = unsafe { std::mem::zeroed() };
      file.read(&mut buf);
      memory.cart = Cart::MBC1(buf);
      Ok(())
    },
    _ => {
      println!("ROM given has invalid / unsupported type {:02x}!", cart_val);
      Err(RomError::Invalid)
    }
  }
}

fn do_load(memory: &mut Memory, file: &mut File) -> Result<(), io::Error> {
  let buf = &mut memory.memory[0..0x8000];
  file.read_exact(buf)
}
