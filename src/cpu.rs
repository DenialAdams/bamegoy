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
      0x01 => {
        // LD BC, d16
        self.c = self.read_byte_immediate(memory);
        self.b = self.read_byte_immediate(memory);
        12
      },
      0x02 => {
        // LD (BC),A
        let value = memory.read_byte((self.b as u16) << 8 | self.c as u16);
        self.a = value;
        8
      },
      0x03 => {
        // INC bc
        let val = self.bc().wrapping_add(1);
        self.b = val.hi();
        self.c = val.lo();
        8
      },
      0x05 => {
        // DEC b\
        let orig = self.b;
        self.b = self.b.wrapping_sub(1);
        self.f.set(ZERO, self.b == 0);
        self.f.insert(SUBTRACT);
        self.f.set(HALF_CARRY, (orig ^ 255 ^ self.b) & 0x10 == 0x10);
        4
      },
      0x06 => {
        // LD n into B
        let value = self.read_byte_immediate(memory);
        self.b = value;
        8
      },
      0x0d => {
        // DEC c
        let orig = self.c;
        self.c = self.c.wrapping_sub(1);
        self.f.set(ZERO, self.c == 0);
        self.f.insert(SUBTRACT);
        self.f.set(HALF_CARRY, (orig ^ 255 ^ self.c) & 0x10 == 0x10);
        4
      }
      0x0e => {
        // LD n into C
        let value = self.read_byte_immediate(memory);
        self.c = value;
        8
      },
      0x18 => {
        // JR
        let rel_target = self.read_signed_byte_immediate(memory);
        self.relative_jump(rel_target);
        12
      }
      0x20 => {
        // JR NZ
        let rel_target = self.read_signed_byte_immediate(memory);
        if !self.f.contains(ZERO) {
          self.relative_jump(rel_target);
          12
        } else {
          8
        }
      },
      0x21 => {
        // LD nn into HL
        let value = self.read_short_immediate(memory);
        self.h = value.hi();
        self.l = value.lo();
        12
      },
      0x23 => {
        // INC HL
        let val = self.hl().wrapping_add(1);
        self.h = val.hi();
        self.l = val.lo();
        8
      },
      0x28 => {
        // JR Z,r8
        let rel_target = self.read_signed_byte_immediate(memory);
        if self.f.contains(ZERO) {
          self.relative_jump(rel_target);
          12
        } else {
          8
        }
      },
      0x2a => {
        // LD A,(HL+)
        self.a = memory.read_byte(self.hl());
        let val = self.hl().wrapping_add(1);
        self.h = val.hi();
        self.l = val.lo();
        8
      },
      0x31 => {
        // LD short as sp
        let value = self.read_short_immediate(memory);
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
      0x3a => {
        // LD A,(HL-)
        self.a = memory.read_byte(self.hl());
        let val = self.hl().wrapping_sub(1);
        self.h = val.hi();
        self.l = val.lo();
        8
      },
      0x3e => {
        // LD # into A
        let result = self.read_byte_immediate(memory);
        self.a = result;
        8
      },
      0x4d => {
        // LD L into C
        self.c = self.l;
        4
      },
      0x5d => {
        // LD E,L
        self.e = self.l;
        4
      },
      0x78 => {
        // LD A,B
        self.a = self.b;
        4
      },
      0x7a => {
        // LD A,D
        self.a = self.d;
        4
      },
      0x7b => {
        // LD A,E
        self.a = self.e;
        4
      },
      0x7c => {
        // LD A, H
        self.a = self.h;
        4
      }
      0x7d => {
        // LD A,L
        self.a = self.l;
        4
      },
      0x8e => {
        // ADC (HL)
        let original = self.a;
        let value = memory.read_byte(self.hl()).wrapping_add(self.f.bits & CARRY.bits);
        self.a = self.a.wrapping_add(value);
        self.f.set(ZERO, self.a == 0);
        self.f.remove(SUBTRACT);
        self.f.set(HALF_CARRY, (original ^ value ^ self.a) & 0x10 == 0x10);
        self.f.set(CARRY, self.a < original);
        8
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
      0xb1 => {
        // OR B
        self.a |= self.b;
        self.f.set(ZERO, self.a == 0);
        self.f.remove(SUBTRACT);
        self.f.remove(HALF_CARRY);
        self.f.remove(CARRY);
        4
      },
      0xc0 => {
        // RET NZ
        let dest = self.pop_short(memory);
        if !self.f.contains(ZERO) {
          self.program_counter = dest;
          20
        } else {
          self.push_short(memory, dest);
          8
        }
      },
      0xc1 => {
        // POP BC
        self.c = self.pop_byte(memory);
        self.b = self.pop_byte(memory);
        12
      },
      0xc3 => {
        // JMP nn
        let target = memory.read_short(self.program_counter);
        self.program_counter = target;
        16
      },
      0xc5 => {
        // PUSH BC
        let b = self.b;
        self.push_byte(memory, b);
        let c = self.c;
        self.push_byte(memory, c);
        16
      },
      0xcd => {
        let pc = self.program_counter;
        self.push_short(memory, pc);
        let target = memory.read_short(self.program_counter);
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
        let dest = self.read_short_immediate(memory);
        memory.write_byte(dest, self.a);
        16
      },
      0xe0 => {
        // LDH n,A
        let offset = self.read_byte_immediate(memory);
        memory.write_byte(0xFF00 + offset as u16, self.a);
        12
      },
      0xe1 => {
        // POP HL
        self.l = self.pop_byte(memory);
        self.h = self.pop_byte(memory);
        12
      }
      0xe5 => {
        // PUSH HL
        let hl = self.hl();
        self.push_short(memory, hl);
        16
      },
      0xf0 => {
        // LDH A,n
        let offset = self.read_byte_immediate(memory);
        self.a = memory.read_byte(0xFF00 + offset as u16);
        12
      },
      0xf1 => {
        // POP AF
        self.f = Flags::from_bits_truncate(self.pop_byte(memory));
        self.a = self.pop_byte(memory);
        12
      },
      0xf3 => {
        // Disable interrupts
        self.interrupts = false;
        4
      },
      0xfe => {
        // Compare A with #
        let value = self.read_byte_immediate(memory);
        self.f.set(ZERO, self.a == value);
        self.f.insert(SUBTRACT);
        self.f.set(HALF_CARRY, (self.a ^ value ^ self.a.wrapping_sub(value)) & 0x10 == 0x10);
        self.f.set(CARRY, self.a > value);
        8
      },
      0xf5 => {
        // PSH AF
        let af = self.af();
        self.push_short(memory, af);
        16
      }
      0xff => {
        // RST 38
        let pc = self.program_counter;
        self.push_short(memory, pc);
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

  fn push_short(&mut self, memory: &mut Memory, value: u16) {
    self.push_byte(memory, value.hi());
    self.push_byte(memory, value.lo());
  }

  fn push_byte(&mut self, memory: &mut Memory, value: u8) {
    self.decrement_sp();
    memory.write_byte(self.stack_pointer, value);
  }

  fn pop_short(&mut self, memory: &Memory) -> u16 {
    let mut x: u16 = memory.read_byte(self.stack_pointer) as u16;
    self.increment_sp();
    x = (memory.read_byte(self.stack_pointer) as u16) << 8 | x;
    self.increment_sp();
    x
  }

  fn pop_byte(&mut self, memory: &Memory) -> u8 {
    let x = memory.read_byte(self.stack_pointer);
    self.increment_sp();
    x
  }

  fn read_short_immediate(&mut self, memory: &Memory) -> u16 {
    let value = memory.read_short(self.program_counter);
    self.program_counter += 2;
    value
  }

  fn read_byte_immediate(&mut self, memory: &Memory) -> u8 {
    let value = memory.read_byte(self.program_counter);
    self.program_counter += 1;
    value
  }

  fn read_signed_byte_immediate(&mut self, memory: &Memory) -> i8 {
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

  fn hl(&self) -> u16 {
    (self.h as u16) << 8 | self.l as u16
  }

  fn af(&self) -> u16 {
    (self.a as u16) << 8 | self.f.bits as u16
  }

  fn bc(&self) -> u16 {
    (self.b as u16) << 8 | self.c as u16
  }
}