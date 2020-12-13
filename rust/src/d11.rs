use crate::utils::{Grid, Lookup};
use std::str::FromStr;
use std::collections::HashMap;

pub(crate) struct Solver;

type Coord = (isize, isize);
type Coords = Vec<Coord>;


fn get_closest_chair(grid: &Grid, (x0, y0): Coord, (x, y): (isize, isize)) -> Option<Coord> {
    let mut current = (x0+x, y0+y);
    while (grid.min_width..grid.max_width).contains(&current.0) &&
        (grid.min_height..grid.max_height).contains(&current.1)  {
        if grid.lookup_coord(current).unwrap() == &'L' { return Some(current) }
        current.0 += x;
        current.1 += y;
    }
    None
}

fn get_closest_chairs(grid: &Grid, coord: Coord) -> Coords {
    let directions:Vec<(isize,isize)> = vec![ (-1, -1), (0, -1), (1, -1),
                          (-1, 0), /* middle*/ (1, 0),
                          (-1, 1), (0, 1), (1, 1)];
    let mut res = vec![];
    for dir in directions {
        match get_closest_chair(grid, coord, dir) {
            None => (),
            Some(c) => res.push(c)
        }
    }
    res
}

fn make_closest_dir(grid: &Grid) -> HashMap<Coord, Coords> {
    let coords = grid.clone();
    coords.into_iter().map(|(xy, _c)| (xy,get_closest_chairs(&grid, xy))).collect()
}

fn make_surounding_dir(grid: &Grid) -> HashMap<Coord, Coords> {
    let mut result = HashMap::new();
    for x in grid.min_width..grid.max_width {
        for y in grid.min_height..grid.max_height {
            result.insert((x,y), vec![(x-1, y-1), (x, y-1), (x+1, y-1),
                 (x-1, y),  /* middle*/ (x+1, y),
                 (x-1, y+1), (x, y+1),  (x+1, y+1)]);
        }
    }
    result
}

fn get_surround_coords(closest_chairs: &HashMap<Coord, Coords>, (x, y) : Coord)-> &Coords {
    closest_chairs.get(&(x, y)).unwrap()
}

fn lookup_coords(map: &Grid, coords: &Coords) -> Vec<char> {
    coords.iter().map( |coord| map.lookup_coord(*coord).unwrap_or(&'.').clone()).collect()
}

fn number_occupied(surround: Vec<char>) -> u32 {
    surround.iter().filter(|&&x| x=='#').count() as u32
}

fn new_state(closest_chairs: &HashMap<Coord, Coords>, map: &Grid, &now: &Coord, crowded_number: u32) -> char {
    let &state = map.lookup_coord(now).unwrap();
    if state == '.' {
        '.'
    } else {
        let surround = number_occupied(lookup_coords(map, get_surround_coords(closest_chairs, now)));
        match (state == 'L' && surround == 0) || (state == '#' && surround < crowded_number) {
            true => '#',
            false => 'L'
        }
    }
}

impl crate::utils::Solver for Solver {
    fn part1(&self, input: &str) -> String {
        let mut grid = Grid::from_str(input).unwrap();
        let mut last_grid = Grid::from_str("").unwrap();
        let surounding_dir = make_surounding_dir(&grid);
        let mut iterations = 0;
        while grid != last_grid {
            iterations += 1;
            last_grid = grid.clone();
            grid = grid.into_iter().map(|(xy,_)|(xy, new_state(&surounding_dir,&last_grid, &xy, 4))).collect();
        }
        println!("iterations: {}", iterations);
        grid.into_iter().filter(|(_, c)| c == &'#').count().to_string()
    }
    fn part2(&self, input: &str) -> String {
        let mut grid = Grid::from_str(input).unwrap();
        let mut last_grid = Grid::from_str("").unwrap();
        let surounding_dir = make_closest_dir(&grid);
        let mut iterations = 0;
        while grid != last_grid {
            iterations += 1;
            last_grid = grid.clone();
            grid = grid.into_iter().map(|(xy,_)|(xy, new_state(&surounding_dir,&last_grid, &xy, 5))).collect();
        }
        println!("iterations: {}", iterations);
        grid.into_iter().filter(|(_, c)| c == &'#').count().to_string()
    }
}