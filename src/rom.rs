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

pub struct MBC1Data {
  pub bank_lo: u8,
  pub bank_hi: u8,
  pub rom: Box<[u8; 2097152]>,
  // This is only one bank for now TODO
  pub ram: Box<[u8; 32768]>,
  pub mode: MBCMode,
  pub ram_enabled: bool
}

pub enum MBCMode {
  Rom,
  Ram
}

pub enum Cart {
  RomOnly(Box<[u8; 16384]>),
  MBC1(MBC1Data)
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

pub fn load_rom(path: &str) -> Result<Memory, RomError> {
  let mut file = File::open(path)?;
  let mut memory = Box::new([0; 65536]);
  {
    let buf = &mut memory[0..0x4000];
    file.read_exact(buf)?
  }
  let cart_val = memory[0x0147];
  match cart_val {
    0x00 => {
      // RomOnly
      let mut buf = Box::new([0; 16384]);
      file.read(&mut *buf);
      let cart = Cart::RomOnly(buf);
      Ok(Memory {
        memory: memory,
        cart: cart
      })
    },
    0x01 => {
      // MBC1
      let mut buf = Box::new([0; 2097152]);
      file.read(&mut *buf);
      let cart = Cart::MBC1(MBC1Data {
        bank_lo: 0x01,
        bank_hi: 0x00,
        rom: buf,
        ram: Box::new([0; 32768]),
        mode: MBCMode::Rom,
        ram_enabled: false
      });
      Ok(Memory {
        memory: memory,
        cart: cart
      })
    },
    _ => {
      println!("ROM given has invalid / unsupported type {:02x}!", cart_val);
      Err(RomError::Invalid)
    }
  }
}

