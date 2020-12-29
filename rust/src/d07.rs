use std::collections::{HashMap, HashSet};
use regex::{Regex};

pub(crate) struct Solver;

lazy_static! {
   static ref TARGET_BAG: Regex = Regex::new(r"^([a-z]+ [a-z]+) bags contain").unwrap();
   static ref ALLOWED_BAGS: Regex = Regex::new(r"(\d+) ([a-z]+ [a-z]+) bag").unwrap();
}

fn add_rule(all_rules: &mut HashMap<String, HashMap<String, usize>>, rule_string: &str)  {
    let color = TARGET_BAG.captures(rule_string).unwrap().get(1).unwrap();
    let allowed_captures = ALLOWED_BAGS.captures_iter(rule_string);
    let mut allows:HashMap<String,usize> = HashMap::new();
    for allowed in allowed_captures {
        allows.insert(allowed.get(2).unwrap().as_str().to_string(),
                      allowed.get(1).unwrap().as_str().parse().unwrap());
    }
    all_rules.insert(color.as_str().to_string(), allows);
}

fn can_be_inside(all_rules: HashMap<String, HashMap<String, usize>>) -> HashMap<String, HashSet<String>> {
    let mut result: HashMap<String,HashSet<String>> = HashMap::new();
    for (color, allowed) in all_rules {
        for allowed_inside_color in allowed.keys() {
            if result.contains_key(allowed_inside_color) {
                result.get_mut(allowed_inside_color).unwrap().insert(color.to_string());
            } else {
                let mut s = HashSet::new();
                s.insert(color.to_string());
                result.insert(allowed_inside_color.to_string(), s);
            }
        }
    }
    return result
}

fn step_insert_colors(can_be_inside: &HashMap<String,HashSet<String>>, mut current: HashSet<String>) -> HashSet<String> {
    for color in current.clone() {
        for new_color in can_be_inside.get(color.as_str()).unwrap_or(&HashSet::new()) {
            current.insert(new_color.to_string());
        }
    }
    return current;
}

fn parse_rules(input: &str) -> HashMap<String,HashMap<String,usize>> {
    let mut result: HashMap<String, HashMap<String,usize>> = HashMap::new();
    for line in input.lines() {
        add_rule(&mut result, line);
    }
    return result;

}

fn all_keys_known(known: &HashMap<String,usize>, needed: &HashMap<String,usize>) -> bool {
    needed.keys().all(|k| known.contains_key(k))
}

fn sum_bags(known: &HashMap<String,usize>, needed: &HashMap<String,usize>) -> usize {
    let mut result = 0;
    for color in needed.keys() {
        result += (known.get(color).unwrap() + 1) * needed.get(color).unwrap();
    }
    return result;
}

impl crate::utils::Solver for Solver {
    fn part1(&self, input: &str) -> String {
        let all_rules = parse_rules(input);
        let can_be_inside = can_be_inside(all_rules);
        let mut current_count = 1;
        let mut last_count = 0;
        let mut start = HashSet::new();
        start.insert("shiny gold".to_string());
        while last_count != current_count {
            last_count = start.len();
            start = step_insert_colors(&can_be_inside, start);
            current_count = start.len();
        }
        return (start.len()-1).to_string();
    }

    fn part2(&self, input: &str) -> String {
        let all_rules = parse_rules(input);
        let mut known_numbers: HashMap<String,usize> =  HashMap::new();
        while !known_numbers.contains_key("shiny gold") {
            for (color, content) in all_rules.iter() {
                if !known_numbers.contains_key(color) && all_keys_known(&known_numbers, content) {
                    known_numbers.insert(color.clone(), sum_bags(&known_numbers, content));
                }
            }
        }
        return known_numbers.get("shiny gold").unwrap().to_string();
    }
}