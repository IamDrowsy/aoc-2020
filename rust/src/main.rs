use crate::utils::{Solver};

mod utils;
mod d01;
mod d02;
mod d03;

#[macro_use]
extern crate lazy_static;


fn main() {

    let day = std::env::args().nth(1).expect("no day given");
    let data_name = std::env::args().nth(2).unwrap_or("drowsy".to_owned());
    let file = format!("../data/d{:02}-input-{}.txt", day.parse::<i32>().unwrap(), data_name);
    println!("Running day {} on input {}", day, file);
    let input = std::fs::read_to_string(&file).unwrap();
    let part01_solution_file = format!("../data/d{:02}-part1-{}.txt", day.parse::<i32>().unwrap(), data_name);
    let part02_solution_file = format!("../data/d{:02}-part2-{}.txt", day.parse::<i32>().unwrap(), data_name);

    let s = match day.as_str() {
        "1" => d01::Solver.solve(&input),
        "2" => d02::Solver.solve(&input),
        "3" => d03::Solver.solve(&input),
        _ => panic!("invalid day")
    };
    println!("Solution for Day {}", day);
    println!("Part 1: {}", s.part1_solution);
    println!("Part 2: {}", s.part2_solution);
    std::fs::write(part01_solution_file, s.part1_solution).expect("Could not write part 1");
    std::fs::write(part02_solution_file, s.part2_solution).expect("Could not write part 2");
}
