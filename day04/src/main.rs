#![feature(str_split_once)]

mod passport;

use passport::Passport;
use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
  path: PathBuf,
  #[structopt(short, long)]
  force_validation: bool,
}

fn main() {
  let opt = Opt::from_args();

  let content = fs::read_to_string(opt.path.clone()).expect("could not read iput");

  let passports = content
    .split("\n\n")
    .filter_map(Passport::from_str)
    .filter(|passport| passport.is_valid(opt.force_validation));

  println!("valid passports: {}", passports.count());
}
