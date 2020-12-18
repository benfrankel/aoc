use crate::prelude::*;

pub fn parse(input: &str) -> String {
    input.to_string()
}

pub fn part1(input: &str) -> impl Debug {
    let expected = hashset! {
        "byr".to_string(),
        "iyr".to_string(),
        "eyr".to_string(),
        "hgt".to_string(),
        "hcl".to_string(),
        "ecl".to_string(),
        "pid".to_string(),
    };
    let allowed = hashset! {
        "cid".to_string(),
    };

    let mut count = 0;
    'outer: for passport in input.split("\n\n") {
        let mut seen = hashset! {};
        for entry in passport.split_whitespace() {
            let (key, _) = entry.split2(":");
            if !expected.contains(&key) && !allowed.contains(&key) {
                continue 'outer;
            }
            if !seen.insert(key) {
                continue 'outer;
            }
        }

        if expected.intersection(&seen).count() == expected.len() {
            count += 1;
        }
    }

    count
}

fn valid_byr(value: &str) -> bool {
    value.len() == 4 && (1920..=2002).contains(&value.parse().unwrap_or(0))
}

fn valid_iyr(value: &str) -> bool {
    value.len() == 4 && (2010..=2020).contains(&value.parse().unwrap_or(0))
}

fn valid_eyr(value: &str) -> bool {
    value.len() == 4 && (2020..=2030).contains(&value.parse().unwrap_or(0))
}

fn valid_hgt(value: &str) -> bool {
    value.len() >= 3
        && (value.ends_with("cm")
            && (150..=193)
                .contains(&value[..value.len() - 2].parse().unwrap_or(0))
            || value.ends_with("in")
                && (59..=76)
                    .contains(&value[..value.len() - 2].parse().unwrap_or(0)))
}

fn valid_hcl(value: &str) -> bool {
    value.len() == 7
        && value.starts_with("#")
        && value
            .chars()
            .skip(1)
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
}

fn valid_ecl(value: &str) -> bool {
    let ecl = hashset! {
        "amb".to_string(),
        "blu".to_string(),
        "brn".to_string(),
        "gry".to_string(),
        "grn".to_string(),
        "hzl".to_string(),
        "oth".to_string(),
    };
    ecl.contains(value)
}

fn valid_pid(value: &str) -> bool {
    value.len() == 9 && value.chars().all(|c| c.is_ascii_digit())
}

pub fn part2(input: &str) -> impl Debug {
    let expected = hashset! {
        "byr".to_string(),
        "iyr".to_string(),
        "eyr".to_string(),
        "hgt".to_string(),
        "hcl".to_string(),
        "ecl".to_string(),
        "pid".to_string(),
    };
    let allowed = hashset! {
        "cid".to_string(),
    };

    let mut count = 0;
    'outer: for passport in input.split("\n\n") {
        let mut seen = hashset! {};
        for entry in passport.split_whitespace() {
            let (key, value) = entry.split2(":");
            if !match key.as_str() {
                "byr" => valid_byr(&value),
                "iyr" => valid_iyr(&value),
                "eyr" => valid_eyr(&value),
                "hgt" => valid_hgt(&value),
                "hcl" => valid_hcl(&value),
                "ecl" => valid_ecl(&value),
                "pid" => valid_pid(&value),
                x if allowed.contains(x) => true,
                _ => false,
            } || !seen.insert(key)
            {
                continue 'outer;
            }
        }

        if expected.intersection(&seen).count() == expected.len() {
            count += 1;
        }
    }

    count
}
