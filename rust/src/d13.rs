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
        return "Result".parse().unwrap();
    }
}