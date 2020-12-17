use std::collections::HashSet;

pub(crate) struct Solver;

type Point = (isize, isize, isize, isize);

fn read_grid(s: &str) -> HashSet<Point> {
    s.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().filter_map(move |(x, char)| if char=='#'{ Some((x as isize, y as isize, 0 as isize, 0 as isize))} else { None })
    }).flatten().collect()
}

fn step(currently_active: &mut HashSet<Point>, is_four_d: bool) {
    let mut one_neighbor = HashSet::new();
    let mut two_neighbors = HashSet::new();
    let mut three_neighbors = HashSet::new();
    let mut more_neighbors = HashSet::new();
    for (x, y, z, w) in currently_active.iter() {
        for ix in -1..=1 {
            for iy in -1..=1 {
                for iz in -1..=1 {
                    for iw in if is_four_d {-1..=1} else {0..=0} {
                        if (ix, iy, iz, iw) != (0, 0, 0, 0) {
                            let p = (x + ix, y + iy, z + iz, w + iw);
                            if one_neighbor.contains(&p) {
                                two_neighbors.insert(p);
                                one_neighbor.remove(&p);
                            } else if two_neighbors.contains(&p) {
                                three_neighbors.insert(p);
                                two_neighbors.remove(&p);
                            } else if three_neighbors.contains(&p) {
                                more_neighbors.insert(p);
                                three_neighbors.remove(&p);
                            } else if !more_neighbors.contains(&p) {
                                one_neighbor.insert(p);
                            }
                        }
                    }
                }
            }
        }
    }
    currently_active.retain(|cube| two_neighbors.contains(&cube));
    currently_active.extend(&three_neighbors);
}


impl crate::utils::Solver for Solver {
    fn part1(&self, input: &str) -> String {
        let mut active = read_grid(input);
        for _i in 0..6 {
            step(&mut active, false);
        }
        return active.len().to_string();
    }
    fn part2(&self, input: &str) -> String {
        let mut active = read_grid(input);
        for _i in 0..6 {
            step(&mut active, true);
        }
        return active.len().to_string();
    }
}