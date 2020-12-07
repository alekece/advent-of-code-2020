use regex::Regex;
use std::str::FromStr;

lazy_static! {
  static ref PASSWORD_PATTERN: Regex = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();
}

pub enum PasswordPolicy {
  Sled,
  Toboggan,
}

impl FromStr for PasswordPolicy {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "sled" => Ok(PasswordPolicy::Sled),
      "toboggan" => Ok(PasswordPolicy::Toboggan),
      _ => Err("unknown password policy"),
    }
  }
}

pub struct Password {
  settings: (usize, usize, char),
  password: String,
}

impl Password {
  pub fn from_str(s: &str) -> Option<Self> {
    for capture in PASSWORD_PATTERN.captures_iter(s) {
      return Some(Self {
        settings: (
          capture[1].parse().ok()?,
          capture[2].parse().ok()?,
          capture[3].parse().ok()?,
        ),
        password: capture[4].to_string(),
      });
    }

    None
  }

  pub fn is_valid(&self, policy: &PasswordPolicy) -> bool {
    match policy {
      PasswordPolicy::Sled => (self.settings.0..=self.settings.1)
        .contains(&(self.password.matches(self.settings.2).count())),
      PasswordPolicy::Toboggan => self
        .password
        .char_indices()
        .filter(|(i, _)| i == &(self.settings.0 - 1) || i == &(self.settings.1 - 1))
        .map(|(_, c)| c == self.settings.2)
        .fold(false, |a, b| a ^ b),
    }
  }
}
