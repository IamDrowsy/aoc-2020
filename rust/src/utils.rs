pub(crate) struct Solution {
    pub(crate) part1_solution : String,
    pub(crate) part2_solution : String
}

pub(crate) trait Solver {
    fn part1(&self, input: &str) -> &str;
    fn part2(&self, input: &str) -> &str;
    fn solve(&self, input: &str) -> Solution {
        return Solution {
            part1_solution: self.part1(input).to_string(),
            part2_solution: self.part2(input).to_string()};
    }
}