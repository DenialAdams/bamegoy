use memory::Memory;
use std::fs::File;
use std::io;
use std::io::Read;

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

pub fn load_rom(memory: &mut Memory, path: &str) -> Result<(), io::Error> {
  let mut file = File::open(path)?;
  let result = {
    let buf = &mut memory.memory[0..0x8000];
    file.read_exact(buf)
  };
  if memory.read_byte(0x0147) != 0x00 {
    println!("ROM {:02x} not supported, but carrying on", memory.read_byte(0x0147));
  }
  result
}

