use std::collections::HashSet;

pub(crate) struct Solver;

fn is_valid(window: &[i64], candidate: i64) -> bool {
    let mut lookup = HashSet::new();
    let mut candidates = HashSet::new();
    for n in window {
        lookup.insert(n.clone());
        if candidate != 2*n {
            // we don't need to check n+n = candidate because both have to be different
            candidates.insert(candidate - n);
        }
    }
    lookup.intersection(&candidates).count() > 0
}

impl crate::utils::Solver for Solver {
    fn part1(&self, input: &str) -> String {
        let preamble = 25;
        let numbers: Vec<i64> = input.lines().map(|s| s.parse().unwrap()).collect();
        let mut result = 0;
        for i in preamble..numbers.len() {
            if is_valid(&numbers[(i-preamble)..i], numbers[i]) {
                // nothing todo;
            } else {
                result = numbers[i];
                break;
            }
        };
       return result.to_string()
    }
    fn part2(&self, input: &str) -> String {
        let exploit: i64 = self.part1(input).parse().unwrap();
        let numbers: Vec<i64> = input.lines().map(|s| s.parse().unwrap()).collect();
        let mut lower = 0; let mut upper = 0;
        let mut sum: i64;
        loop {
            sum = numbers[lower..upper].iter().sum();
            if sum == exploit {
                break;
            } else if sum > exploit {
                lower += 1;
            } else {
                upper += 1;
            }
        }
        (numbers[lower..upper].iter().max().unwrap() + numbers[lower..upper].iter().min().unwrap()).to_string()
    }
}