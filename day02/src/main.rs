#[macro_use]
extern crate lazy_static;

mod password;

use password::{Password, PasswordPolicy};
use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
  path: PathBuf,
  #[structopt(short, long, default_value = "sled")]
  policy: PasswordPolicy,
}

fn main() {
  let opt = Opt::from_args();

  let valid_passwords = fs::read_to_string(opt.path.clone())
    .expect("could not read input")
    .split('\n')
    .filter_map(Password::from_str)
    .filter(|password| password.is_valid(&opt.policy))
    .count();

  println!("{}", valid_passwords);
}
