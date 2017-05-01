use memory::Memory;
use util::LoHi;

// TODO: use bitflags?
struct Flag {
  zero: bool,
  subtract: bool,
  half_carry: bool,
  carry: bool
}

pub struct CPU {
  a: u8,
  f: u8,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  h: u8,
  l: u8,
  stack_pointer: u16,
  program_counter: u16,
  flags: Flag,
  cycles: u64
}

impl CPU {
  pub fn new() -> CPU {
    CPU {
      a: 0x01,
      f: 0xb0,
      b: 0x00,
      c: 0x13,
      d: 0x00,
      e: 0xd8,
      h: 0x01,
      l: 0x4d,
      stack_pointer: 0xfffe,
      program_counter: 0x100,
      flags: Flag {
        zero: false,
        subtract: false,
        half_carry: false,
        carry: false
      },
      cycles: 0
    }
  }

  pub fn step(&mut self, memory: &mut Memory) {
    // Fetch
    let opcode: u8 = memory.read_byte(self.program_counter);
    //println!("{:02x} at address {:04x}", opcode, self.program_counter);
    // Increment
    self.program_counter += 1;
    // Execute
    match opcode {
      0x00 => {
        // NOP
        self.cycles += 4;
      },
      0x05 => {
        // DEC b
        self.b = self.b.wrapping_sub(1);
        self.flags.zero = self.b == 0;
        self.flags.subtract = true;
        self.flags.half_carry = self.b & 0x10 == 0x10; // @Correctness?
        self.cycles + 4;
      },
      0x06 => {
        // LD n into B
        let value = self.read_byte_parameter(memory);
        self.b = value;
        self.cycles += 8;
      },
      0x0d => {
        // DEC c
        self.c = self.c.wrapping_sub(1);
        self.flags.zero = self.c == 0;
        self.flags.subtract = true;
        self.flags.half_carry = self.c & 0x10 == 0x10; // @Correctness?
        self.cycles += 4;
      }
      0x0e => {
        // LD n into C
        let value = self.read_byte_parameter(memory);
        self.c = value;
        self.cycles += 8;
      },
      0x20 => {
        let rel_target = self.read_signed_byte_parameter(memory);
        if !self.flags.zero {
          self.relative_jump(rel_target);
        }
        self.cycles += 8;
      },
      0x21 => {
        // LD nn into HL
        let value = self.read_short_parameter(memory);
        self.h = value.hi();
        self.l = value.lo();
        self.cycles += 12;
      },
      0x32 => {
        // LDD A into HL
        let result = (self.a as u16).wrapping_sub(1);
        self.h = result.hi();
        self.l = result.lo();
        self.cycles += 8;
      },
      0x3e => {
        // LD # into A
        let result = self.read_byte_parameter(memory);
        self.a = result;
        self.cycles += 8;
      },
      0x4d => {
        // LD L into C
        self.c = self.l;
        self.cycles += 4;
      },
      0xaf => {
        // XOR A with A
        self.a ^= self.a;
        self.flags.zero = self.a == 0;
        self.flags.subtract = false;
        self.flags.half_carry = false;
        self.flags.carry = false;
        self.cycles += 4;
      },
      0xc3 => {
        // JMP nn
        let target = memory.read_short(self.program_counter);
        println!("jumping to {:04x}", target);
        self.program_counter = target;
        self.cycles += 12; // @Correctness; conflicting information on this
      },
      0xe0 => {
        // LDH n,A
        let offset = self.read_byte_parameter(memory);
        memory.write_byte(0xFF00 + offset as u16, self.a);
        self.cycles += 12;
      },
      0xf0 => {
        // LDH A,n
        let offset = self.read_byte_parameter(memory);
        self.a = memory.read_byte(0xFF00 + offset as u16);
        self.cycles += 12;
      },
      0xf1 => {
        // Pop into AF
        let short = self.pop_short(memory);
        self.a = short.hi();
        self.f = short.lo();
        self.cycles += 12
      },
      0xf3 => {
        // Disable interrupts
        // TODO
        self.cycles += 4;
      },
      0xfe => {
        // Compare A with #
        let value = self.read_byte_parameter(memory);
        self.flags.zero = self.a == value;
        self.flags.subtract = true;
        self.flags.half_carry = (self.a.wrapping_sub(value)) & 0x10 == 0x10; // @Correctness?
        self.flags.carry = self.a > value;
        self.cycles += 8
      },
      0xff => {
        // RST 38
        self.write_pc_to_stack(memory);
        self.program_counter = 0x0038;
        self.cycles += 32;
      },
      _ => {
        println!("{:02x} at address {:04x}", opcode, self.program_counter);
        unimplemented!()
      }
    }
  }

  fn relative_jump(&mut self, rel_target: i8) {
    if rel_target < 0 {
      self.program_counter -= -rel_target as u16;
    } else {
      self.program_counter += rel_target as u16;
    }
  }

  fn write_pc_to_stack(&mut self, memory: &mut Memory) {
    self.decrement_sp();
    memory.write_byte(self.stack_pointer, self.program_counter.hi());
    self.decrement_sp();
    memory.write_byte(self.stack_pointer, self.program_counter.lo());
  }

  fn pop_short(&mut self, memory: &Memory) -> u16 {
    let mut x: u16 = memory.read_byte(self.stack_pointer) as u16;
    self.increment_sp();
    x = (memory.read_byte(self.stack_pointer) as u16) << 8 | x;
    self.increment_sp();
    x
  }

  fn read_short_parameter(&mut self, memory: &Memory) -> u16 {
    let value = memory.read_short(self.program_counter);
    self.program_counter += 2;
    value
  }

  fn read_byte_parameter(&mut self, memory: &Memory) -> u8 {
    let value = memory.read_byte(self.program_counter);
    self.program_counter += 1;
    value
  }

  fn read_signed_byte_parameter(&mut self, memory: &Memory) -> i8 {
    let value = memory.read_signed_byte(self.program_counter);
    self.program_counter += 1;
    value
  }

  fn decrement_sp(&mut self) {
    debug_assert!(self.stack_pointer != 0xFF80);
    self.stack_pointer -= 1;
  }

  fn increment_sp(&mut self) {
    debug_assert!(self.stack_pointer != 0xFFFE);
    self.stack_pointer += 1;
  }
}