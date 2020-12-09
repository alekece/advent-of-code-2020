use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
  path: PathBuf,
  #[structopt(short, long)]
  find_seat: bool,
}

fn compute_pivot(min: u64, max: u64) -> u64 {
  max - ((max - min) / 2)
}

fn compute_seat_id(s: &str) -> Option<u64> {
  if s.len() != 10 {
    return None;
  }

  let row = s.chars().take(7).fold(Some((0, 127)), |acc, letter| {
    acc.and_then(|(min, max)| match letter {
      'F' => Some((min, compute_pivot(min, max) - 1)),
      'B' => Some((compute_pivot(min, max), max)),
      _ => None,
    })
  });

  let col = s.chars().skip(7).fold(Some((0, 7)), |acc, letter| {
    acc.and_then(|(min, max)| match letter {
      'L' => Some((min, compute_pivot(min, max) - 1)),
      'R' => Some((compute_pivot(min, max), max)),
      _ => None,
    })
  });

  match (row, col) {
    (Some((row, _)), Some((_, col))) => Some(row * 8 + col),
    _ => None,
  }
}

fn find_seat(seats: &mut Vec<u64>) -> Option<u64> {
  seats.sort();

  let mut it = seats.iter_mut().peekable();

  while let Some(seat) = it.next() {
    let found_seat = it
      .peek()
      .and_then(|next_seat| match next_seat.saturating_sub(*seat) {
        2 => Some(*seat + 1),
        _ => None,
      });

    if found_seat.is_some() {
      return found_seat;
    }
  }

  None
}

fn main() {
  let opt = Opt::from_args();

  let content = fs::read_to_string(opt.path).expect("could not read input");

  let mut seats = content
    .split('\n')
    .filter_map(compute_seat_id)
    .collect::<Vec<_>>();

  if opt.find_seat {
    println!("Found seat: {:?}", find_seat(&mut seats));
  } else {
    println!("Hightest seat id: {}", seats.into_iter().max().unwrap());
  }
}
