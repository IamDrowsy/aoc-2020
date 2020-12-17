
use std::collections::HashMap;

pub(crate) struct Solver;

#[derive(Debug)]
enum Before {Once(i32), Twice(i32, i32)}

fn insert(before: Option<&Before>, new: i32) -> Before {
    match before {
        None => Before::Once(new),
        Some(Before::Once(x)) => {
            Before::Twice(new, x.clone())},
        Some(Before::Twice(x, _)) => Before::Twice(new, x.clone())
    }
}

fn solve(input: &str, max_index: i32) -> String {
    let mut previously_spoken = HashMap::new();
    let mut last_spoken= 0;
    let mut current_index = 1;
    for spoken in input.split(",").map(|s| s.parse::<i32>().unwrap()) {
        previously_spoken.insert(spoken, Before::Once(current_index));
        current_index += 1;
        last_spoken = spoken;
    }

    for i in (current_index)..max_index+1 {
        if i % 100000 == 0 {
            println!("{}", i);
        }
        let previously = previously_spoken.get(&last_spoken).unwrap();
        let spoken = match previously {
            Before::Once(_) => 0,
            Before::Twice(last_index, before_last_index) => last_index - before_last_index
        };

        previously_spoken.insert(spoken, insert(previously_spoken.get(&spoken), i));
        last_spoken = spoken;
    }

    return last_spoken.to_string();
}

impl crate::utils::Solver for Solver {
    fn part1(&self, input: &str) -> String {
        solve(input, 2020)
    }
    fn part2(&self, input: &str) -> String {
        solve(input, 30000000)
    }
}