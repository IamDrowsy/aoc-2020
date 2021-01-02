use regex::Regex;

pub(crate) struct Solver;

#[derive(Debug)]
struct Timetable {
    timestamp: u32,
    bus_ids: Vec<u32>
}

fn parse_part1(input: &str) -> Timetable {
    let re = Regex::new(r"\d+").unwrap();
    let mut lines = input.lines();
    Timetable {
        timestamp: lines.next().unwrap().parse().unwrap(),
        bus_ids: re.find_iter(lines.next().unwrap()).map(|s| s.as_str().parse().unwrap()).collect()
    }
}

impl crate::utils::Solver for Solver {
    fn part1(&self, input: &str) -> String {
        let t = parse_part1(input);
        let result = t.bus_ids.iter()
            .map(|id| (id-t.timestamp%id, id))
            .min_by(|a,b| a.0.cmp(&b.0))
            .unwrap();
        return (result.0 * result.1).to_string()
    }
    fn part2(&self, input: &str) -> String {
        let mut numbers_with_offset : Vec<(u64, u64)> = vec![];
        let mut offset = 0;
        for entry in input.lines().nth(1).unwrap().split(",") {
            match entry.parse::<u64>() {
                Ok(value) => {numbers_with_offset.push((value, offset.clone()))}
                Err(_) => {}
            }
            offset += 1;
        }
        let mut current_step_size = 1;
        let mut current_number = 1;
        for (number, offset) in numbers_with_offset {
            while (current_number + offset)  % number != 0 {
                current_number += current_step_size;
            }
            current_step_size *= number;
        }
        return current_number.to_string();
    }
}