use std::collections::HashMap;
use std::str::FromStr;
use std::string::ParseError;

pub(crate) struct Solver;

#[derive(Debug)]
struct InfiniteGrid {
    height: usize,
    base_width: usize,
    base_grid: HashMap<(usize, usize), char>
}

trait Lookup {
    fn lookup(&self, x: usize, y: usize) -> char;
}

impl Lookup for InfiniteGrid {
    fn lookup(&self, x: usize, y: usize) -> char {
        **&self.base_grid.get(&(x % &self.base_width, y)).unwrap()
    }
}

trait Slopable {
    fn check_slope(&self, v_x: usize, v_y: usize) -> u64;
}

impl Slopable for InfiniteGrid {
    fn check_slope(&self, v_x: usize, v_y: usize) -> u64 {
        let mut x = 0; let mut y = 0;
        let mut count = 0;
        while &y < &self.height {
            if &self.lookup(x,y) == &'#' {
                count += 1;
            }
            x += v_x; y += v_y;
        }
        return count;
    }
}

impl FromStr for InfiniteGrid {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tmp_base_grid: HashMap<(usize, usize), char> = HashMap::new();
        let tmp_base_width: usize = s.lines().next().unwrap().len();
        let tmp_height: usize = s.lines().count();
        let tmp_char_vec = s.chars().collect::<Vec<_>>();
        for x in 0..tmp_base_width {
            for y in 0..tmp_height {
                tmp_base_grid.insert((x, y), tmp_char_vec[y*tmp_base_width + x + y]);
            }
        }
        Ok(InfiniteGrid {
            height: tmp_height,
            base_width: tmp_base_width,
            base_grid: tmp_base_grid})
    }
}

impl crate::utils::Solver for Solver {
    fn part1(&self, input: &str) -> String {
        let grid = InfiniteGrid::from_str(input).unwrap();
        grid.check_slope(3,1).to_string()

    }
    fn part2(&self, input: &str) -> String {
        let grid = InfiniteGrid::from_str(input).unwrap();
        (grid.check_slope(1,1) *
            grid.check_slope(3,1) *
            grid.check_slope(5,1) *
            grid.check_slope(7,1) *
            grid.check_slope(1,2)).to_string()
    }
}