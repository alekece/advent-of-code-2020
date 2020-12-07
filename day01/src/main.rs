use std::{fs, path::PathBuf};

use itertools::Itertools;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
  path: PathBuf,
  #[structopt(short, long, default_value = "1")]
  permutation_chunk: usize,
}

fn compute_expense(expenses: &[u64], permutation_chunk: usize) -> Vec<u64> {
  expenses
    .iter()
    .copied()
    .permutations(permutation_chunk)
    .filter_map(|permutation| {
      if permutation.iter().sum::<u64>() == 2020 {
        Some(permutation.iter().product())
      } else {
        None
      }
    })
    .collect()
}

fn main() {
  let opt = Opt::from_args();

  let content = fs::read_to_string(opt.path)
    .expect("could not read input")
    .split('\n')
    .filter_map(|x| x.parse().ok())
    .collect::<Vec<_>>();

  let result = compute_expense(&content, opt.permutation_chunk);

  println!("{:?}", result);
}
