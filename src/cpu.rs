// TODO: use bitflags?
struct Flag {
  zero: bool,
  subtract: bool,
  half_carry: bool,
  carry: bool
}

struct CPU {
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
  fn new() -> CPU {
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

  fn step(&mut self) {
    // Fetch
    let opcode: u8 = unimplemented!();
    // Increment
    self.program_counter += 1;
    // Execute
    match opcode {
      0x00 => {
        // NOP
        self.cycles += 4;
      },
      _ => unimplemented!()
    }
  }
}