use itertools::Itertools;
use std::str::Chars;
use stats::Frequencies;

pub(crate) struct Solver;

#[derive(Debug)]
struct Tile {
    id: u64,
    borders: [u64; 8]
}

fn border_to_number(border_chars: &str) -> u64 {
    let binary_string = border_chars
        .replace(".", "0")
        .replace("#", "1");
    u64::from_str_radix(&binary_string, 2).unwrap()
}

fn parse_tile(tile_str: &str) -> Tile {
    let lines: Vec<&str> = tile_str.lines().collect();
    let tile_id = &lines[0][5..9];
    let id: u64 = tile_id.parse().unwrap();
    let borders: [u64; 8] = [
        border_to_number(lines.iter().skip(1).map(|l|l.chars().nth(9).unwrap()).collect::<String>().as_str()),
        border_to_number(lines[1]),
        border_to_number(lines.iter().skip(1).map(|l|l.chars().nth(0).unwrap()).collect::<String>().as_str()),
        border_to_number(lines[10]),
        border_to_number(lines.iter().skip(1).rev().map(|l|l.chars().nth(9).unwrap()).collect::<String>().as_str()),
        border_to_number(lines[1].chars().rev().collect::<String>().as_str()),
        border_to_number(lines.iter().skip(1).rev().map(|l|l.chars().nth(0).unwrap()).collect::<String>().as_str()),
        border_to_number(lines[10].chars().rev().collect::<String>().as_str())
    ];
    Tile {
        id,
        borders
    }
}

fn is_corner(freqs: &Frequencies<u64>, tile: &Tile) -> bool {
    tile.borders.iter().filter(|num| freqs.count(num) == 1).count() == 4
}

impl crate::utils::Solver for Solver {
    fn part1(&self, input: &str) -> String {
        let tiles: Vec<Tile> = input.split("\n\n").map(parse_tile).collect();
        let mut freq: Frequencies<u64> = Frequencies::new();
        for tile in tiles.iter() {
            for border in tile.borders.iter() {
                freq.add(*border);
            }
        }
        let corners: Vec<&Tile> = tiles.iter().filter(|tile| is_corner(&freq, tile)).collect();
        return corners.iter().fold(1, |prod, tile| prod * tile.id).to_string();
    }
    fn part2(&self, input: &str) -> String {
        return "Result".parse().unwrap();
    }
}