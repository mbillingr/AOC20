use common::input::Input;
use common::regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = Input::from_file("data/day04-input.txt");

    let passports = input
        .iter_blocks()
        .map(|block| {
            block.split_whitespace().map(|entry| {
                let mut e = entry.split(':');
                (e.next().unwrap(), e.next().unwrap())
            })
        })
        .map(PassPort::from_iter)
        .collect::<Vec<_>>();

    let n_valid = passports.iter().filter(|p| p.has_all_fields()).count();
    println!("Part 1: {}", n_valid);

    let n_valid = passports.iter().filter(|p| p.is_valid().is_some()).count();
    println!("Part 2: {}", n_valid);
}

#[derive(Debug)]
struct PassPort {
    fields: HashMap<String, String>,
}

impl PassPort {
    fn from_iter<'a>(iter: impl IntoIterator<Item = (&'a str, &'a str)>) -> Self {
        PassPort {
            fields: iter
                .into_iter()
                .map(|(key, val)| (key.to_owned(), val.to_owned()))
                .collect(),
        }
    }

    fn has_all_fields(&self) -> bool {
        for &required_key in &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
            if !self.fields.contains_key(required_key) {
                return false;
            }
        }
        true
    }

    fn is_valid(&self) -> Option<()> {
        self.validate_date_range("byr", 1920, 2002)?;
        self.validate_date_range("iyr", 2010, 2020)?;
        self.validate_date_range("eyr", 2020, 2030)?;

        self.fields
            .get("hgt")
            .map(|val| {
                if val.ends_with("cm") {
                    val.split("cm")
                        .next()
                        .unwrap()
                        .parse::<u16>()
                        .ok()
                        .filter(|&val_cm| val_cm >= 150)
                        .filter(|&val_cm| val_cm <= 193)
                } else if val.ends_with("in") {
                    val.split("in")
                        .next()
                        .unwrap()
                        .parse::<u16>()
                        .ok()
                        .filter(|&val_in| val_in >= 59)
                        .filter(|&val_in| val_in <= 76)
                } else {
                    None
                }
            })
            .filter(Option::is_some)?;

        self.fields
            .get("hcl")
            .filter(|val| Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(val))?;

        self.fields.get("ecl").filter(|val| {
            Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$")
                .unwrap()
                .is_match(val)
        })?;

        self.fields
            .get("pid")
            .filter(|val| Regex::new(r"^[0-9]{9}$").unwrap().is_match(val))?;

        Some(())
    }

    fn validate_date_range(&self, key: &str, min: u16, max: u16) -> Option<()> {
        self.fields
            .get(key)
            .and_then(|val| val.parse::<u16>().ok())
            .filter(|&val| val >= min)
            .filter(|&val| val <= max)
            .map(|_| ())
    }
}
