#![feature(str_split_once)]

mod vm;

use std::path::PathBuf;
use structopt::StructOpt;

use vm::VMMachine;

#[derive(StructOpt)]
struct Opt {
  path: PathBuf,
  #[structopt(short, long)]
  patch_instructions: bool,
}

fn main() {
  let opt = Opt::from_args();

  let mut vm = VMMachine::from_file(opt.path);

  if opt.patch_instructions {
    vm = VMMachine::patch_and_execute(&vm).unwrap();
  } else {
    vm.execute();
  }

  println!("global register: {}", vm.global_register());
}
