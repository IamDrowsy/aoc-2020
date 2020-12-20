pub(crate) struct Solver;

fn as_number(c: char) -> u64 {
    c.to_digit(10).unwrap() as u64
}

fn step(expression: &Vec<char>, i: usize, result: u64) -> (usize, u64) {
    if i == expression.len() { return (i, result) }
        let c = expression[i];
        match c {
            '(' => {
                let (next, res) = step(expression, i + 1, 0);
                if result == 0 {
                    step(expression, next, res)
                } else {
                    (next, res)
                }
            },
            ')' => {
                    (i + 1, result)
            },
            '+' => {let (next, res) = step(expression, i+1, 1);
                step(expression, next, result+res)},
            '*' => {let (next, res) = step(expression, i+1, 1);
                step(expression, next, result*res)},
            _  => if result==0 {
                step(expression, i+1, as_number(c))
            } else {
                (i+1, as_number(c))
            }
        }
    }

fn step2(expression: &Vec<char>, i: usize, result: u64, prec: u64) -> (usize, u64, u64) {
    if i == expression.len() { return (i, result, prec); }
    let c = expression[i];
    //println!("i:{}, c:{}, result:{}, prec:{}", i, c, result, prec);
    match c {
        '(' => {
            let mut next: (usize, u64, u64);
            next = step2(expression, i + 1, 0, prec+1);
            while next.2 > prec {
                next = step2(expression, next.0, next.1, next.2);
            }
            if result == 0 {
                step2(expression, next.0, next.1, next.2)
            } else {
                next
            }
        }
        ')' => {
            (i + 1, result, prec-1)
        }
        '+' => {
            let (next, res,pre) = step2(expression, i + 1, 1, prec);
            (next, result + res, pre)
        }
        '*' => {
            let mut next: (usize, u64, u64);
            next = step2(expression, i + 1, 0, prec+1);
            while next.2 > prec {
                next = step2(expression, next.0, next.1, next.2);
            }
            (next.0, result * next.1, next.2-1)
        }
        _ => if result == 0 {
            step2(expression, i + 1, as_number(c), prec)
        } else {
            (i + 1, as_number(c), prec)
        }
    }
}

fn eval(input: &str) -> u64 {
    let chars: Vec<char> = input.replace(" ", "").chars().collect();
    let (_, result) = step(&chars, 0, 0);
    return result
}

fn eval2(input: &str) -> u64 {
    let chars: Vec<char> = format!("({})",input).replace(" ", "").chars().collect();
    let (_, result, _) = step2(&chars, 0, 0, 0);
    return result
}

impl crate::utils::Solver for Solver {
    fn part1(&self, input: &str) -> String {
        return input.lines().map(eval).sum::<u64>().to_string();
    }
    fn part2(&self, input: &str) -> String {
        return input.lines().map(eval2).sum::<u64>().to_string();
    }
}