use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Copy)]
enum Instruction {
  NoOp(i64),
  Accumulate(i64),
  Jump(i64),
  Unknown,
}

#[derive(Debug, Clone)]
pub struct VMMachine {
  global_register: i64,
  instruction_index: i64,
  instructions: Vec<(u64, Instruction)>,
}

impl VMMachine {
  pub fn from_file(path: PathBuf) -> Self {
    fs::read_to_string(path)
      .map(|s| Self {
        global_register: 0,
        instruction_index: 0,
        instructions: s
          .split('\n')
          .map(|s| {
            s.split_once(' ')
              .map(|(op, value)| match op {
                "nop" => Instruction::NoOp(value.parse().unwrap()),
                "acc" => Instruction::Accumulate(value.parse().unwrap()),
                "jmp" => Instruction::Jump(value.parse().unwrap()),
                _ => Instruction::Unknown,
              })
              .unwrap_or(Instruction::Unknown)
          })
          .map(|instruction| (0, instruction))
          .collect(),
      })
      .unwrap()
  }

  pub fn execute(&mut self) -> bool {
    loop {
      let (ref mut counter, instruction) = self
        .instructions
        .get_mut(self.instruction_index as usize)
        .unwrap();

      match *counter + 1 {
        2 => break false,
        n => *counter = n,
      };

      match instruction {
        Instruction::Accumulate(value) => self.global_register += *value,
        Instruction::Jump(value) => self.instruction_index += *value - 1,
        _ => (),
      };

      self.instruction_index += 1;

      if self.instruction_index as usize == self.instructions.len() {
        break true;
      }
    }
  }

  pub fn patch_and_execute(vm: &VMMachine) -> Option<VMMachine> {
    for index_instruction in
      vm.instructions
        .iter()
        .enumerate()
        .filter_map(|(i, (_, instruction))| match instruction {
          Instruction::NoOp(..) | Instruction::Jump(..) => Some(i),
          _ => None,
        })
    {
      let mut vm = vm.clone();
      let (_, ref mut instruction) = vm.instructions.get_mut(index_instruction).unwrap();

      *instruction = match instruction {
        Instruction::NoOp(value) => Instruction::Jump(*value),
        Instruction::Jump(value) => Instruction::NoOp(*value),
        _ => *instruction,
      };

      if vm.execute() {
        return Some(vm);
      }
    }

    None
  }

  pub fn global_register(&self) -> i64 {
    self.global_register
  }
}
