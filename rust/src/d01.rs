pub(crate) struct Solver;

fn parse_int(input: &str) -> i32 {
    input.parse::<i32>().unwrap()
}

fn parse_input(input: &str) ->  Vec<i32> {
    return input.lines().map(parse_int).collect();
}

fn solve_part_1(mut vec: Vec<i32>, target: i32) -> Option<i32> {
    vec.sort();
    let mut lower = 0;
    let mut upper = vec.len() - 1;
    let result = loop {
        let val = vec[lower] + vec[upper];
        if vec[lower] == target || vec[upper] == target {
          // when we hit the target we just continue as this is not valid
        } else if val == target {
            break Some(vec[lower] * vec[upper]);
        } else if lower == upper {
            break None;
        } else if val > target {
            upper -= 1;
        } else {
            lower += 1;
        }
    };
    return result;
}

impl crate::utils::Solver for Solver {

    fn part1(&self, input: &str) -> String {
        let vec = parse_input(input);
        let result = solve_part_1(vec, 2020);
        return result.unwrap().to_string();
    }
    fn part2(&self, input: &str) -> String {
        let vec = parse_input(input);
        let mut result = 0;
        for elem in vec.clone() {
            let inner_result = solve_part_1(vec.clone(), 2020 - elem);
            if (inner_result.is_some()) {
                result = inner_result.unwrap() * elem;
                break;
            }
        };
        return result.to_string();
    }
}