use std::collections::{HashMap};
use regex::{Regex};

pub(crate) struct Solver;

fn valid_number_between(number: &str, min: u64, max: u64) -> bool {
    match number.parse::<u64>() {
        Ok(v) => {min <= v && v <= max}
        Err(_) => {false}
    }
}

fn valid_height_internal(height: &str, height_type: &str, min: u64, max: u64) -> bool {
    let re = Regex::new(&*(r"^(\d+)".to_owned() + height_type + "$")).unwrap();
    match re.captures(&*height) {
        None => {false}
        Some(v) => {valid_number_between(&v[1], min, max)}
    }
}

fn valid_height(heigth: &str) -> bool {
    valid_height_internal(heigth, "cm", 150, 193)
        || valid_height_internal(heigth, "in", 59, 76)
}

fn valid_hair_color(hcl: &str) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    re.is_match(hcl)
}

fn valid_eye_color(ecl: &str) -> bool {
    let re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    re.is_match(ecl)
}

fn valid_pid(pid: &str) -> bool {
    let re = Regex::new(r"^\d{9}$").unwrap();
    re.is_match(pid)
}

fn valid(passport: &HashMap<&str, &str>, key: &str, f: fn(&str) -> bool) -> bool {
    match passport.get(key) {
        None => {false}
        Some(v) => {f(v)}
    }
}

fn is_valid2(passport: &HashMap<&str, &str>) -> bool {
    valid(passport, "byr", |s| valid_number_between(s, 1920, 2002))
        && valid(passport, "iyr", |s| valid_number_between(s, 2010, 2020))
        && valid(passport, "eyr", |s| valid_number_between(s, 2020, 2030))
        && valid(passport, "hgt", valid_height)
        && valid(passport, "hcl", valid_hair_color)
        && valid(passport, "ecl", valid_eye_color)
        && valid(passport, "pid", valid_pid)
}

fn is_valid1(passport: &HashMap<&str, &str>) -> bool {
    vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|key| passport.contains_key(key))
        .filter(|&b| !b).count() == 0
}

fn parse_passport(passport_str: &str) -> HashMap<&str, &str> {
    let mut result = HashMap::new();
    let re = Regex::new(r"([a-z]{3}):(\S*)").unwrap();
    for m in re.find_iter(passport_str) {
        let caps = re.captures(m.as_str()).unwrap();
        result.insert(caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str());
    }
    return result;
}

impl crate::utils::Solver for Solver {
    fn part1(&self, input: &str) -> String {
        let re = Regex::new(r"\n\n").unwrap();
        re.split(input)
            .map(parse_passport)
            .filter(|passport| is_valid1(passport))
            .count().to_string()
    }
    fn part2(&self, input: &str) -> String {
        let re = Regex::new(r"\n\n").unwrap();
        re.split(input)
            .map(parse_passport)
            .filter(|passport| is_valid2(passport))
            .count().to_string()
    }
}