use memory::Memory;
use std::fs::File;
use std::io;
use std::io::Read;

enum Cart {
  RomOnly = 0x00,
  RomMBC1,
  RomMBC1Ram,
  RomMBC1RamBatt,
  RomMBC2,
  RomMBC2Batt,
  RomRam = 0x08,
  RomRamBatt,
  RomMMM1 = 0x0B,
  RomMM1Sram,
  RomMM1SramBatt,
  RomMBC3TimerBatt = 0x0F,
  RomMBC3TimerRamBatt,
  RomMBC3,
  RomMBC3Ram,
  RomMBC3RamBatt = 0x13,
  RomMBC5 = 0x19,
  RomMBC5Ram,
  RomMBC5RamBatt,
  RomMBC5Rumble,
  RomMBC5RumbleSram,
  RomMBC5RumbleSramBatt,
  RomMBC6 = 0x20,
  RomMBC7SensorRumbleRamBatt = 0x22
  PocketCamera = 0xFC,
  BandaiTAMA5
  HudsonHuC3
  HudsonHuC1
}

pub fn load_rom(memory: &mut Memory, path: &str) -> Result<(), io::Error> {
  let mut file = File::open(path)?;
  let mut buf = &mut memory.memory[0..0x8000];
  file.read_exact(buf)
}

