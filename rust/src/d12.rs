use std::str::FromStr;
use std::string::ParseError;
use num::complex::Complex;

pub(crate) struct Solver;

#[derive(Debug)]
enum Action {
    Move(Complex<i32>), Rotate(Complex<i32>), MoveForward(i32)
}

impl FromStr for Action {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action_type = &s[0..1];
        let action_amount: i32 = s[1..].to_string().parse().unwrap();
        let action = match action_type {
            "N" => Action::Move(Complex::new(0, action_amount)),
            "S" => Action::Move(Complex::new(0, -action_amount)),
            "E" => Action::Move(Complex::new(action_amount, 0)),
            "W" => Action::Move(Complex::new(-action_amount, 0)),
            "L" => Action::Rotate(
                match action_amount {
                    90 => Complex::new(0, 1),
                    180 => Complex::new(-1, 0),
                    270 => Complex::new(0, -1),
                    _ => panic!("Invalid rotation {}", action_amount)
                }),
            "R" => Action::Rotate(
                match action_amount {
                    90 => Complex::new(0, -1),
                    180 => Complex::new(-1, 0),
                    270 => Complex::new(0, 1),
                    _ => panic!("Invalid rotation {}", action_amount)
                }),
            "F" => Action::MoveForward(action_amount),
            _ => panic!("Invalid action")
        };
        Ok(action)
    }
}

fn parse_input(input: &str) -> Vec<Action> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[derive(Debug)]
struct Ship {
    position: Complex<i32>,
    direction: Complex<i32>
}

impl Ship {
    fn do_action(&mut self, action: &Action) {
        match action {
            Action::Move(a) => self.position+=a,
            Action::Rotate(r) => self.direction *= r ,
            Action::MoveForward(a) => self.position += self.direction * a
        };
    }
}

struct System {
    ship_position: Complex<i32>,
    waypoint_position: Complex<i32>
}

impl System {
    fn do_action(&mut self, action: &Action) {
        match action {
            Action::Move(a) => self.waypoint_position += a,
            Action::Rotate(r) => self.waypoint_position *= r,
            Action::MoveForward(a) => self.ship_position += self.waypoint_position*a
        };
    }
}

impl crate::utils::Solver for Solver {
    fn part1(&self, input: &str) -> String {
        let actions = parse_input(input);
        let mut ship = Ship {
            position: Complex::new(0,0),
            direction: Complex::new(1,0)
        };
        actions.iter().map(|a| ship.do_action(a)).count();
        (num::abs(ship.position.re) + num::abs(ship.position.im)).to_string()
    }
    fn part2(&self, input: &str) -> String {
        let actions = parse_input(input);
        let mut system = System {
            ship_position: Complex::new(0,0),
            waypoint_position: Complex::new(10,1)
        };
        actions.iter().map(|a| system.do_action(a)).count();
        (num::abs(system.ship_position.re) + num::abs(system.ship_position.im)).to_string()
    }
}