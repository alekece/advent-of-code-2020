#![feature(iterator_fold_self)]

use std::{collections::HashSet, fs, iter::FromIterator, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
  path: PathBuf,
  #[structopt(short, long)]
  corrected_instructions: bool,
}

fn main() {
  let opt = Opt::from_args();

  let content = fs::read_to_string(opt.path.clone()).expect("could not read input");

  let answers: usize = content
    .split("\n\n")
    .map(|s| {
      if opt.corrected_instructions {
        s.split_whitespace()
          .map(|s| HashSet::<char>::from_iter(s.chars()))
          .fold_first(|acc, set| acc.intersection(&set).cloned().collect())
          .unwrap_or(HashSet::default())
      } else {
        HashSet::<char>::from_iter(s.split_whitespace().flat_map(|s| s.chars()))
      }
    })
    .map(|yeses| yeses.len())
    .sum();

  println!("Yes answers : {}", answers);
}
