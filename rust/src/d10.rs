pub(crate) struct Solver;

fn read_input(input: &str) -> Vec<u64> {
    let mut numbers: Vec<u64> = input.lines().map(|s| s.parse().unwrap()).collect();
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
        let mut combinations: u64 = 1;
        for i in 0..numbers.len()-3 {
            if numbers[i+3] == numbers[i] + 3 {
                combinations *= 3;
            } else if numbers[i+2] == numbers[i] + 2 || numbers[i+2] == numbers[i] + 3 {
                combinations *= 2;
            } else {
                // combinations stays the same
            }

        }
        return combinations.to_string();
    }
}