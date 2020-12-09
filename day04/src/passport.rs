use regex::Regex;
use std::collections::HashMap;

fn validate_pattern(pattern: &str, value: &str) -> bool {
  Regex::new(pattern)
    .map(|re| re.is_match(value))
    .unwrap_or(false)
}

fn validate_range(min: u64, max: u64, value: &str) -> bool {
  value
    .parse::<u64>()
    .map(|value| (min..=max).contains(&value))
    .unwrap_or(false)
}

#[derive(Debug)]
pub struct Passport {
  pub fields: HashMap<String, String>,
}

impl Passport {
  pub fn from_str(s: &str) -> Option<Self> {
    let mut passport = Self {
      fields: HashMap::default(),
    };

    for (key, value) in s.split_ascii_whitespace().filter_map(|s| s.split_once(':')) {
      passport.fields.insert(key.to_string(), value.to_string());
    }

    Some(passport)
  }

  pub fn is_valid(&self, force_validation: bool) -> bool {
    ["pid", "byr", "iyr", "eyr", "hgt", "hcl", "ecl"]
      .iter()
      .all(|key| {
        self
          .fields
          .get(&key.to_string())
          .map(|value| match *key {
            "pid" => validate_pattern(r"^\d{9}$", &value),
            "byr" => validate_range(1920, 2002, &value),
            "iyr" => validate_range(2010, 2020, &value),
            "eyr" => validate_range(2020, 2030, &value),
            "hgt" => Regex::new(r"(\d{2,3})(cm|in)")
              .map(|re| {
                for capture in re.captures_iter(value) {
                  if let Some((min, max)) = match &capture[2] {
                    "cm" => Some((150, 193)),
                    "in" => Some((59, 76)),
                    _ => None,
                  } {
                    return validate_range(min, max, &capture[1]);
                  }
                }

                false
              })
              .unwrap_or(false),
            "hcl" => validate_pattern(r"#[0-9a-f]{6}", &value),
            "ecl" => validate_pattern(r"amb|blu|brn|gry|grn|hzl|oth", &value),
            _ => true,
          })
          .map(|result| match force_validation {
            true => true,
            false => result,
          })
          .unwrap_or(false)
      })
  }
}
