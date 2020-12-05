pub(crate) struct Solver;

fn line_to_seat_id(line: &str) -> usize {
    let binary = line
        .replace("F", "0")
        .replace("B", "1")
        .replace("L", "0")
        .replace("R", "1");
    usize::from_str_radix(&binary, 2).unwrap()
}

impl crate::utils::Solver for Solver {
    fn part1(&self, input: &str) -> String {
        let mut seats = input.lines()
            .map(line_to_seat_id)
            .collect::<Vec<usize>>();
        seats.sort();
        seats[seats.len()-1].to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut seats = input.lines()
            .map(line_to_seat_id)
            .collect::<Vec<usize>>();
        seats.sort();
        let mut last: usize = seats[0] - 1;
        for id in seats {
            if id - last > 1 {
                return (last + 1).to_string();
            }
            last = id;
        }
        return "Not found".parse().unwrap();
    }
}