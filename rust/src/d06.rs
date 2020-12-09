use std::collections::HashSet;
use itertools::Itertools;

pub(crate) struct Solver;

fn yes_answers_in_group(group_string: &str) -> usize {
    group_string.chars().filter(|c| c != &'\n').unique().count()
}

fn yes_answer_for_all_in_group(group_string: &str) -> usize {
    group_string.lines()
        .map(|answers| answers.chars().collect::<HashSet<char>>())
        .fold1(|s1, s2| s1.intersection(&s2).cloned().collect())
        .unwrap()
        .len()
}

impl crate::utils::Solver for Solver {
    fn part1(&self, input: &str) -> String {
        input.split("\n\n")
            .map(yes_answers_in_group)
            .fold(0, |x,y| x + y)
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        input.split("\n\n")
            .map(yes_answer_for_all_in_group)
            .fold(0, |x,y| x + y)
            .to_string()
    }
}