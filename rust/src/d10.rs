use std::collections::HashMap;

pub(crate) struct Solver;

fn read_input(input: &str) -> Vec<i64> {
    let mut numbers: Vec<i64> = input.lines().map(|s| s.parse().unwrap()).collect();
    // outlet jolt
    numbers.push(0);
    // device jolt
    numbers.push(numbers.iter().max().unwrap() + 3);
    numbers.sort();
    return numbers;
}

impl crate::utils::Solver for Solver {
    fn part1(&self, input: &str) -> String {
        let numbers = read_input(input);
        let mut one_jolts = 0;
        let mut three_jolts = 0;
        for i in 1..numbers.len() {
            match numbers[i] - numbers[i-1] {
                1 => {one_jolts += 1}
                3 => {three_jolts += 1}
                _ => {}
            }
        };
        return (one_jolts * three_jolts).to_string()
    }
    fn part2(&self, input: &str) -> String {
        let numbers = read_input(input);
        let mut known_combinations: HashMap<i64, i64> = HashMap::new();
        known_combinations.insert(0,1);
        for i in numbers.iter().skip(1) {
            let combinations =
                known_combinations.get(&(i-1)).unwrap_or(&0) +
                known_combinations.get(&(i-2)).unwrap_or(&0) +
                known_combinations.get(&(i-3)).unwrap_or(&0);
            known_combinations.insert(i.clone(), combinations);
        }
        return known_combinations.get(numbers.last().unwrap()).unwrap().to_string();
    }
}