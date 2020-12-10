#![feature(str_split_once)]
mod bag;

use bag::Bag;
use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
  path: PathBuf,
  #[structopt(short, long, default_value = "shiny gold")]
  color: String,
  #[structopt(short, long)]
  nested_bag: bool,
}

fn main() {
  let opt = Opt::from_args();

  let content = fs::read_to_string(&opt.path).expect("could not read input");

  let bags = content
    .split('\n')
    .filter_map(Bag::from_str)
    .collect::<Vec<_>>();
  let bags = if opt.nested_bag {
    // remove the "shiny gold" bag to the counting
    Bag::count_bags(&opt.color, &bags) - 1
  } else {
    Bag::find_parent_bags(&opt.color, &bags).len()
  };

  println!("bags: {:?}", bags);
}
