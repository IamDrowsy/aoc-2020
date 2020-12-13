use std::collections::HashMap;
use std::collections::hash_map;
use std::str::FromStr;
use std::string::ParseError;
use std::fmt::{Display, Formatter};
use std::fmt;
use itertools::__std_iter::FromIterator;

pub(crate) struct Solution {
    pub(crate) part1_solution : String,
    pub(crate) part2_solution : String
}

pub(crate) trait Solver {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
    fn solve(&self, input: &str) -> Solution {
        return Solution {
            part1_solution: self.part1(input).to_string(),
            part2_solution: self.part2(input).to_string()};
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct Grid {
    min_width: isize,
    max_width: isize,
    min_height: isize,
    max_height: isize,
    grid: HashMap<(isize, isize), char>
}

pub(crate) trait Lookup {
    fn lookup_coord(&self, coord : (isize,isize) ) -> Option<&char>;
    fn lookup(&self, x: isize, y: isize) -> Option<&char> {
        self.lookup_coord((x, y))
    }
}

impl FromStr for Grid {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tmp_grid: HashMap<(isize, isize), char> = HashMap::new();
        let tmp_width: isize = s.lines().next().unwrap_or("").len() as isize;
        let tmp_height: isize = s.lines().count() as isize;
        let tmp_char_vec = s.chars().collect::<Vec<_>>();
        for x in 0..tmp_width {
            for y in 0..tmp_height {
                tmp_grid.insert((x , y), tmp_char_vec[(y*tmp_width + x + y) as usize]);
            }
        }
        Ok(Grid {
            min_width: 0,
            max_width: tmp_width,
            min_height: 0,
            max_height: tmp_height,
            grid: tmp_grid})
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grid: width=[{}..{}], height=[{}..{}]", self.min_width, self.max_width, self.min_height, self.max_height)?;
        for y in self.min_height..self.max_height {
            for x in self.min_width..self.max_width {
                write!(f, "{}" ,self.lookup(x, y).unwrap_or(&' ').to_string())?;
            }
            write!(f,"{}", "\n")?;
        }
        Ok(())
    }
}

impl Lookup for Grid {
    fn lookup_coord(&self, coord: (isize, isize)) -> Option<&char> {
        self.grid.get(&coord)
    }
}

impl FromIterator<((isize, isize), char)> for Grid {
    fn from_iter<T: IntoIterator<Item=((isize, isize), char)>>(iter: T) -> Self {
        let mut min_width = 0;
        let mut max_width = 0;
        let mut min_height = 0;
        let mut max_height = 0;
        let mut map = HashMap::new();
        for (coord, c) in iter {
            let (x, y) = coord;
            if x > max_width { max_width = x }
            if x < min_width { min_width = x }
            if y > max_height { max_height = y}
            if y < min_height { min_height = y}
            map.insert(coord, c);
        }
        Grid {
            min_width: min_width.clone(),
            max_width: max_width.clone() + 1,
            min_height: min_height.clone(),
            max_height: max_height.clone() + 1,
            grid: map
        }
    }
}

impl IntoIterator for Grid {
    type Item = ((isize, isize), char);
    type IntoIter = hash_map::IntoIter<(isize, isize), char>;

    fn into_iter(self) -> Self::IntoIter{
        self.grid.into_iter()
    }
}
