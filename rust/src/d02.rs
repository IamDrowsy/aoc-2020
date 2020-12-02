pub(crate) struct Solver;
use stats::Frequencies;
use regex::Regex;

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
}

#[derive(Debug)]
struct Policy {
    c: char,
    min: u64,
    max: u64,
}

fn parse_line(line: &str) -> (Policy, String) {
    let parsed = LINE_REGEX.captures(line).unwrap();
    let policy =  Policy {
        c: (&parsed[3]).parse().unwrap(),
        min: (&parsed[1]).parse().unwrap(),
        max: (&parsed[2]).parse().unwrap()
    };
    return ( policy, parsed[4].to_string());
}

fn frequencies(password: String) -> Frequencies<char> {
    let mut result = Frequencies::new();
    for c in password.chars() {
        result.add(c);
    }
    return result;
}

fn matches_part1((Policy {c, min, max}, password): &(Policy, String)) -> bool {
    let in_password = frequencies(password.to_string()).count(&c);
    return min <= &in_password && &in_password <= max;
}

fn matches_index(v: &Vec<char>, i: u64, c: &char) -> bool {
    if v.len() > i as usize {
        return &v[i as usize] == c;
    } else {
        return false;
    }
}

fn matches_part2((Policy {c, min: pos1, max: pos2}, password): &(Policy, String)) -> bool {
    let chars: Vec<char> = password.chars().collect();
    let matches1 = matches_index(&chars,pos1 - 1, c);
    let matches2 = matches_index(&chars,pos2 - 1, c);
    return (matches1 && !matches2) || (!matches1 && matches2)
}

impl crate::utils::Solver for Solver {
    fn part1(&self, input: &str) -> String {
        return input
            .lines()
            .map(parse_line)
            .filter(matches_part1)
            .count()
            .to_string();
    }
    fn part2(&self, input: &str) -> String {
        return input
            .lines()
            .map(parse_line)
            .filter(matches_part2)
            .count()
            .to_string();
    }
}