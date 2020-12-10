use regex::Regex;
use std::{
  cmp::{Eq, PartialEq},
  collections::HashSet,
  hash::{Hash, Hasher},
  iter::FromIterator,
};

#[derive(Debug, Clone)]
pub struct Bag {
  pub color: String,
  pub bags: Vec<(usize, String)>,
}

impl Hash for Bag {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.color.hash(state);
  }
}

impl PartialEq for Bag {
  fn eq(&self, other: &Self) -> bool {
    self.color.eq(&other.color)
  }
}

impl Eq for Bag {}

impl Bag {
  pub fn from_str(s: &str) -> Option<Self> {
    let re = Regex::new(r"^(.+) bags contain (?:no other bags|(.+)).$").unwrap();

    for capture in re.captures_iter(s) {
      return Some(Self {
        color: capture[1].to_string(),
        bags: match capture.get(2) {
          Some(capture) => capture
            .as_str()
            .split(',')
            .map(|s| {
              s.trim()
                .split_once(' ')
                .map(|(quantity, color)| {
                  (
                    quantity.parse().unwrap(),
                    color
                      .strip_suffix(" bag")
                      .or_else(|| color.strip_suffix(" bags"))
                      .map(|color| color.to_string())
                      .unwrap(),
                  )
                })
                .unwrap()
            })
            .collect(),
          None => Vec::default(),
        },
      });
    }

    None
  }

  pub fn find_parent_bags(color: &str, bags: &Vec<Self>) -> HashSet<Bag> {
    HashSet::<Bag>::from_iter(
      bags
        .iter()
        .filter(|bag| bag.contains_bag(&color))
        .cloned()
        .flat_map(|bag| {
          let mut parent_bags = vec![bag.clone()];

          parent_bags.append(
            &mut Self::find_parent_bags(&bag.color, &bags)
              .into_iter()
              .collect(),
          );

          parent_bags
        }),
    )
  }

  pub fn count_bags(color: &str, bags: &Vec<Self>) -> usize {
    bags
      .iter()
      .find(|bag| bag.color.eq(&color))
      .map(|bag| {
        bag.bags.iter().fold(1, |acc, (quantity, color)| {
          acc + quantity * Self::count_bags(&color, &bags)
        })
      })
      .unwrap()
  }

  pub fn contains_bag(&self, color: &str) -> bool {
    self.bags.iter().find(|bag| bag.1.eq(&color)).is_some()
  }
}
