use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
  path: PathBuf,
  #[structopt(short, long, default_value = "3,1", parse(try_from_str = parse_slope))]
  slopes: Vec<Slope>,
}

fn parse_slope(s: &str) -> Result<Slope, String> {
  match s.split(',').collect::<Vec<_>>() {
    slope if slope.len() == 2 => Ok(Slope {
      step_x: slope[0].parse().unwrap(),
      step_y: slope[1].parse().unwrap(),
    }),
    _ => Err(format!("invalid slope format: {}", s)),
  }
}

struct Slope {
  step_x: usize,
  step_y: usize,
}

fn count_trees(raw_world: &[&str], slope: &Slope) -> usize {
  raw_world
    .iter()
    .enumerate()
    .step_by(slope.step_y)
    .skip(1)
    .map(|(i, s)| s.chars().cycle().nth(i * slope.step_x) == Some('#'))
    .filter(|x| *x)
    .count()
}

fn main() {
  let opt = Opt::from_args();

  let raw_world = fs::read_to_string(opt.path).expect("could not read input");
  let raw_world = raw_world.split('\n').collect::<Vec<_>>();

  let encountered_trees: usize = opt
    .slopes
    .iter()
    .map(|slope| count_trees(&raw_world, &slope))
    .product();

  println!("{}", encountered_trees);
}
