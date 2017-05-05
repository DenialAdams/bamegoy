use memory::Memory;
use util::LoHi;

bitflags! {
    struct Flags: u8 {
        const ZERO =       0b10000000;
        const SUBTRACT =   0b01000000;
        const HALF_CARRY = 0b00100000;
        const CARRY =      0b00010000;
    }
}

pub struct CPU {
  a: u8,
  f: Flags,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  h: u8,
  l: u8,
  stack_pointer: u16,
  program_counter: u16,
  interrupts: bool
}

impl CPU {
  pub fn new() -> CPU {
    CPU {
      a: 0x01,
      f: Flags::from_bits_truncate(0xb0),
      b: 0x00,
      c: 0x13,
      d: 0x00,
      e: 0xd8,
      h: 0x01,
      l: 0x4d,
      stack_pointer: 0xfffe,
      program_counter: 0x100,
      interrupts: true
    }
  }

  pub fn step(&mut self, memory: &mut Memory) -> u8 {
    // Fetch
    let opcode: u8 = memory.read_byte(self.program_counter);
    println!("{:02x} at address {:04x}", opcode, self.program_counter);
    // Increment
    self.program_counter += 1;
    // Execute
    match opcode {
      0x00 => {
        // NOP
        4
      },
      0x02 => {
        // LD (BC),A
        let value = memory.read_byte((self.b as u16) << 8 | self.c as u16);
        self.a = value;
        8
      },
      0x05 => {
        // DEC b
        self.b = self.b.wrapping_sub(1);
        self.f.set(ZERO, self.b == 0);
        self.f.insert(SUBTRACT);
        self.f.set(HALF_CARRY, (self.b ^ 255 ^ self.b.wrapping_sub(1)) & 0x10 == 0x10);
        4
      },
      0x06 => {
        // LD n into B
        let value = self.read_byte_parameter(memory);
        self.b = value;
        8
      },
      0x0d => {
        // DEC c
        self.c = self.c.wrapping_sub(1);
        self.f.set(ZERO, self.c == 0);
        self.f.insert(SUBTRACT);
        self.f.set(HALF_CARRY, (self.c ^ 255 ^ self.c.wrapping_sub(1)) & 0x10 == 0x10);
        4
      }
      0x0e => {
        // LD n into C
        let value = self.read_byte_parameter(memory);
        self.c = value;
        8
      },
      0x18 => {
        // JR
        let rel_target = self.read_signed_byte_parameter(memory);
        self.relative_jump(rel_target);
        12
      }
      0x20 => {
        // JR NZ
        let rel_target = self.read_signed_byte_parameter(memory);
        if !self.f.contains(ZERO) {
          self.relative_jump(rel_target);
          12
        } else {
          8
        }
      },
      0x21 => {
        // LD nn into HL
        let value = self.read_short_parameter(memory);
        self.h = value.hi();
        self.l = value.lo();
        12
      },
      0x31 => {
        // LD short as sp
        let value = self.read_short_parameter(memory);
        self.stack_pointer = value;
        12
      },
      0x32 => {
        // LDD A into HL
        let result = (self.a as u16).wrapping_sub(1);
        self.h = result.hi();
        self.l = result.lo();
        8
      },
      0x3e => {
        // LD # into A
        let result = self.read_byte_parameter(memory);
        self.a = result;
        8
      },
      0x4d => {
        // LD L into C
        self.c = self.l;
        4
      },
      0x7c =>{
        // LD A, H
        self.a = self.h;
        4
      }
      0x7d => {
        // LD A,L
        self.a = self.l;
        4
      },
      0xa3 => {
        // AND E
        self.a &= self.e;
        self.f.set(ZERO, self.a == 0);
        self.f.remove(SUBTRACT);
        self.f.insert(HALF_CARRY);
        self.f.remove(CARRY);
        4
      },
      0xaf => {
        // XOR A with A
        self.a ^= self.a;
        self.f.set(ZERO, self.a == 0);
        self.f.remove(SUBTRACT);
        self.f.remove(HALF_CARRY);
        self.f.remove(CARRY);
        4
      },
      0xc3 => {
        // JMP nn
        let target = memory.read_short(self.program_counter);
        println!("jumping to {:04x}", target);
        self.program_counter = target;
        16
      },
      0xcd => {
        let pc = self.program_counter;
        self.write_short_to_stack(memory, pc);
        let target = memory.read_short(self.program_counter);
        println!("jumping to {:04x}", target);
        self.program_counter = target;
        24
      },
      0xc9 => {
        // RET
        let dest = self.pop_short(memory);
        self.program_counter = dest;
        16
      },
      0xea => {
        // LD nn,A
        let dest = self.read_short_parameter(memory);
        memory.write_byte(dest, self.a);
        16
      },
      0xe0 => {
        // LDH n,A
        let offset = self.read_byte_parameter(memory);
        memory.write_byte(0xFF00 + offset as u16, self.a);
        12
      },
      0xf0 => {
        // LDH A,n
        let offset = self.read_byte_parameter(memory);
        self.a = memory.read_byte(0xFF00 + offset as u16);
        12
      },
      0xf1 => {
        // Pop into AF
        let short = self.pop_short(memory);
        self.a = short.hi();
        self.f = Flags::from_bits_truncate(short.lo());
        12
      },
      0xf3 => {
        // Disable interrupts
        self.interrupts = false;
        4
      },
      0xfe => {
        // Compare A with #
        let value = self.read_byte_parameter(memory);
        self.f.set(ZERO, self.a == value);
        self.f.insert(SUBTRACT);
        self.f.set(HALF_CARRY, (self.a ^ value ^ self.a.wrapping_sub(value)) & 0x10 == 0x10);
        self.f.set(CARRY, self.a > value);
        8
      },
      0xff => {
        // RST 38
        let pc = self.program_counter;
        self.write_short_to_stack(memory, pc);
        self.program_counter = 0x0038;
        16
      },
      _ => {
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

  fn write_short_to_stack(&mut self, memory: &mut Memory, value: u16) {
    self.decrement_sp();
    memory.write_byte(self.stack_pointer, value.hi());
    self.decrement_sp();
    memory.write_byte(self.stack_pointer, value.lo());
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