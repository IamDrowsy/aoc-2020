use regex::Regex;
use itertools::{Itertools};
use std::ops::RangeInclusive;
use std::collections::{HashSet, HashMap};

pub(crate) struct Solver;

lazy_static! {
    static ref FIELDS: Regex = Regex::new(r"([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    static ref TICKET_NUMBERS: Regex = Regex::new(r"\d+").unwrap();
}

struct Field {
    name: String,
    valid_numbers: HashSet<u32>
}

fn ranges_to_hashset(range1: RangeInclusive<u32>, range2: RangeInclusive<u32>) -> HashSet<u32> {
    let mut result = HashSet::new();
    for i in range1 {
        result.insert(i);
    }
    for i in range2 {
        result.insert(i);
    }
    return result;
}

fn match_to_int(pattern: regex::Match) -> u32 {
    pattern.as_str().parse::<u32>().unwrap()
}

fn invalid_numbers_sum(valid_numbers: &HashSet<u32>, ticket: &Vec<u32>) -> u32 {
    ticket.iter().filter(|number| !valid_numbers.contains(number)).sum()
}

fn is_ticket_valid(valid_numbers: &HashSet<u32>, ticket: &Vec<u32>) -> bool {
    ticket.iter().all(|number| valid_numbers.contains(number))
}

fn parse_input(input: &str) -> (Vec<Field>, Vec<u32>, Vec<Vec<u32>>) {
    let splitted = input.split("\n\n").collect::<Vec<&str>>();
    let fields = splitted[0].lines().map(|field| FIELDS.captures(field).unwrap())
        .map(|cap| Field {
            name: cap.get(1).unwrap().as_str().to_string(),
            valid_numbers: ranges_to_hashset(match_to_int(cap.get(2).unwrap())..=match_to_int(cap.get(3).unwrap()),
                                             match_to_int(cap.get(4).unwrap())..=match_to_int(cap.get(5).unwrap()))
        }).collect();
    let my_ticket = TICKET_NUMBERS.find_iter(splitted[1].lines().nth(1).unwrap()).map(match_to_int).collect();
    let other_tickets: Vec<Vec<u32>> = splitted[2].lines().dropping(1).map(|ticket| TICKET_NUMBERS.find_iter(ticket).map(match_to_int).collect()).collect();
    (fields, my_ticket, other_tickets)
}

impl crate::utils::Solver for Solver {
    fn part1(&self, input: &str) -> String {
        let (fields, _, other_tickets) = parse_input(input);
        let all_valid_numbers = fields.iter().fold(HashSet::new(), |mut set, field| {set.extend(&field.valid_numbers); return set});
        let result: u32 = other_tickets.iter().map(|ticket| invalid_numbers_sum(&all_valid_numbers, ticket)).sum();
        return result.to_string()
    }
    fn part2(&self, input: &str) -> String {
        let (fields, my_ticket, other_tickets) = parse_input(input);
        let all_valid_numbers = fields.iter().fold(HashSet::new(), |mut set, field| {set.extend(&field.valid_numbers); return set});
        let valid_tickets: Vec<&Vec<u32>> = other_tickets.iter().filter(|ticket| is_ticket_valid(&all_valid_numbers, ticket)).collect();
        let mut numbers_for_fields: Vec<HashSet<u32>> = other_tickets[0].iter().map(|_| HashSet::new()).collect();
        for tickets in valid_tickets {
            for (i, number) in tickets.iter().enumerate() {
                let found_numbers = &mut numbers_for_fields[i];
                found_numbers.insert(number.clone());
            }
        }

        let mut valid_index_for_name: HashMap<String,HashSet<u32>> = fields.iter().map(|field| (field.name.clone(), HashSet::new())).collect();
        for field in fields {
            for (i, found_numbers) in numbers_for_fields.iter().enumerate() {
                if found_numbers.is_subset(&field.valid_numbers) {
                    valid_index_for_name.get_mut(&field.name).unwrap().insert(i as u32);
                }
            }
        }
        let mut index_for_name: HashMap<String,u32> = HashMap::new();
        let mut known_index: HashSet<u32> = HashSet::new();
        for (field_name, valid_index) in valid_index_for_name.iter().sorted_by(|(_a,b), (_a2, b2)|  b.len().cmp(&b2.len())) {
            index_for_name.insert(field_name.to_string(), valid_index.difference(&known_index).nth(0).unwrap().clone());
            known_index.extend(valid_index);
        }
        let departure_index: Vec<usize> = index_for_name.iter().filter(|(k, _v)| k.starts_with("departure")).map(|(_k, v)| *v as usize).collect();
        let result = departure_index.iter().fold(1, |x, &i| x * my_ticket[i] as u64);
        return result.to_string();
    }
}