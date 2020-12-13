use crate::utils::{Grid, Lookup};
use std::str::FromStr;

pub(crate) struct Solver;

type Coord = (isize, isize);
type Coords = Vec<Coord>;



fn get_surround_coords( (x, y) : Coord)-> Coords {
    vec![(x-1, y-1), (x, y-1), (x+1, y-1),
         (x-1, y),  /* middle*/ (x+1, y),
         (x-1, y+1), (x, y+1),  (x+1, y+1)]
}

fn lookup_coords(map: &Grid, coords: Coords) -> Vec<char> {
    coords.iter().map( |coord| map.lookup_coord(*coord).unwrap_or(&'.').clone()).collect()
}

fn number_occupied(surround: Vec<char>) -> u32 {
    surround.iter().filter(|&&x| x=='#').count() as u32
}

fn new_state(map: &Grid, &now: &Coord) -> char {
    let &state = map.lookup_coord(now).unwrap();
    if state == '.' {
        '.'
    } else {
        let surround = number_occupied(lookup_coords(map, get_surround_coords(now)));
        match (state == 'L' && surround == 0) || (state == '#' && surround < 4) {
            true => '#',
            false => 'L'
        }
    }
}

impl crate::utils::Solver for Solver {
    fn part1(&self, input: &str) -> String {
        let mut grid = Grid::from_str(input).unwrap();
        let mut last_grid = Grid::from_str("").unwrap();
        let mut iterations = 0;
        while grid != last_grid {
            iterations += 1;
            last_grid = grid.clone();
            grid = grid.into_iter().map(|(xy,_)|(xy, new_state(&last_grid, &xy))).collect();
        }
        println!("iterations: {}", iterations);
        grid.into_iter().filter(|(_, c)| c == &'#').count().to_string()
    }
    fn part2(&self, input: &str) -> String {
        return "Result".parse().unwrap();
    }
}